use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Deserialize)]
pub struct DownloadRequest {
    pub url: String,
    pub quality: Option<String>,
}


#[derive(Serialize, Clone, Default)]
pub struct ProgressUpdate {
    pub percent: String,
    pub speed: String,
    pub eta: String,
    pub status: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct DownloadRecord {
    pub id: i64,
    pub url: String,
    pub title: Option<String>,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
}
