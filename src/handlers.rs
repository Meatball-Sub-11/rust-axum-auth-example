// src/handlers.rs
// This file contains all the HTTP handler functions for your Axum routes, using Askama for templating.

// Import necessary types from axum.
use axum::{
    response::{Html, IntoResponse}, // To send HTML responses.
    Json, // To handle JSON requests/responses.
    http::StatusCode, // To set HTTP status codes.
};
use tracing; // For logging within handlers.

// Askama imports for templating.
use askama::Template; // The core Askama Template trait and derive macro.
 
// Import models and user_data functions from their respective modules.
use crate::models::{ApiResponse, LoginRequest, AuthResponse};
use crate::user_data; // Import the module so we can use `user_data::load_users`

// --- Web UI Handlers (using Askama Templates) ---

/// Askama template struct for rendering the login HTML page.
/// The `#[template(path = "login.html")]` attribute specifies the template file path relative to `CARGO_MANIFEST_DIR`.
#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginPage; // Public so main.rs can use it for direct rendering if needed.

/// Handler for the root path ("/"), which displays the login page.
pub async fn show_login_page() -> impl IntoResponse {
    // Render the `LoginPage` template. `askama_web` provides the `IntoResponse` implementation for Askama templates.
    Html(LoginPage.render().unwrap()) // `.unwrap()` is used for simplicity; in production, you would handle the `Result` gracefully (e.g., return a 500 error page).
}

/// Askama template struct for rendering the dashboard HTML page.
#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardPage {
    // Add a field to hold the application version.
    pub version: String,
}

/// Handler for the "/dashboard" path, displayed after successful login.
pub async fn show_dashboard_page() -> impl IntoResponse {
    // Get the application version from Cargo.toml at compile time.
    let version = env!("CARGO_PKG_VERSION").to_string();
    
    // Create an instance of the DashboardPage struct, passing the version.
    let dashboard_page = DashboardPage { version };

    // Render the `DashboardPage` template.
    Html(dashboard_page.render().unwrap()) // `.unwrap()` for simplicity; handle errors gracefully in production.
}

// --- REST API Handlers ---

/// Handles GET requests to the `/status` endpoint.
/// Returns basic information about the API's status and version in JSON format.
pub async fn get_status() -> Json<ApiResponse> {
    Json(ApiResponse {
        status: "success".to_string(),
        message: "API is up and running!".to_string(),
        version: Some(env!("CARGO_PKG_VERSION").to_string()), // Get version from Cargo.toml.
    })
}

/// Handles POST requests to the `/login` endpoint for user authentication.
/// It expects a `LoginRequest` JSON payload and returns an `AuthResponse` JSON payload.
/// Sets HTTP status codes: 200 OK for success, 401 Unauthorized for failure, 500 for server errors.
pub async fn login(Json(payload): Json<LoginRequest>) -> (StatusCode, Json<AuthResponse>) {
    // Attempt to load user data from the file using the `load_users` function from the `user_data` module.
    let users = match user_data::load_users().await {
        Ok(u) => u,
        Err(e) => {
            tracing::error!("Error loading users for login attempt: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(AuthResponse {
                success: false,
                message: "Server internal error during user lookup.".to_string(),
            }));
        }
    };

    // Try to find the user by their username in the loaded HashMap.
    if let Some(user) = users.get(&payload.username) {
        // Compare the provided password hash with the stored hash.
        if user.server_hashed_password == payload.password_hash {
            tracing::info!("Login successful for user: {}", payload.username);
            (StatusCode::OK, Json(AuthResponse {
                success: true,
                message: "Login successful!".to_string(),
            }))
        } else {
            // Log and return Unauthorized if password hash doesn't match.
            tracing::warn!("Failed login attempt for user: {} (invalid password hash)", payload.username);
            (StatusCode::UNAUTHORIZED, Json(AuthResponse {
                success: false,
                message: "Invalid username or password.".to_string(),
            }))
        }
    } else {
        // Log and return Unauthorized if user not found.
        tracing::warn!("Failed login attempt for unknown user: {}", payload.username);
        (StatusCode::UNAUTHORIZED, Json(AuthResponse {
            success: false,
            message: "Invalid username or password.".to_string(),
        }))
    }
}
