use std::fmt::Display;

use axum::{http, response::IntoResponse, Json};
use serde_json::json;
use tokio::io;

#[derive(Debug, Clone)]
pub enum ServerError {
    Database(String),
    FileSystem(String),
    NotFound(String),
    BadRequest(String),
    InternalServerError(String),
}

impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::Database(e) => write!(f, "Database error: {}", e),
            ServerError::FileSystem(e) => write!(f, "FileSystem error: {}", e),
            ServerError::NotFound(e) => write!(f, "NotFound error: {}", e),
            ServerError::BadRequest(e) => write!(f, "BadRequest error: {}", e),
            ServerError::InternalServerError(e) => write!(f, "InternalServerError: {}", e),
        }
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, error_msg) = match self {
            ServerError::Database(e) => (http::StatusCode::INTERNAL_SERVER_ERROR, e),
            ServerError::FileSystem(e) => (http::StatusCode::INTERNAL_SERVER_ERROR, e),
            ServerError::NotFound(e) => (http::StatusCode::NOT_FOUND, e),
            ServerError::BadRequest(e) => (http::StatusCode::BAD_REQUEST, e),
            ServerError::InternalServerError(e) => (http::StatusCode::INTERNAL_SERVER_ERROR, e),
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

impl From<io::Error> for ServerError {
    fn from(value: io::Error) -> Self {
        Self::FileSystem(value.to_string())
    }
}
