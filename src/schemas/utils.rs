use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct QueryOptions {
    pub cursor: Option<chrono::DateTime<chrono::Utc>>,
    pub limit: Option<usize>,
}
