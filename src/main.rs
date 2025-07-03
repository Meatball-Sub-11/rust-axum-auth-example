// src/main.rs

use axum::{
    routing::{get, post},
    Router,
};
// 1. Import ServeDir for serving static files
use tower_http::services::ServeDir;

use tokio::fs;
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter
};

mod models;
mod user_data;
mod handlers;
// This might be on a different branch, but including it for completeness
// mod sha2_manual; 

use user_data::{setup_initial_users, USER_DATA_FILE};

#[tokio::main]
async fn main() {
    // ... (logging and initial user setup remain the same) ...
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
        // API and Page routes
        .route("/", get(handlers::show_login_page))
        .route("/status", get(handlers::get_status))
        .route("/login", post(handlers::login))
        .route("/dashboard", get(handlers::show_dashboard_page))
        
        // 2. Add the static file service
        // This service will serve files from the `public` directory
        // Use .fallback_service for the static file handler.
        // This will handle any request that did not match a route above.
        .fallback_service(ServeDir::new("public"));


    // --- Start the HTTP Server ---
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind listener to address.");

    tracing::info!("🦀 Backend listening on http://localhost:3000 🦀");
    
    axum::serve(listener, app)
        .await
        .expect("Server failed to start or encountered a runtime error.");
}