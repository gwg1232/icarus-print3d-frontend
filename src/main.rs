mod views;

use axum::{Router, routing::get};
use maud::Markup;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    let app = Router::new()
        .route("/", get(home))
        .route("/about", get(about));

    axum::serve(listener, app).await.unwrap();
}

async fn home() -> Markup {
    views::pages::home::home()
}

async fn about() -> Markup {
    views::pages::about::about()
}
