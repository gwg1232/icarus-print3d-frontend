use sqlx::{
    postgres::{PgConnectOptions, PgPool, PgPoolOptions},
    ConnectOptions,
};
use std::{str::FromStr, time::Duration};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub fn init_logging() {
    let filter = EnvFilter::builder()
        .with_default_directive(tracing::Level::INFO.into())
        .from_env_lossy();

    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set up logging");
}

pub async fn init_database(database_url: &str) -> PgPool {
    tracing::debug!("Setting up database connection");

    let options = PgConnectOptions::from_str(database_url)
        .expect("Failed to parse database URL")
        .disable_statement_logging();

    let db = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(options)
        .await
        .expect("Failed to connect to the database");

    tracing::debug!("Successfully connected to database");

    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Failed to run migrations");

    tracing::debug!("Successfully migrated database");

    db
}
