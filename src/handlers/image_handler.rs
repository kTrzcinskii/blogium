use std::path::Path;

use axum::{body::Body, extract, http, response::IntoResponse};
use tokio::fs;
use tokio_util::io::ReaderStream;

use crate::{config::APP_CONFIG, errors::ServerError};

pub struct ImageHandler;

impl ImageHandler {
    pub async fn get_image_handler(
        extract::Path(id): extract::Path<String>,
    ) -> Result<impl IntoResponse, ServerError> {
        let images_upload_dir = Path::new(&APP_CONFIG.image_uploads_dir);
        let image_path = images_upload_dir.join(Path::new(&id));
        let file = fs::File::open(&image_path)
            .await
            .map_err(|_| ServerError::NotFound("Image not found".into()))?;
        let content_type = "image/png";
        let stream = ReaderStream::new(file);
        let body = Body::from_stream(stream);
        let headers = [(http::header::CONTENT_TYPE, content_type)];

        Ok((headers, body))
    }
}
