use axum::extract::FromRef;
use sqlx::PgPool;

pub struct AppConfig {
    pub server_addr: String,
    pub database_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let database_url = dotenvy::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        let server_addr = dotenvy::var("SERVER_ADDR")
            .unwrap_or_else(|_| "127.0.0.1:8000".to_string());

        Self {
            server_addr,
            database_url,
        }
    }
}

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: PgPool,
}
