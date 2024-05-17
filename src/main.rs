#[tokio::main]
async fn main() {
    antiquarian::tracing::init_tracing();

    let addr: std::net::SocketAddr = std::env::var("ANTIQUARIAN_LISTEN_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8000".to_owned())
        .parse()
        .expect("Invalid address format");

    let database = antiquarian::database::sqlite::init_db()
        .await
        .expect("to initialize database");

    let router = antiquarian::build_router(database);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("to bind listener to address");

    tracing::info!("Starting server at: {}", addr);
    axum::serve(listener, router)
        .await
        .expect("to bind server to address");
}
