// src/user_data.rs

use crate::error::AppError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::fs;

pub const USER_DATA_FILE: &str = "users.txt";

/// Represents a single user's stored information.
/// NOTE: For Phase 2, this now stores the plain text password.
/// This is for educational purposes and is NOT secure for production.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub username: String,
    // The field is renamed to reflect it stores a plain text password.
    pub password: String,
}

/// Asynchronously loads user data from the `USER_DATA_FILE` into a `HashMap`.
pub async fn load_users() -> Result<HashMap<String, User>, AppError> {
    // The '?' operator will now convert std::io::Error into our AppError
    let content = fs::read_to_string(USER_DATA_FILE).await?;

    let users: Vec<User> = serde_json::from_str(&content)
        // We handle this one manually as it's not a type we've converted
        .map_err(|e| {
            tracing::error!("Failed to parse user data from '{}': {}", USER_DATA_FILE, e);
            // We can create a custom error variant for this later if needed
            AppError::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to parse user data",
            ))
        })?;

    let mut user_map = HashMap::new();
    for user in users {
        user_map.insert(user.username.clone(), user);
    }
    Ok(user_map)
}

/// Helper function to set up initial user data with PLAIN TEXT passwords.
pub async fn setup_initial_users() -> Result<(), Box<dyn std::error::Error>> {
    let users = vec![
        User {
            username: "testuser".to_string(),
            password: "testpassword".to_string(), // Storing plain text
        },
        User {
            username: "admin".to_string(),
            password: "adminpassword".to_string(), // Storing plain text
        },
        User {
            username: "easyuser".to_string(),
            password: "123".to_string(), // Storing plain text
        },
    ];
    let json_data = serde_json::to_string_pretty(&users)?;
    fs::write(USER_DATA_FILE, json_data).await?;
    tracing::info!(
        "Initial users created in {} with plain text passwords.",
        USER_DATA_FILE
    );
    Ok(())
}
