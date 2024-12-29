use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};

mod day12;
mod day2;
mod day5;
mod day9;

use day12::*;
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
enum Item {
    Empty,
    Cookie,
    Milk,
    Wall,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Pos {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Game {
    board: HashMap<Pos, Item>,
}

impl Game {
    fn reset(&mut self) {
        self.board = HashMap::new();
        // bottom wall
        for x in 0..=5 {
            self.board.insert(Pos { x, y: 4 }, Item::Wall);
        }
        for y in 0..=4 {
            self.board.insert(Pos { x: 0, y }, Item::Wall);
            self.board.insert(Pos { x: 5, y }, Item::Wall);
        }
        for x in 1..=4 {
            for y in 0..=3 {
                self.board.insert(Pos { x, y }, Item::Empty);
            }
        }
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = String::new();
        for y in 0..=4 {
            for x in 0..=5 {
                let item = self.board.get(&Pos { x, y }).unwrap_or(&Item::Empty);
                let chr = match item {
                    Item::Empty => '⬛',
                    Item::Cookie => '🍪',
                    Item::Milk => '🥛',
                    Item::Wall => '⬜',
                };
                s.push_str(&format!("{}", chr));
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
struct AppState {
    bucket: u32,
    game: Game,
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let mut game = Game {
        board: HashMap::new(),
    };
    game.reset();

    let shared_state: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState { bucket: 5, game }));

    // run function in background using tokio
    tokio::spawn({
        let shared_state = shared_state.clone();
        async move {
            loop {
                {
                    let mut state = shared_state.lock().unwrap();
                    // println!("tick: {state:?}");
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
        .route("/12/board", get(board))
        .route("/12/reset", post(reset))
        .with_state(shared_state.clone())
        .fallback(fallback);

    Ok(router.into())
}
