// src/main.rs

use axum::{
    Router,
    Router,
    routing::{get, post},
};
// 1. Import ServeDir for serving static files
use tower_http::services::ServeDir;

use dotenvy::dotenv;
use std::env;
use tokio::fs;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;
mod models;
mod sha2_manual;
mod user_data;

use user_data::{USER_DATA_FILE, setup_initial_users};

#[tokio::main]
async fn main() {
    // NEW: Load the .env file
    dotenv().ok();

    // --- Logging and initial user setup ---
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_axum_auth_example=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    if fs::metadata(USER_DATA_FILE).await.is_err() {
        tracing::info!("'{}' not found. Setting up initial users.", USER_DATA_FILE);
        if let Err(e) = setup_initial_users().await {
            tracing::error!("Failed to set up initial users: {}", e);
            std::process::exit(1);
        }
    }

    // --- Define Application Routes ---
    let app = Router::new()
        .route("/", get(handlers::show_login_page))
        .route("/status", get(handlers::get_status))
        .route("/login", post(handlers::login))
        .route("/dashboard", get(handlers::show_dashboard_page))
        .fallback_service(ServeDir::new("public"));

    // --- Start the HTTP Server ---

    // NEW: Read the server address from the environment, with a fallback default
    let server_address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "0.0.0.0:3000".to_string());

    // UPDATED: Use the server_address variable
    let listener = tokio::net::TcpListener::bind(&server_address)
        .await
        .expect("Failed to bind listener to address.");

    // NEW: Create a separate, clickable URL for logging
    // It replaces "0.0.0.0" with "localhost" and adds "http://"
    let display_url = format!("http://{}", server_address.replace("0.0.0.0", "localhost"));

    // UPDATED: Log the new clickable URL
    tracing::info!("🦀 Backend listening on {} 🦀", display_url);

    axum::serve(listener, app)
        .await
        .expect("Server failed to start or encountered a runtime error.");
}
