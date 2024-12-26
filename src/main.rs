use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use day2::*;

async fn hello_world() -> &'static str {
    "Hello, bird!"
}

async fn fallback() -> impl IntoResponse {
    (
        StatusCode::FOUND,
        [(
            header::LOCATION,
            "https://www.youtube.com/watch?v=9Gc4QTqslN4",
        )],
        "not found",
    )
}

mod day2;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/2/dest", get(task1))
        .route("/2/key", get(task2))
        .route("/2/v6/dest", get(task1_v6))
        .route("/2/v6/key", get(task2_v6))
        .fallback(fallback);

    Ok(router.into())
}
