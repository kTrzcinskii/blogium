use std::path::Path;

use bytes::Bytes;
use sqlx::SqliteConnection;
use tokio::fs;
use uuid::Uuid;

use crate::{config, errors::ServerError, models::image_model::ImageModel};

pub struct ImageService;

impl ImageService {
    pub async fn get_image_url(
        conn: &mut SqliteConnection,
        image_id: i64,
    ) -> Result<String, ServerError> {
        let result: Result<ImageModel, ServerError> =
            sqlx::query_as("SELECT * FROM images WHERE id = $1")
                .bind(image_id)
                .fetch_one(conn)
                .await
                .map_err(|e| e.into());
        result.map(|im| format!("/api/images/{}", im.uuid))
    }

    /// Creates new file with image contents and inserts new record into db
    /// Returns id of inserted record
    pub async fn create_new_image_entry(
        conn: &mut SqliteConnection,
        content: Bytes,
    ) -> Result<i64, ServerError> {
        let id = Uuid::new_v4().to_string();
        Self::create_image_file(&id, content).await?;
        let response = sqlx::query(r#"INSERT INTO images (uuid) VALUES (?)"#)
            .bind(id)
            .execute(conn)
            .await?;
        Ok(response.last_insert_rowid())
    }

    async fn create_image_file(uuid: &str, content: Bytes) -> Result<(), ServerError> {
        let images_upload_dir = Path::new(config::IMAGE_UPLOADS_DIR);
        let file_path = images_upload_dir.join(Path::new(uuid));
        fs::write(file_path, content).await?;
        Ok(())
    }
}
