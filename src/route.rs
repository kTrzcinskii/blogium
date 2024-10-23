use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{handler, state::AppState};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(handler::redirect_to_home_handler))
        .route("/home", get(handler::home_page_handler))
        .with_state(app_state)
}
