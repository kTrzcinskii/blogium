use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ImageModel {
    pub id: i64,
    pub uuid: String,
}
