use std::sync::Arc;

use anyhow::{Context, Result};
use blogium::{route, state::AppState};
use sqlx::SqlitePool;

// TODO: put vars such as db url and port in envs

#[tokio::main]
async fn main() -> Result<()> {
    let pool = SqlitePool::connect("sqlite://blogium_db.sqlite")
        .await
        .context("Cannot connect to database")?;
    let state = AppState::new(pool);

    let app = route::create_router(Arc::new(state));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
