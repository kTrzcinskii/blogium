use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handlers::{image_handler::ImageHandler, post_handler::PostHandler},
    state::AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/posts", get(PostHandler::get_posts_list_handler))
        .route("/api/posts/create", post(PostHandler::create_post_handler))
        .route("/api/images/:uuid", get(ImageHandler::get_image_handler))
        .with_state(app_state)
}
