use std::{path::Path, sync::Arc};

use anyhow::{Context, Result};
use axum::{extract::DefaultBodyLimit, http::HeaderValue, http::Method};
use blogium::{config::APP_CONFIG, route, state::AppState};
use sqlx::SqlitePool;
use tokio::fs;
use tower_http::{cors, trace::TraceLayer};

#[tokio::main]
async fn main() -> Result<()> {
    let pool = SqlitePool::connect(&APP_CONFIG.db_url)
        .await
        .context("Cannot connect to database")?;

    fs::create_dir_all(Path::new(&APP_CONFIG.image_uploads_dir))
        .await
        .context("Cannot create dir for uploaded image files")?;

    let state = AppState::new(pool);
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let cors_layer = cors::CorsLayer::new()
        .allow_origin(
            APP_CONFIG
                .frontend_url
                .parse::<HeaderValue>()
                .context("Cannot setup cors origin")?,
        )
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(cors::Any);

    let app = route::create_router(Arc::new(state))
        .layer(DefaultBodyLimit::max(APP_CONFIG.request_body_limit))
        .layer(TraceLayer::new_for_http())
        .layer(cors_layer);

    let listener = tokio::net::TcpListener::bind(&APP_CONFIG.server_addr)
        .await
        .unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
