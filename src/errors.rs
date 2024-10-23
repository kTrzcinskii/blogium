use std::fmt::Display;

use axum::{http, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub enum ServerError {
    Database(String),
}

impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::Database(e) => write!(f, "Database error: {}", e),
        }
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, error_msg) = match self {
            ServerError::Database(e) => (http::StatusCode::INTERNAL_SERVER_ERROR, e),
        };
        let body = Json(json!({"status": "error", "message": format!("{:?}", error_msg)}));
        (status_code, body).into_response()
    }
}

impl From<sqlx::Error> for ServerError {
    fn from(value: sqlx::Error) -> Self {
        Self::Database(value.to_string())
    }
}
