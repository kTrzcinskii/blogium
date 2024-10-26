use std::fmt::Display;

use axum::{http, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub enum ServerResponse {
    Success(http::StatusCode),
}

impl Display for ServerResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerResponse::Success(code) => write!(f, "Response: success, code: {}", code),
        }
    }
}

impl IntoResponse for ServerResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            ServerResponse::Success(code) => {
                let body = Json(json!({"status": "success"}));
                (code, body).into_response()
            }
        }
    }
}
