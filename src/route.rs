use std::sync::Arc;

use axum::{routing::post, Router};

use crate::{handler, state::AppState};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/posts/create", post(handler::create_post_handler))
        .with_state(app_state)
}
