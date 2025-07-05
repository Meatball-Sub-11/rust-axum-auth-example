// src/models.rs
// This file defines the data structures used for API requests and responses.

use serde::{Deserialize, Serialize};

/// Represents the JSON response structure for the `/status` GET endpoint.
#[derive(Serialize)]
pub struct ApiResponse {
    pub status: String,
    pub message: String,
    // This attribute ensures that if `version` is `None`, it won't be included
    // in the serialized JSON, keeping the output clean.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Represents the JSON request body for the `/login` POST endpoint.
#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    // The client is expected to send the password pre-hashed with SHA-256.
    pub password_hash: String,
}

/// Represents the JSON response body for the `/login` POST endpoint.
#[derive(Serialize)]
pub struct AuthResponse {
    pub success: bool,
    pub message: String,
}
