use std::collections::HashMap;

use axum::{
    body::Body,
    extract::{Request, State},
    http::HeaderMap,
    response::IntoResponse,
};
use serde::{Serialize, Serializer};
use sqlx::Executor;

use crate::Database;

pub async fn collector(
    State(database): State<Database>,
    request: Request<Body>,
) -> crate::Result<impl IntoResponse> {
    let (parts, body) = request.into_parts();

    let body_bytes = axum::body::to_bytes(body, usize::MAX).await?;
    let headers_serialised = serde_json::to_string(&SerializableHeaderMap(&parts.headers))?;

    let query = sqlx::query::<sqlx::Sqlite>(
        "INSERT INTO requests (method, headers,  path, body) VALUES (?, ?, ?, ?)",
    )
    .bind(parts.method.to_string())
    .bind(parts.uri.to_string())
    .bind(headers_serialised)
    .bind(body_bytes.to_vec());

    database.execute(query).await?;

    Ok(axum::http::StatusCode::CREATED)
}

// Function to convert HeaderMap to a serializable HashMap
fn serialize_headers(headers: &HeaderMap) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    for (key, value) in headers {
        let key = key.as_str().to_string();
        let value = value.to_str().unwrap_or("").to_string();
        map.entry(key).or_insert_with(Vec::new).push(value);
    }
    map
}

struct SerializableHeaderMap<'a>(&'a HeaderMap);

impl<'a> Serialize for SerializableHeaderMap<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let map = serialize_headers(self.0);
        map.serialize(serializer)
    }
}
