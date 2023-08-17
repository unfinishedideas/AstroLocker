use axum::response::{IntoResponse, Response};
use axum::Json;
use http::StatusCode;
use serde_json::json;
use sqlx::Error;

#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    MissingCredentials,
    InvalidPassword,
    UserDoesNotExist,
    UserAlreadyExists,
    InvalidToken,
    InternalServerError,
    NASAError,
    InvalidDateRange,
    #[allow(dead_code)]
    Any(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(err) => (StatusCode::SERVICE_UNAVAILABLE, err.to_string()),
            AppError::Any(err) => {
                let message = format!("Internal server error! {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, message)
            }
            AppError::MissingCredentials => (
                StatusCode::UNAUTHORIZED,
                "Your credentials were missing or otherwise incorrect".to_string(),
            ),
            AppError::UserDoesNotExist => (
                StatusCode::UNAUTHORIZED,
                "Your account does not exist!".to_string(),
            ),
            AppError::UserAlreadyExists => (
                StatusCode::UNAUTHORIZED,
                "There is already an account with that email address in the system".to_string(),
            ),
            AppError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid Token".to_string()),
            AppError::InvalidPassword => (StatusCode::UNAUTHORIZED, "Invalid Password".to_string()),
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something terrible happened".to_string(),
            ),
            AppError::NASAError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something terrible happened with NASA".to_string(),
            ),
            AppError::InvalidDateRange => (
                StatusCode::BAD_REQUEST,
                "You used a value outside the legal date-range for NASA".to_string(),
            ),
        };

        let body = Json(json!({"error": error_message}));
        (status, body).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: Error) -> Self {
        AppError::Database(value)
    }
}
