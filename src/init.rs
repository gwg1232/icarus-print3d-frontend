use sqlx::{
    ConnectOptions,
    postgres::{PgConnectOptions, PgPool, PgPoolOptions},
};
use std::str::FromStr;
use tower_sessions::SessionManagerLayer;
use tower_sessions_sqlx_store::PostgresStore;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub fn init_logging() {
    let filter = EnvFilter::builder()
        .with_default_directive(tracing::Level::INFO.into())
        .from_env_lossy();

    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set up logging");
}

pub async fn init_database(database_url: &str) -> PgPool {
    let options = PgConnectOptions::from_str(database_url)
        .expect("Failed to parse database URL")
        .disable_statement_logging();

    let db = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect_with(options)
        .await
        .expect("Failed to connect to the database");

    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Failed to run migrations");

    db
}

pub async fn init_session(db: PgPool) -> SessionManagerLayer<PostgresStore> {
    let session_store = PostgresStore::new(db);

    session_store
        .migrate()
        .await
        .expect("Failed to migrate sessions");

    SessionManagerLayer::new(session_store)
        .with_expiry(tower_sessions::Expiry::OnInactivity(time::Duration::days(1)))
}
