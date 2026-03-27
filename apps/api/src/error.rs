use axum::{
    Json,
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

use crate::habit::HabitError;
use crate::storage::StorageError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Habit(#[from] HabitError),
    #[error(transparent)]
    Storage(#[from] StorageError),
    #[error("invalid request body")]
    InvalidJson(#[from] JsonRejection),
    #[error("error: {0}")]
    InvalidInput(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Habit(HabitError::HabitNotFound(_)) => {
                (StatusCode::NOT_FOUND, self.to_string())
            }
            AppError::Habit(HabitError::DuplicateHabit(_)) => {
                (StatusCode::CONFLICT, self.to_string())
            }
            AppError::Habit(HabitError::DuplicateCompletion(_)) => {
                (StatusCode::CONFLICT, self.to_string())
            }
            AppError::InvalidJson(_) => {
                (StatusCode::BAD_REQUEST, "invalid request body".to_string())
            }
            AppError::InvalidInput(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            _ => {
                tracing::error!(error = ?self, "internal server error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal server error".to_string(),
                )
            }
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
