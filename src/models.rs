// src/models.rs
// This file defines the data structures used for API requests and responses.

use serde::{Deserialize, Serialize}; // Import Serde traits for JSON (de)serialization.

/// Represents the response structure for the `/status` GET endpoint.
#[derive(Serialize)]
pub struct ApiResponse {
    pub status: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")] // Omit `version` if it's None in JSON.
    pub version: Option<String>,
}

/// Represents the expected request body for the `/login` POST endpoint.
/// The frontend sends the SHA-256 hashed password.
#[derive(Deserialize)] // Allows deserializing JSON into this struct.
pub struct LoginRequest {
    pub username: String,
    pub password_hash: String, // Expecting SHA-256 hashed password from client.
}

/// Represents the response body for the `/login` POST endpoint.
#[derive(Serialize)] // Allows serializing this struct into JSON.
pub struct AuthResponse {
    pub success: bool,
    pub message: String,
}
