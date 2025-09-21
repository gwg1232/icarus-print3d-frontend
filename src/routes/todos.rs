// use crate::handlers;
// use axum::{
//     Router,
//     routing::{get, post, put, delete},
// };

// pub fn routes() -> Router {
//     Router::new()
//         .route("/todos", get(handlers::todos::todos_page))
//         .route("/api/todos", get(handlers::todos::list).post(handlers::todos::create))
//         .route("/api/todos/:id",
//             get(handlers::todos::get)
//             .put(handlers::todos::update)
//             .delete(handlers::todos::delete)
//         )
// }
