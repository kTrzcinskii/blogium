use serde::Deserialize;

pub type QueryCursor = chrono::NaiveDateTime;

#[derive(Debug, Deserialize, Default)]
pub struct QueryOptions {
    pub cursor: Option<QueryCursor>,
    pub limit: Option<usize>,
}
