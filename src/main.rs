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
use rand::{rngs::StdRng, Rng, SeedableRng};

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    rng: StdRng,
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

        self.rng = rand::rngs::StdRng::seed_from_u64(2024);
    }

    fn place(&mut self, team: &str, column: u32) -> Result<(), &str> {
        if !(1..=4).contains(&column) {
            return Err("Invalid column");
        }

        if self.board.get(&Pos { x: column, y: 0 }).unwrap() != &Item::Empty {
            return Err("Column is full");
        }

        let team = match team {
            "cookie" => Item::Cookie,
            "milk" => Item::Milk,
            _ => return Err("Invalid team"),
        };

        for y in (0..=3).rev() {
            if self.board.get(&Pos { x: column, y }).unwrap() == &Item::Empty {
                self.board.insert(Pos { x: column, y }, team);
                return Ok(());
            }
        }
        Ok(())
    }

    fn is_winner(&self) -> Option<Item> {
        // check rows
        for y in 0..=3 {
            let row = (1..=4).map(|x| self.board.get(&Pos { x, y }).unwrap());
            let row_items: Vec<&Item> = row.collect();
            let first = row_items[0];

            if *first != Item::Empty && row_items.iter().all(|item| *item == first) {
                return Some(*first);
            }
        }

        // check columns
        for x in 1..=4 {
            let column = (0..=3).map(|y| self.board.get(&Pos { x, y }).unwrap());
            let items: Vec<&Item> = column.collect();
            let first = items[0];

            if *first != Item::Empty && items.iter().all(|item| *item == first) {
                return Some(*first);
            }
        }

        // check diagonals
        let diagonal1 = (1..=4).map(|x| self.board.get(&Pos { x, y: x - 1 }).unwrap());
        let items: Vec<&Item> = diagonal1.collect();
        let first = items[0];
        if *first != Item::Empty && items.iter().all(|item| *item == first) {
            return Some(*first);
        }

        // x = 1, y = 3
        // x = 2, y = 2
        let diagonal2 = (1..=4).map(|x| self.board.get(&Pos { x, y: 4 - x }).unwrap());
        let items: Vec<&Item> = diagonal2.collect();
        let first = items[0];
        if *first != Item::Empty && items.iter().all(|item| *item == first) {
            return Some(*first);
        }

        None
    }

    fn is_full(&self) -> bool {
        for y in 0..=3 {
            for x in 1..=4 {
                if self.board.get(&Pos { x, y }).unwrap() == &Item::Empty {
                    return false;
                }
            }
        }
        true
    }

    fn make_rand(&mut self) {
        for y in 0..=3 {
            for x in 1..=4 {
                let cookie = self.rng.gen::<bool>();
                let item = if cookie { Item::Cookie } else { Item::Milk };

                self.board.insert(Pos { x, y }, item);
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
        rng: rand::rngs::StdRng::seed_from_u64(2024),
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
        .route("/12/place/:team/:column", post(place))
        .route("/12/random-board", get(random_board))
        .with_state(shared_state.clone())
        .fallback(fallback);

    Ok(router.into())
}
