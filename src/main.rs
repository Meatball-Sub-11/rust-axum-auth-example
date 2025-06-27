// src/main.rs
// Main entry point for the Rust Axum web application.
// It sets up logging, initialises core components and defines the main routing structure.

// Import necessary Axum components.
use axum::{
    routing::{get, post},   // For defining HTTP GET and POST routes.
    Router                  // For building the application's routing tree.
};

// Import `tokio::fs` for asynchronous file system operations, used for initial user data setup.
use tokio::fs;

// Import `EnvFilter` from `tracing-subscriber` for flexible logging configuration.
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter
};

// Declare user created modules.
mod models;
mod user_data;
mod handlers;

// Use specific items (functions, structs, constants) from our modules.
use user_data::{setup_initial_users, USER_DATA_FILE};

/// The entry point of the application.
/// `#[tokio::main]` attribute sets up the Tokio asynchronous runtime for this function.
#[tokio::main]
async fn main() {
    // --- Initialize Logging ---
    // Sets up the `tracing` logging system to print formatted logs to the console.
    tracing_subscriber::registry()
        .with(
            // `EnvFilter` allows controlling log verbosity via the `RUST_LOG` environment variable.
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_mini_project=info,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // --- Setup Initial User Data (executed once at startup if `users.txt` is missing) ---
    if fs::metadata(USER_DATA_FILE).await.is_err() {
        tracing::info!("'{}' not found. Setting up initial users.", USER_DATA_FILE);
        if let Err(e) = setup_initial_users().await {
            tracing::error!("Failed to set up initial users: {}", e);
            std::process::exit(1);
        }
    }

    // --- Define Application Routes ---
    let app = Router::new()
        // Define a GET route for the root path ("/"): Serves the HTML login page.
        .route("/", get(handlers::show_login_page)) // Using handler from `handlers` module.

        // Define a GET route for the `/status` API endpoint: Provides server status information.
        .route("/status", get(handlers::get_status))

        // Define a POST route for the `/login` API endpoint: Handles user authentication via POST requests.
        .route("/login", post(handlers::login))

        // Define a GET route for the `/dashboard` page: Serves the embedded dashboard HTML after successful login.
        .route("/dashboard", get(handlers::show_dashboard_page));

    // --- Start the HTTP Server ---
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind listener to address.");

    // Log the address where the server is listening, guiding the user on how to access it.
    tracing::info!("🦀 Backend listening on http://localhost:3000 🦀");

    // Start serving the Axum application. This call is blocking and keeps the server running
    axum::serve(listener, app)
        .await
        .expect("Server failed to start or encountered a runtime error.");
}
