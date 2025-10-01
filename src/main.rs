use maud_tailwind_htmx_axum_sqlx_postgres::{
    config::{AppConfig, AppState},
    init, routes,
};

#[tokio::main]
async fn main() {
    init::init_logging();

    let config = AppConfig::from_env();
    let db = init::init_database(&config.database_url).await;
    let state = AppState { db };

    tracing::info!("Server is starting...");
    tracing::info!("Listening at {}", config.server_addr);

    let listener = tokio::net::TcpListener::bind(&config.server_addr)
        .await
        .unwrap();

    let app = routes::create_routes(state);

    axum::serve(listener, app).await.unwrap();
}
