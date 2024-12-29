use std::sync::{Arc, Mutex};

use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};

mod day2;
mod day5;
mod day9;

use day2::*;
use day5::*;
use day9::*;

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

#[derive(Debug)]
struct AppState {
    bucket: u32,
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let shared_state: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState { bucket: 5 }));

    // run function in background using tokio
    tokio::spawn({
        let shared_state = shared_state.clone();
        async move {
            loop {
                {
                    let mut state = shared_state.lock().unwrap();
                    println!("tick: {state:?}");
                    if state.bucket < 5 {
                        state.bucket += 1;
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await
            }
        }
    });

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/2/dest", get(task1))
        .route("/2/key", get(task2))
        .route("/2/v6/dest", get(task1_v6))
        .route("/2/v6/key", get(task2_v6))
        .route("/5/manifest", post(day5_manifest))
        .route("/9/milk", post(milk))
        .route("/9/refill", post(refill))
        .with_state(shared_state.clone())
        .fallback(fallback);

    Ok(router.into())
}
