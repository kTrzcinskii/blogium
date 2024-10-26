use std::path::Path;

use axum::{
    body::Body,
    extract::{self, State},
    http,
    response::IntoResponse,
};
use axum_typed_multipart::TypedMultipart;
use tokio::fs;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

use crate::{
    config, errors::ServerError, responses::ServerResponse, schema::CreatePostSchema,
    state::ArcAppState,
};

pub async fn create_post_handler(
    State(app_state): State<ArcAppState>,
    TypedMultipart(body): TypedMultipart<CreatePostSchema>,
) -> Result<impl IntoResponse, ServerError> {
    let mut transaction = app_state.db.begin().await?;

    let images_upload_dir = Path::new(config::IMAGE_UPLOADS_DIR);

    let mut post_image_id: Option<i64> = None;
    if let Some(image_data) = body.image {
        let file_name = image_data.metadata.file_name;
        let id = Uuid::new_v4().to_string();
        let file_path = images_upload_dir.join(Path::new(&id));
        let content = image_data.contents;
        fs::write(file_path, content).await?;
        let response = sqlx::query(r#"INSERT INTO images (uuid, file_name) VALUES (?, ?)"#)
            .bind(id)
            .bind(file_name)
            .execute(&mut *transaction)
            .await?;
        post_image_id = Some(response.last_insert_rowid());
    }

    // TODO: handle downloading and saving avatar here

    sqlx::query(r#"INSERT INTO posts (username, content, post_image_id) VALUES (?, ?, ?)"#)
        .bind(body.username)
        .bind(body.content)
        .bind(post_image_id)
        .execute(&mut *transaction)
        .await?;

    transaction.commit().await?;

    Ok(ServerResponse::Success(http::StatusCode::CREATED))
}

pub async fn get_image_handler(
    extract::Path(id): extract::Path<String>,
) -> Result<impl IntoResponse, ServerError> {
    let images_upload_dir = Path::new(config::IMAGE_UPLOADS_DIR);
    let image_path = images_upload_dir.join(Path::new(&id));
    let file = fs::File::open(&image_path)
        .await
        .map_err(|_| ServerError::NotFound("Image not found".into()))?;
    let content_type = mime_guess::from_path(&image_path)
        .first_raw()
        // TODO: isn't it some security breach?
        .unwrap_or("images/jpeg");
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);
    let headers = [(http::header::CONTENT_TYPE, content_type)];

    Ok((headers, body))
}
