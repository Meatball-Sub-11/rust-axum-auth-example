// src/user_data.rs
// This file manages reading and initialising user data from the `users.txt` file.

use crate::error::AppError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::fs;

/// The path to the text file where user data is stored as a JSON array.
pub const USER_DATA_FILE: &str = "users.txt";

/// Represents a single user's stored information.
///
/// This struct is designed to be easily serialized to and from JSON.
/// In this version of the project, it stores the password in plain text
/// for educational purposes, to demonstrate on-the-fly hashing on the server.
/// **This is not secure and should not be used in production.**
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub username: String,
    // The user's plain text password.
    pub password: String,
}

/// Asynchronously loads user data from `USER_DATA_FILE` into a `HashMap`.
///
/// This allows for efficient, O(1) lookups by username.
/// Returns an `AppError` if the file cannot be read or if the JSON is malformed.
pub async fn load_users() -> Result<HashMap<String, User>, AppError> {
    // The `?` operator will automatically convert any `std::io::Error` into our `AppError`.
    let content = fs::read_to_string(USER_DATA_FILE).await?;

    // Attempt to parse the file content as a vector of `User` structs.
    let users: Vec<User> = serde_json::from_str(&content)
        // Manually map the serde error to our AppError type for consistent error handling.
        .map_err(|e| {
            tracing::error!("Failed to parse user data from '{}': {}", USER_DATA_FILE, e);
            // We wrap the serde error in a generic I/O error.
            AppError::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to parse user data from file.",
            ))
        })?;

    // Convert the vector of users into a HashMap for efficient username lookups.
    let user_map = users.into_iter().map(|u| (u.username.clone(), u)).collect();

    Ok(user_map)
}

/// Creates the `users.txt` file with a set of default users if it does not exist.
/// This is a convenience function for development to ensure the app can start up.
pub async fn setup_initial_users() -> Result<(), Box<dyn std::error::Error>> {
    let users = vec![
        User {
            username: "testuser".to_string(),
            password: "testpassword".to_string(),
        },
        User {
            username: "admin".to_string(),
            password: "adminpassword".to_string(),
        },
        User {
            username: "easyuser".to_string(),
            password: "123".to_string(),
        },
    ];
    // Serialize the user data into a nicely formatted JSON string.
    let json_data = serde_json::to_string_pretty(&users)?;
    fs::write(USER_DATA_FILE, json_data).await?;
    tracing::info!(
        "Initial users created in {} with plain text passwords.",
        USER_DATA_FILE
    );
    Ok(())
}
