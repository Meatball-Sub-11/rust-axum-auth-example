// src/handlers.rs

use crate::error::AppError;
use crate::models::{ApiResponse, AuthResponse, LoginRequest};
use crate::user_data;
use askama::Template;
use axum::{
    Json,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use sha2::{Digest, Sha256};

// --- Web UI Handlers ---

/// A template for the HTML login page.
#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginPage;

/// Serves the login page.
///
/// Returns a `Result` that will be converted into an HTML response on success,
/// or an `AppError` (which renders the 500 error page) on failure.
pub async fn show_login_page() -> Result<impl IntoResponse, AppError> {
    let page = LoginPage;
    // The `?` operator handles any potential rendering errors gracefully.
    Ok(Html(page.render()?))
}

/// A template for the HTML dashboard page, which requires the app version.
#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardPage {
    pub version: String,
}

/// Serves the dashboard page, displayed after a successful login.
pub async fn show_dashboard_page() -> Result<impl IntoResponse, AppError> {
    let version = env!("CARGO_PKG_VERSION").to_string();
    let page = DashboardPage { version };
    // The `?` operator handles any potential rendering errors gracefully.
    Ok(Html(page.render()?))
}

// --- REST API Handlers ---

/// Handles GET requests to `/status` to provide API health information.
pub async fn get_status() -> Json<ApiResponse> {
    Json(ApiResponse {
        status: "success".to_string(),
        message: "API is up and running!".to_string(),
        version: Some(env!("CARGO_PKG_VERSION").to_string()),
    })
}

/// Handles POST requests to `/login` to authenticate a user.
///
/// This handler returns a tuple `(StatusCode, Json<AuthResponse>)` to allow setting
/// different HTTP status codes (e.g., 200 OK, 401 Unauthorized) in the response.
pub async fn login(Json(payload): Json<LoginRequest>) -> (StatusCode, Json<AuthResponse>) {
    // Attempt to load user data from the file.
    let users = match user_data::load_users().await {
        Ok(u) => u,
        Err(e) => {
            tracing::error!("Error loading users for login attempt: {e}");
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
        // Hash the plain text password from the user file on-the-fly.
        let mut hasher = Sha256::new();
        hasher.update(user.password.as_bytes());
        let server_hash_result = hasher.finalize();
        let server_hashed_password = format!("{server_hash_result:x}");

        // Compare the newly generated hash with the hash sent from the client.
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

/// A development-only route to test the custom 500 error page.
/// This handler will always return an error.
pub async fn test_error_handler() -> Result<(), AppError> {
    Err(AppError::IoError(std::io::Error::other(
        "This is a test error to display the custom 500 page.",
    )))
}
