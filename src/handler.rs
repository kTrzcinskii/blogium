use std::{mem, path::Path};

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

use crate::{
    config,
    errors::ServerError,
    responses::ServerResponse,
    schema::CreatePostSchema,
    services::{image_service::ImageService, post_service::PostService},
    state::ArcAppState,
};

pub async fn create_post_handler(
    State(app_state): State<ArcAppState>,
    TypedMultipart(mut body): TypedMultipart<CreatePostSchema>,
) -> Result<impl IntoResponse, ServerError> {
    let mut transaction = app_state.db.begin().await?;

    let post_image_id = if let Some(image_data) = body.image {
        Some(ImageService::create_new_image_entry(&mut transaction, image_data.contents).await?)
    } else {
        None
    };

    let user_avatar_id = if let Some(avatar_url) = body.avatar_path {
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
        Some(ImageService::create_new_image_entry(&mut transaction, bytes).await?)
    } else {
        None
    };
    PostService::create_new_post_entry(
        &mut transaction,
        mem::take(&mut body.username),
        mem::take(&mut body.content),
        post_image_id,
        user_avatar_id,
    )
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
