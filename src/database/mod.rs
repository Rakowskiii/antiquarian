use std::collections::HashMap;

use axum::{body::Body, extract::Request, http::HeaderMap};
use serde::{Serialize, Serializer};

pub mod sqlite;

pub type Result<T> = std::result::Result<T, Error>;

pub trait Database: Send + Sync + Clone + 'static {
    fn insert_request(
        &self,
        request: Request<Body>,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] sqlx::Error),

    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Axum error: {0}")]
    Axum(#[from] axum::Error),
}

fn generate_hashmap(headers: &HeaderMap) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    for (key, value) in headers {
        let key = key.as_str().to_string();
        let value = value.to_str().unwrap_or("").to_string();
        map.entry(key).or_insert_with(Vec::new).push(value);
    }
    map
}

pub struct SerializableHeaderMap<'a>(&'a HeaderMap);

impl<'a> Serialize for SerializableHeaderMap<'a> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let map = generate_hashmap(self.0);
        map.serialize(serializer)
    }
}
