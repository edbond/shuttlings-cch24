use crate::{AppState, Item};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse};
use std::sync::Mutex;
use std::sync::{Arc, MutexGuard};

pub async fn board(state: State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    let state = state.lock().unwrap();
    (StatusCode::OK, game_board(&state))
}

pub async fn reset(state: State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    let mut state = state.lock().unwrap();
    state.game.reset();
    format!("{}", state.game)
}

pub async fn place(
    state: State<Arc<Mutex<AppState>>>,
    Path((team, column)): Path<(String, u32)>,
) -> impl IntoResponse {
    println!("move: {team} {column}");

    if team != "milk" && team != "cookie" {
        return (StatusCode::BAD_REQUEST, "invalid team".to_string());
    }

    if !(1..=4).contains(&column) {
        return (StatusCode::BAD_REQUEST, "invalid column".to_string());
    }

    let mut state = state.lock().unwrap();

    if state.game.is_winner().is_some() {
        return (StatusCode::SERVICE_UNAVAILABLE, game_board(&state));
    }

    if state.game.place(&team, column).is_err() {
        let response = game_board(&state);
        return (StatusCode::SERVICE_UNAVAILABLE, response);
    }

    let response = game_board(&state);
    (StatusCode::OK, response)
}

pub async fn random_board(state: State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    let mut state = state.lock().unwrap();
    state.game.make_rand();
    format!("{}", state.game)
}

fn game_board(state: &MutexGuard<'_, AppState>) -> String {
    let mut response = state.game.to_string();

    if let Some(winner) = state.game.is_winner() {
        let winner = match winner {
            Item::Milk => "🥛",
            Item::Cookie => "🍪",
            _ => "draw",
        };
        response.push_str(format!("{winner} wins!\n").as_str());
    } else if state.game.is_full() {
        response.push_str("No winner.\n");
    }
    response
}
