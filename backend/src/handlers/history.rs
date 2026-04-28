use axum::{extract::State, Json};
use sqlx::SqlitePool;
use crate::models::DownloadRecord;

pub async fn get_history(
    State(pool): State<SqlitePool>,
) -> Json<Vec<DownloadRecord>> {
    let records = sqlx::query_as::<_, DownloadRecord>(
        "SELECT id, url, title, status, created_at FROM downloads ORDER BY created_at DESC"
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    Json(records)
}
