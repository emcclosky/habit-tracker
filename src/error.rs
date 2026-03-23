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
        let status = match &self {
            AppError::Habit(HabitError::HabitNotFound(_)) => StatusCode::NOT_FOUND,
            AppError::Habit(HabitError::DuplicateHabit(_)) => StatusCode::CONFLICT,
            AppError::Habit(HabitError::DuplicateCompletion(_)) => StatusCode::CONFLICT,
            AppError::InvalidJson(_) => StatusCode::BAD_REQUEST,
            AppError::InvalidInput(_) => StatusCode::UNPROCESSABLE_ENTITY,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, Json(json!({ "error": self.to_string() }))).into_response()
    }
}
