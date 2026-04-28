use axum::{
    routing::get,
    Router,
};
use tower_http::cors::CorsLayer;
use crate::handlers;

use sqlx::SqlitePool;

pub fn create_router(pool: SqlitePool) -> Router {
    let cors = CorsLayer::permissive();

    Router::new()
        .route("/", get(|| async { "Professional Rust API is Live!" }))
        .route("/download", get(handlers::video::download_handler))
        .route("/history", get(handlers::history::get_history))
        .layer(cors)
        .with_state(pool)
}
