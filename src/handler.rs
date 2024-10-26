use axum::{extract::State, http};
use axum_typed_multipart::TypedMultipart;

use crate::{
    errors::ServerError, responses::ServerResponse, schema::CreatePostSchema, state::ArcAppState,
};

type ServerResult = Result<ServerResponse, ServerError>;

pub async fn create_post_handler(
    State(app_state): State<ArcAppState>,
    TypedMultipart(body): TypedMultipart<CreatePostSchema>,
) -> ServerResult {
    // TODO: handle images
    println!("BODY: {:?}", body);

    let result = sqlx::query(r#"INSERT INTO posts (username, content) VALUES (?, ?)"#)
        .bind(body.username)
        .bind(body.content)
        .execute(&app_state.db)
        .await;

    match result {
        Ok(_) => Ok(ServerResponse::Success(http::StatusCode::CREATED)),
        Err(e) => Err(e.into()),
    }
}
