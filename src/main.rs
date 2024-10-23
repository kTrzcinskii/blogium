use anyhow::{Context, Result};
use axum::{extract::State, routing::get, Router};
use sqlx::{Pool, Sqlite, SqlitePool};

// TODO: put vars such as db url and port in envs

#[tokio::main]
async fn main() -> Result<()> {
    let pool = SqlitePool::connect("sqlite://blogium_db.sqlite")
        .await
        .context("Cannot connect to database")?;

    let app = Router::new()
        .route("/", get(get_posts_count))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn get_posts_count(State(pool): State<Pool<Sqlite>>) -> String {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM posts")
        .fetch_one(&pool)
        .await
        .unwrap();

    format!("Posts count: {}", row.0)
}
