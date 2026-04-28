mod models;
mod services;
mod handlers;
mod routes;
mod db;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Load .env if it exists
    let _ = dotenvy::dotenv();

    // 1. Initialize Database
    let pool = db::init_db().await.expect("Failed to initialize database");

    // 2. Get our router from the routes module
    let app = routes::create_router(pool);

    // 3. Start the listener
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    
    axum::serve(listener, app).await.unwrap();
}
