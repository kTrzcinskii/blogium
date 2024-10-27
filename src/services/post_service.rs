use chrono::{DateTime, Utc};
use sqlx::SqliteConnection;

use crate::{
    errors::ServerError,
    models::post_model::{PostModel, PostResponse},
};

use super::image_service::ImageService;

pub struct PostService;

impl PostService {
    pub async fn transform_model_to_response(
        conn: &mut SqliteConnection,
        model: &PostModel,
    ) -> Result<PostResponse, ServerError> {
        let image_url = if let Some(id) = model.post_image_id {
            Some(ImageService::get_image_url(conn, id).await?)
        } else {
            None
        };

        let avatar_url = if let Some(id) = model.user_avatar_id {
            Some(ImageService::get_image_url(conn, id).await?)
        } else {
            None
        };

        Ok(PostResponse {
            username: model.username.clone(),
            content: model.content.clone(),
            posted_at: model.posted_at,
            post_image_url: image_url,
            user_avatar_url: avatar_url,
        })
    }

    /// Inserts new post entry into db
    /// Returns id of inserted record
    pub async fn create_new_post_entry(
        conn: &mut SqliteConnection,
        username: String,
        content: String,
        post_image_id: Option<i64>,
        user_avatar_id: Option<i64>,
    ) -> Result<i64, ServerError> {
        let response = sqlx::query(r#"INSERT INTO posts (username, content, post_image_id, user_avatar_id) VALUES (?, ?, ?, ?)"#)
        .bind(username)
        .bind(content)
        .bind(post_image_id)
        .bind(user_avatar_id)
        .execute(conn)
        .await?;
        Ok(response.last_insert_rowid())
    }

    pub async fn get_posts_list(
        conn: &mut SqliteConnection,
        limit: usize,
        cursor: DateTime<Utc>,
    ) -> Result<Vec<PostModel>, ServerError> {
        todo!()
    }
}
