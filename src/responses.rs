use std::fmt::{Debug, Display};

use axum::{http, response::IntoResponse, Json};
use serde_json::json;

use crate::models::post_model::PostResponse;

type ListCursor = chrono::DateTime<chrono::Utc>;

#[derive(Debug)]
pub enum ServerResponse {
    Success(http::StatusCode),
    List(Vec<PostResponse>, Option<ListCursor>),
}

impl Display for ServerResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerResponse::Success(code) => write!(f, "Response: success, code: {}", code),
            ServerResponse::List(vec, curs) => {
                write!(f, "Response: list, content: {:?}, cursor: {:?}", vec, curs)
            }
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
            ServerResponse::List(vec, cursor) => {
                let body = Json(
                    json!({"status": "success", "count": vec.len(), "data": vec, "next_cursor": cursor}),
                );
                (http::StatusCode::OK, body).into_response()
            }
        }
    }
}
