use axum::{
    extract::State,
    http,
    response::{IntoResponse, Redirect},
    Json,
};

use crate::state::ArcAppState;

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
