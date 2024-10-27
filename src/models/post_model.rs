use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PostModel {
    pub id: i64,
    pub username: String,
    pub content: String,
    pub posted_at: chrono::DateTime<chrono::Utc>,
    pub post_image_id: Option<i64>,
    pub user_avatar_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostResponse {
    pub username: String,
    pub content: String,
    pub posted_at: chrono::DateTime<chrono::Utc>,
    pub post_image_url: Option<String>,
    pub user_avatar_url: Option<String>,
}
