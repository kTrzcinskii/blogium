use axum::body::Bytes;
use axum_typed_multipart::{FieldData, TryFromMultipart};

#[derive(Debug, TryFromMultipart)]
pub struct CreatePostSchema {
    pub username: String,
    pub content: String,
    #[form_data(limit = "3Mib")]
    pub image: Option<FieldData<Bytes>>,
    pub avatar_path: Option<String>,
}
