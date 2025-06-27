// src/user_data.rs
// This file manages reading and initializing user data from `users.txt`.

use std::collections::HashMap; // For storing users in a HashMap for efficient lookups.
use tokio::fs; // For asynchronous file system operations.
use serde::{Deserialize, Serialize}; // For JSON (de)serialization of User structs.
use tracing; // Import tracing for logging.

/// Defines the path to the text file where user data is stored.
pub const USER_DATA_FILE: &str = "users.txt";

/// Represents a single user's stored information.
/// `Debug`: For easy debugging output.
/// `Deserialize`, `Serialize`: To read from and write to JSON file.
/// `Clone`: To allow easy duplication of user data if needed (e.g., for passing to other parts).
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub username: String,
    // This field stores the SHA-256 hash of the password.
    // REMINDER: For production, use a strong KDF like Argon2 for server-side hashing.
    pub server_hashed_password: String,
}

/// Asynchronously loads user data from the `USER_DATA_FILE` into a `HashMap`.
/// Returns `Ok(HashMap)` on success or `Err(String)` if file reading or JSON parsing fails.
pub async fn load_users() -> Result<HashMap<String, User>, String> {
    // Attempt to read the entire file content into a String.
    let content = fs::read_to_string(USER_DATA_FILE)
        .await
        .map_err(|e| format!("Failed to read user data file '{}': {}", USER_DATA_FILE, e))?;

    // Attempt to parse the JSON string into a vector of `User` structs.
    let users: Vec<User> = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse user data from '{}': {}", USER_DATA_FILE, e))?;

    // Convert the vector of users into a HashMap for efficient lookup by username.
    let mut user_map = HashMap::new();
    for user in users {
        user_map.insert(user.username.clone(), user);
    }
    Ok(user_map)
}

/// Helper function to set up initial user data in `users.txt` if the file does not exist.
/// This is typically called once at application startup for convenience in development.
pub async fn setup_initial_users() -> Result<(), Box<dyn std::error::Error>> {
    let users = vec![
        User {
            username: "testuser".to_string(),
            server_hashed_password: "5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8"
                .to_string(), // SHA-256 of "testpassword"
        },
        User {
            username: "admin".to_string(),
            server_hashed_password: "8c6976e5b5410415bde908bd4dee15dfb167a9c873fc4bb8a81f6f2ab448a918"
                .to_string(), // SHA-256 of "adminpassword"
        },
        User {
            username: "easyuser".to_string(),
            server_hashed_password: "a665a45920422f9d417e4867efdc4fb8a04a1f3fff1fa07e998e86f7f7a27ae3"
                .to_string(), // SHA-256 of "123"
        },
    ];
    let json_data = serde_json::to_string_pretty(&users)?; // Serialize users to pretty-printed JSON.
    fs::write(USER_DATA_FILE, json_data).await?; // Write JSON to file.
    tracing::info!("Initial users created in {}.", USER_DATA_FILE);
    Ok(())
}
