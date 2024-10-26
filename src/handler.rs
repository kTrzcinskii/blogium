use std::path::Path;

use axum::{
    body::Body,
    extract::{self, State},
    http,
    response::IntoResponse,
};
use axum_typed_multipart::TypedMultipart;
use reqwest::Client;
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

    // TODO: should probably extract common logic of inserting image to some function
    // Handle post image
    let mut post_image_id: Option<i64> = None;
    if let Some(image_data) = body.image {
        let id = Uuid::new_v4().to_string();
        let file_path = images_upload_dir.join(Path::new(&id));
        let content = image_data.contents;
        fs::write(file_path, content).await?;
        let response = sqlx::query(r#"INSERT INTO images (uuid) VALUES (?)"#)
            .bind(id)
            .execute(&mut *transaction)
            .await?;
        post_image_id = Some(response.last_insert_rowid());
    }

    // Handle user avatar
    let mut user_avatar_id: Option<i64> = None;
    if let Some(avatar_url) = body.avatar_path {
        let client = Client::new();
        let error = ServerError::BadRequest("Couldn't download avatar from provided url".into());
        let avatar_response = client
            .get(&avatar_url)
            .send()
            .await
            .map_err(|_| error.clone())?;
        if !avatar_response.status().is_success() {
            return Err(error);
        }
        let content_type = avatar_response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .ok_or(ServerError::BadRequest(
                "Provided url misses content type header".into(),
            ))?;
        let content_type_value = content_type.to_str().unwrap_or_default();
        if !content_type_value.starts_with("image/") {
            return Err(ServerError::BadRequest(
                "Provided avatar url doesn't point to image.".into(),
            ));
        }
        let bytes = avatar_response
            .bytes()
            .await
            .map_err(|e| ServerError::InternalServerError(e.to_string()))?;
        let id = Uuid::new_v4().to_string();
        let avatar_path = images_upload_dir.join(Path::new(&id));
        fs::write(avatar_path, bytes).await?;
        let response = sqlx::query(r#"INSERT INTO images (uuid) VALUES (?)"#)
            .bind(id)
            .execute(&mut *transaction)
            .await?;
        user_avatar_id = Some(response.last_insert_rowid());
    }

    sqlx::query(r#"INSERT INTO posts (username, content, post_image_id, user_avatar_id) VALUES (?, ?, ?, ?)"#)
        .bind(body.username)
        .bind(body.content)
        .bind(post_image_id)
        .bind(user_avatar_id)
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
        .unwrap_or("images/jpeg");
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);
    let headers = [(http::header::CONTENT_TYPE, content_type)];

    Ok((headers, body))
}
