use std::{path::Path, sync::Arc};

use anyhow::{Context, Result};
use axum::extract::DefaultBodyLimit;
use blogium::{config, route, state::AppState};
use sqlx::SqlitePool;
use tokio::fs;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = SqlitePool::connect(config::DB_URL)
        .await
        .context("Cannot connect to database")?;

    fs::create_dir_all(Path::new(config::IMAGE_UPLOADS_DIR))
        .await
        .context("Cannot create dir for uploaded image files")?;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let state = AppState::new(pool);
    let app = route::create_router(Arc::new(state))
        .layer(DefaultBodyLimit::max(config::REQUEST_BODY_LIMIT))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(config::SERVER_ADDR)
        .await
        .unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
