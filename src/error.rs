// src/error.rs

use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use std::fmt;

/// A template for rendering a user-friendly 500 Internal Server Error page.
#[derive(Template)]
#[template(path = "error.html")]
struct ErrorPage;

/// The central error type for the application.
///
/// This enum consolidates all possible application-level errors into a single type.
/// It implements `IntoResponse`, allowing Axum to automatically convert any `AppError`
/// into a user-facing HTTP response.
#[derive(Debug)]
pub enum AppError {
    /// An error that occurs during Askama template rendering.
    TemplateError(askama::Error),
    /// An error that occurs during file I/O operations.
    IoError(std::io::Error),
}

/// Implements the `Display` trait to provide a user-friendly string representation of the error.
/// This is primarily used for logging purposes.
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::TemplateError(err) => write!(f, "Template Rendering Error: {err}"),
            AppError::IoError(err) => write!(f, "Input/Output Error: {err}"),
        }
    }
}

/// Automatically converts an `askama::Error` into our `AppError`.
/// This allows the `?` operator to work seamlessly in handlers that render templates.
impl From<askama::Error> for AppError {
    fn from(err: askama::Error) -> Self {
        AppError::TemplateError(err)
    }
}

/// Automatically converts a `std::io::Error` into our `AppError`.
/// This allows the `?` operator to work seamlessly in functions that perform file I/O.
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err)
    }
}

/// Defines how an `AppError` is converted into an HTTP response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Log the detailed, debug-formatted error for internal diagnostics.
        tracing::error!("An error occurred: {:?}", self);

        // Attempt to render the user-facing error page.
        let page = ErrorPage;
        let html_result = page.render();

        match html_result {
            // If the error page renders successfully, send it as a 500 response.
            Ok(html) => (StatusCode::INTERNAL_SERVER_ERROR, Html(html)).into_response(),
            // If the error page itself fails to render, it's a critical issue.
            // We fall back to a plain text response to avoid an infinite error loop.
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
