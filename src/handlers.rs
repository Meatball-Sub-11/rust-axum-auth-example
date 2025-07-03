// src/handlers.rs

use askama::Template;
use axum::{
    Json,
    http::StatusCode,
    response::{Html, IntoResponse},
};
// Import the Sha256 hasher and the Digest trait from the sha2 crate.
use sha2::{Digest, Sha256};

use crate::error::AppError;
use crate::models::{ApiResponse, AuthResponse, LoginRequest};
use crate::user_data;

// ... (show_login_page, DashboardPage, show_dashboard_page, and get_status handlers remain unchanged) ...
// --- Web UI Handlers (using Askama Templates) ---

/// Askama template struct for rendering the login HTML page.
/// The `#[template(path = "login.html")]` attribute specifies the template file path relative to `CARGO_MANIFEST_DIR`.
#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginPage; // Public so main.rs can use it for direct rendering if needed.

/// Handler for the root path ("/"), which displays the login page.
pub async fn show_login_page() -> Result<impl IntoResponse, AppError> {
    // Replace .unwrap() with the '?' operator
    let html = LoginPage.render()?;
    Ok(Html(html))
}

/// Askama template struct for rendering the dashboard HTML page.
#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardPage {
    // Add a field to hold the application version.
    pub version: String,
}

/// Handler for the "/dashboard" path, displayed after successful login.
pub async fn show_dashboard_page() -> Result<impl IntoResponse, AppError> {
    // Get the application version from Cargo.toml at compile time.
    let version = env!("CARGO_PKG_VERSION").to_string();

    // Create an instance of the DashboardPage struct, passing the version.
    let dashboard_page = DashboardPage { version };

    // Replace .unwrap() here too
    let html = dashboard_page.render()?;
    Ok(Html(html))
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
        // --- NEW HASHING LOGIC ---
        // 1. Create a new SHA-256 hasher instance.
        let mut hasher = Sha256::new();

        // 2. Feed the plain text password from the user file into the hasher.
        hasher.update(user.password.as_bytes());

        // 3. Finalize the hash and get the result.
        let server_hash_result = hasher.finalize();

        // 4. Convert the hash result to a lowercase hexadecimal string.
        let server_hashed_password = format!("{server_hash_result:x}");
        // --- END OF NEW LOGIC ---

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

// NEW: A handler specifically for testing the error page.
pub async fn test_error_handler() -> Result<(), AppError> {
    // This will always fail, triggering our AppError response.
    Err(AppError::IoError(std::io::Error::other(
        "This is a test error",
    )))
}
