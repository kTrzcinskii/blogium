use axum::{extract::State, http, response::IntoResponse, Json};

use crate::{errors::ServerError, schema::CreatePostSchema, state::ArcAppState};

pub async fn create_post_handler(
    State(app_state): State<ArcAppState>,
    Json(body): Json<CreatePostSchema>,
) -> Result<impl IntoResponse, ServerError> {
    // TODO: handle images
    let result = sqlx::query(r#"INSERT INTO posts (username, content) VALUES (?, ?)"#)
        .bind(body.username)
        .bind(body.content)
        .execute(&app_state.db)
        .await;

    match result {
        Ok(_) => Ok(http::StatusCode::CREATED),
        Err(e) => Err(e.into()),
    }
}
