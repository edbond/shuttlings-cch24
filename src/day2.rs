use std::net::{Ipv4Addr, Ipv6Addr};

use axum::{extract::Query, http::StatusCode, response::IntoResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Task1 {
    from: Ipv4Addr,
    key: Ipv4Addr,
}

pub async fn task1(encryption: Query<Task1>) -> impl IntoResponse {
    let from = encryption.from.octets();
    let key = encryption.key.octets();

    let mut result = Vec::with_capacity(4);

    for (f, k) in from.iter().zip(key) {
        let r = add_overflow(*f, k);
        result.push(r);
    }

    let addr = Ipv4Addr::new(result[0], result[1], result[2], result[3]);

    (StatusCode::OK, addr.to_string())
}

fn add_overflow(f: u8, k: u8) -> u8 {
    ((f as u16 + k as u16) % 256) as u8
}

#[derive(Deserialize)]
pub struct Task2 {
    from: Ipv4Addr,
    to: Ipv4Addr,
}

fn sub_overflow(t: u8, f: u8) -> u8 {
    if t >= f {
        t - f
    } else {
        (256 - (f as u16) + (t as u16)) as u8
    }
}

pub async fn task2(query: Query<Task2>) -> impl IntoResponse {
    let from = query.from.octets();
    let to = query.to.octets();

    let mut result = Vec::with_capacity(4);

    for (f, t) in from.iter().zip(to) {
        let r = sub_overflow(t, *f);
        result.push(r);
    }

    let addr = Ipv4Addr::new(result[0], result[1], result[2], result[3]);

    (StatusCode::OK, addr.to_string())
}

#[derive(Deserialize)]
pub struct Task3 {
    from: Ipv6Addr,
    to: Option<Ipv6Addr>,
    key: Option<Ipv6Addr>,
}

pub async fn task1_v6(query: Query<Task3>) -> impl IntoResponse {
    let from = query.from.segments();
    let key = query.key.expect("key parameter missing").segments();

    xor(from, key)
}

pub async fn task2_v6(query: Query<Task3>) -> impl IntoResponse {
    let from = query.from.segments();
    let to = query.to.expect("to parameter missing").segments();

    xor(from, to)
}

fn xor(from: [u16; 8], to: [u16; 8]) -> impl IntoResponse {
    let mut result: Vec<u16> = Vec::with_capacity(8);

    for (f, t) in from.iter().zip(to) {
        let r = t ^ *f;
        result.push(r);
    }

    let addr = Ipv6Addr::new(
        result[0], result[1], result[2], result[3], result[4], result[5], result[6], result[7],
    );

    (StatusCode::OK, addr.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add_overflow(1, 1), 2);
        assert_eq!(add_overflow(255, 1), 0);
        assert_eq!(add_overflow(255, 2), 1);
    }
}
