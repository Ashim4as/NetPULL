use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ProgressUpdate {
    pub percent: String,
    pub speed: String,
    pub eta: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DownloadRequest { 
    pub url: String 
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DownloadRecord {
    pub id: i64,
    pub url: String,
    pub title: Option<String>,
    pub status: String,
    pub created_at: Option<String>,
}
