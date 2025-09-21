use maud_tailwind_htmx_axum_sqlx_postgres::{init, routes};

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8000";

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    init::logging();

    tracing::info!("Server is starting...");

    tracing::info!("Listening at {}", addr);

    let app = routes::create_routes();

    axum::serve(listener, app).await.unwrap();
}
