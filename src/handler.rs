use axum::{
    extract::State,
    http,
    response::{IntoResponse, Redirect},
    Form, Json,
};

use crate::{errors::ServerError, schema::CreatePostSchema, state::ArcAppState};

pub async fn home_page_handler(
    State(app_state): State<ArcAppState>,
) -> Result<impl IntoResponse, (http::StatusCode, Json<serde_json::Value>)> {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM posts")
        .fetch_one(&app_state.db)
        .await
        .unwrap();

    Ok(format!("Posts count: {}", row.0))
}

pub async fn redirect_to_home_handler() -> impl IntoResponse {
    Redirect::to("/home")
}

pub async fn create_post_handler(
    State(app_state): State<ArcAppState>,
    Form(body): Form<CreatePostSchema>,
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
