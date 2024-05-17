use axum::response::IntoResponse;
use sqlx::{Executor, SqlitePool};
use thiserror::Error;

pub mod handlers;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] sqlx::Error),

    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Axum error: {0}")]
    Axum(#[from] axum::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response<axum::body::Body> {
        log::error!("Error: {:?}", self);
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error",
        )
            .into_response()
    }
}

pub type Database = SqlitePool;

const SERVICE_NAME: &str = std::env!("CARGO_PKG_NAME");

pub async fn init_db() -> Result<Database> {
    let conn_str = std::env::var(format!("{SERVICE_NAME}_DATABASE_URI"))
        .unwrap_or_else(|_| String::from(":memory:"));

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
