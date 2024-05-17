use axum::{
    body::Body,
    extract::{Request, State},
    response::IntoResponse,
};

use crate::database::Database;

pub async fn collector<D: Database>(
    State(database): State<D>,
    request: Request<Body>,
) -> crate::Result<impl IntoResponse> {
    tracing::info!("Received request: {:?}", request);
    database.insert_request(request).await?;

    Ok(axum::http::StatusCode::CREATED)
}
