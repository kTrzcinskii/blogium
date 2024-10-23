use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreatePostSchema {
    pub username: String,
    pub content: String,
    pub image_path: Option<String>,
    pub avatar_path: Option<String>,
}
