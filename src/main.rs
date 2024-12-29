use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};

mod day2;
mod day5;

use day2::*;
use day5::*;

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

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/2/dest", get(task1))
        .route("/2/key", get(task2))
        .route("/2/v6/dest", get(task1_v6))
        .route("/2/v6/key", get(task2_v6))
        .route("/5/manifest", post(day5_manifest))
        .fallback(fallback);

    Ok(router.into())
}
