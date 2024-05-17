use axum::{body::Body, extract::Request};
use sqlx::Executor;

use super::{Database, SerializableHeaderMap};

impl Database for sqlx::SqlitePool {
    async fn insert_request(&self, request: Request<Body>) -> super::Result<()> {
        let (parts, body) = request.into_parts();

        let body_bytes = axum::body::to_bytes(body, usize::MAX).await?;
        let headers_serialised = serde_json::to_string(&SerializableHeaderMap(&parts.headers))?;

        let query = sqlx::query::<sqlx::Sqlite>(
            "INSERT INTO requests (method, headers, path, body) VALUES (?, ?, ?, ?)",
        )
        .bind(parts.method.to_string())
        .bind(parts.uri.to_string())
        .bind(headers_serialised)
        .bind(body_bytes.to_vec());

        self.execute(query).await?;
        Ok(())
    }
}

pub async fn init_db() -> super::Result<sqlx::SqlitePool> {
    let conn_str =
        std::env::var("ANTIQUARIAN_DATABASE_URI").expect("ANTIQUARIAN_DATABASE_URI to be set");

    let options = sqlx::sqlite::SqliteConnectOptions::new()
        .filename(conn_str)
        .create_if_missing(true);

    let conn = sqlx::SqlitePool::connect_with(options).await?;

    conn.execute(
        "CREATE TABLE requests (
            id INTEGER PRIMARY KEY,
            method TEXT NOT NULL,
            headers TEXT NOT NULL,
            path TEXT NOT NULL,
            body BLOB 
        )",
    )
    .await?;

    Ok(conn)
}
