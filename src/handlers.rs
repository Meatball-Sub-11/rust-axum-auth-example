// src/handlers.rs

use crate::models::{ApiResponse, AuthResponse, LoginRequest};
use crate::sha2_manual;
use crate::user_data;
use askama::Template;
use axum::{
    Json,
    http::StatusCode,
    response::{Html, IntoResponse},
};
// ... (show_login_page, DashboardPage, show_dashboard_page, and get_status handlers remain unchanged) ...
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

/// Handles POST requests to the `/login` endpoint.
/// Now performs SHA-256 hashing on the backend.
pub async fn login(Json(payload): Json<LoginRequest>) -> (StatusCode, Json<AuthResponse>) {
    let users = match user_data::load_users().await {
        Ok(u) => u,
        Err(e) => {
            tracing::error!("Error loading users for login attempt: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AuthResponse {
                    success: false,
                    message: "Server internal error during user lookup.".to_string(),
                }),
            );
        }
    };

    if let Some(user) = users.get(&payload.username) {
        tracing::info!("✅ Using MANUAL SHA-256 implementation to hash password.");

        // --- Use the manual SHA-256 implementation ---
        let server_hashed_password = sha2_manual::digest(&user.password);

        // Compare the newly generated hash with the hash from the client.
        if server_hashed_password == payload.password_hash {
            tracing::info!("Login successful for user: {}", payload.username);
            (
                StatusCode::OK,
                Json(AuthResponse {
                    success: true,
                    message: "Login successful!".to_string(),
                }),
            )
        } else {
            tracing::warn!(
                "Failed login attempt for user: {} (invalid password)",
                payload.username
            );
            (
                StatusCode::UNAUTHORIZED,
                Json(AuthResponse {
                    success: false,
                    message: "Invalid username or password.".to_string(),
                }),
            )
        }
    } else {
        tracing::warn!(
            "Failed login attempt for unknown user: {}",
            payload.username
        );
        (
            StatusCode::UNAUTHORIZED,
            Json(AuthResponse {
                success: false,
                message: "Invalid username or password.".to_string(),
            }),
        )
    }
}
