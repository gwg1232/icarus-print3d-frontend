use crate::models::todo::{CreateTodo, UpdateTodo};
use crate::views;
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Form,
    http::StatusCode,
};
use maud::Markup;
use sqlx::PgPool;

pub async fn todos_page() -> Markup {
    views::pages::todos::todos()
}

pub async fn list(
    State(_pool): State<PgPool>,
) -> Response {
    StatusCode::NOT_IMPLEMENTED.into_response()
}

pub async fn create(
    State(_pool): State<PgPool>,
    Form(_input): Form<CreateTodo>,
) -> Response {
    StatusCode::NOT_IMPLEMENTED.into_response()
}

pub async fn get(
    State(_pool): State<PgPool>,
    Path(_id): Path<i32>,
) -> Response {
    StatusCode::NOT_IMPLEMENTED.into_response()
}

pub async fn update(
    State(_pool): State<PgPool>,
    Path(_id): Path<i32>,
    Form(_input): Form<UpdateTodo>,
) -> Response {
    StatusCode::NOT_IMPLEMENTED.into_response()
}

pub async fn delete(
    State(_pool): State<PgPool>,
    Path(_id): Path<i32>,
) -> Response {
    StatusCode::NOT_IMPLEMENTED.into_response()
}