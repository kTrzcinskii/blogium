use std::{path::Path, sync::Arc};

use anyhow::{Context, Result};
use axum::extract::DefaultBodyLimit;
use blogium::{config, route, state::AppState};
use sqlx::SqlitePool;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = SqlitePool::connect(config::DB_URL)
        .await
        .context("Cannot connect to database")?;

    fs::create_dir_all(Path::new(config::IMAGE_UPLOADS_DIR))
        .await
        .context("Cannot create dir for uploaded image files")?;

    let state = AppState::new(pool);
    let app = route::create_router(Arc::new(state))
        .layer(DefaultBodyLimit::max(config::REQUEST_BODY_LIMIT));

    let listener = tokio::net::TcpListener::bind(config::SERVER_ADDR)
        .await
        .unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
