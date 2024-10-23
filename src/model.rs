use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PostModel {
    pub id: i64,
    pub username: String,
    pub content: String,
    pub posted_at: chrono::DateTime<chrono::Utc>,
    pub image_path: Option<String>,
    pub avatar_path: Option<String>,
}
