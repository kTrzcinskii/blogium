use std::sync::Arc;

use sqlx::{Pool, Sqlite};

pub struct AppState {
    pub db: Pool<Sqlite>,
}

pub type ArcAppState = Arc<AppState>;

impl AppState {
    pub fn new(db: Pool<Sqlite>) -> Self {
        Self { db }
    }
}
