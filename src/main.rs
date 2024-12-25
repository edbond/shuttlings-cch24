use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};

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
        .fallback(fallback);

    Ok(router.into())
}
