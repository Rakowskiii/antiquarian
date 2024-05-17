use axum::{routing::any, Router};
use rakceptor::handlers;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let db = rakceptor::init_db().await.expect("to initialize database");

    let router = Router::new()
        .route("/*path", any(handlers::collector))
        .with_state(db);

    Ok(router.into())
}
