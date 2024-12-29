use std::sync::{Arc, Mutex};

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};

use crate::AppState;

pub async fn refill(State(state): State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    let mut state = state.lock().unwrap();
    state.bucket = 5;
}

pub async fn milk(
    State(state): State<Arc<Mutex<AppState>>>,
    headers: HeaderMap,
    body: String,
) -> impl IntoResponse {
    let content_type = headers
        .get("content-type")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("text/plain");

    println!("request: {headers:?}, {body:?}");

    // rate limiting
    let mut state = state.lock().unwrap();
    if state.bucket > 0 {
        state.bucket -= 1;
    } else {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            "No milk available\n".to_string(),
        );
    }

    if content_type == "application/json" {
        // parse json body
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
            if json.as_object().is_none() {
                return (StatusCode::BAD_REQUEST, "Invalid request\n".to_string());
            }

            // json should have exactly one of the following keys
            // - liters
            // - gallons
            // - litres
            // - pints
            // and the value should be a number
            let keys = json.as_object().unwrap().keys().collect::<Vec<&String>>();
            if keys.len() != 1 {
                return (StatusCode::BAD_REQUEST, "Invalid request\n".to_string());
            }

            let allowed_keys = vec!["liters", "litres", "gallons", "pints"];
            if !allowed_keys.contains(&keys[0].as_str()) {
                return (StatusCode::BAD_REQUEST, "Invalid request\n".to_string());
            }

            if let Some(liters) = json.get("liters").and_then(|v| v.as_f64()) {
                (
                    StatusCode::OK,
                    format!(r#"{{ "gallons": {} }}"#, liters as f64 * 0.264172),
                )
            } else if let Some(gallons) = json.get("gallons").and_then(|v| v.as_f64()) {
                (
                    StatusCode::OK,
                    format!(r#"{{ "liters": {} }}"#, gallons as f64 / 0.264172),
                )
            } else if let Some(litres) = json.get("litres").and_then(|v| v.as_f64()) {
                (
                    StatusCode::OK,
                    format!(r#"{{ "pints": {} }}"#, litres as f64 * 1.75975),
                )
            } else if let Some(pints) = json.get("pints").and_then(|v| v.as_f64()) {
                (
                    StatusCode::OK,
                    format!(r#"{{ "litres": {} }}"#, pints as f64 / 1.75975),
                )
            } else {
                (StatusCode::BAD_REQUEST, "Invalid request\n".to_string())
            }
        } else {
            (StatusCode::BAD_REQUEST, "Invalid request\n".to_string())
        }
    } else {
        (StatusCode::OK, "Milk withdrawn\n".to_string())
    }
}
