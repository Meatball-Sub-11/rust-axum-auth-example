// src/error.rs

use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use std::fmt; // Import the fmt module

// Create a struct for our error page template
#[derive(Template)]
#[template(path = "error.html")]
struct ErrorPage;

// Our custom error type
#[derive(Debug)]
pub enum AppError {
    TemplateError(askama::Error),
    IoError(std::io::Error),
}

// UPDATED: Implement the Display trait for AppError
// This now includes the underlying error message, which resolves the compiler warnings.
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // We now include the source error in the display format.
            AppError::TemplateError(err) => write!(f, "Template Rendering Error: {err}"),
            AppError::IoError(err) => write!(f, "Input/Output Error: {err}"),
        }
    }
}

// This allows us to convert an `askama::Error` into our `AppError`
impl From<askama::Error> for AppError {
    fn from(err: askama::Error) -> Self {
        AppError::TemplateError(err)
    }
}

// This allows us to convert a `std::io::Error` into our `AppError`
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err)
    }
}

// This now renders the HTML error page
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Log the specific error for debugging purposes, regardless of what we show the user.
        // We use the Debug format `{:?}` here to get all the details.
        tracing::error!("An error occurred: {:?}", self);

        // Render the user-facing error page
        let page = ErrorPage;
        let html_result = page.render();

        // Create the final response
        match html_result {
            // If the error page itself renders correctly, send it
            Ok(html) => (StatusCode::INTERNAL_SERVER_ERROR, Html(html)).into_response(),
            // If the error page fails to render (which would be a critical issue),
            // fall back to a plain text response.
            Err(err) => {
                tracing::error!("FATAL: Could not render error page: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "A critical internal error occurred and the error page could not be displayed.",
                )
                    .into_response()
            }
        }
    }
}
