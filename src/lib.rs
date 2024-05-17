use axum::{response::IntoResponse, routing::any, Router};
use database::Database;
use thiserror::Error;

pub mod database;
pub mod handlers;
pub mod tracing;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] database::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response<axum::body::Body> {
        let uuid = uuid::Uuid::new_v4();
        tracing::error!("{uuid:?} error: {self:?}");

        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal Server Error: {uuid:?}"),
        )
            .into_response()
    }
}

pub fn build_router<D: Database>(database: D) -> Router {
    Router::new()
        //TODO: Handle base path
        .route("/*path", any(handlers::collector::<D>))
        .with_state(database)
}
