use crate::AppState;
use axum::{extract::State, response::IntoResponse};
use std::sync::Arc;
use std::sync::Mutex;

pub async fn board(state: State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    format!("{}", state.lock().unwrap().game)
}

pub async fn reset(state: State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    let mut state = state.lock().unwrap();
    state.game.reset();
    format!("{}", state.game)
}
