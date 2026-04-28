use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::error::Error;

pub async fn init_db() -> Result<Pool<Sqlite>, Box<dyn Error>> {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:yt-dl.db".to_string());
    
    // Ensure the database file exists or create it
    if !database_url.starts_with("sqlite::memory:") {
        let path = database_url.trim_start_matches("sqlite:");
        if !std::path::Path::new(path).exists() {
            println!(">>> CREATING DATABASE FILE: {}", path);
            std::fs::File::create(path)?;
        }
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Create the table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS downloads (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            url TEXT NOT NULL,
            title TEXT,
            status TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(&pool)
    .await?;

    println!(">>> DATABASE INITIALIZED: {}", database_url);
    Ok(pool)
}
