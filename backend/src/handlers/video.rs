use axum::{
    extract::{Query, State},
    response::sse::{Event, Sse},
};
use sqlx::SqlitePool;
use crate::models::DownloadRequest;
use crate::services::downloader;
use futures::StreamExt;
use std::convert::Infallible;

pub async fn download_handler(
    State(pool): State<SqlitePool>,
    Query(payload): Query<DownloadRequest>
) -> Sse<impl futures::Stream<Item = Result<Event, Infallible>>> {
    println!(">>> API RECEIVED REQUEST: {}", payload.url);

    let stream = downloader::stream_download(payload.url, payload.quality, pool).map(|update| {
        let json = serde_json::to_string(&update).unwrap();
        Ok(Event::default().data(json))
    });

    Sse::new(stream)
}
