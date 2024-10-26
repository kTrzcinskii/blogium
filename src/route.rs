use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{handler, state::AppState};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/posts/create", post(handler::create_post_handler))
        .route("/api/images/:uuid", get(handler::get_image_handler))
        .with_state(app_state)
}
