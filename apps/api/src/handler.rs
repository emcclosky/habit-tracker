use axum::{
    Json,
    extract::{Path, State, rejection::JsonRejection},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::NaiveDate;
use serde::Deserialize;

use crate::error::AppError;
use crate::service;
use crate::storage::SharedState;

const DATE_FORMAT: &str = "%Y-%m-%d";
const DATE_FORMAT_ERROR: &str = "Invalid date format, expected YYYY-MM-DD";

#[derive(Deserialize, Debug)]
pub struct CreateHabitRequest {
    name: String,
}

pub async fn get_habits(
    State(shared_state): State<SharedState>,
) -> Result<impl IntoResponse, AppError> {
    let storage = shared_state.read().expect("habit store lock was poisoned");
    let habits = service::list_habits(&storage)?;

    Ok((StatusCode::OK, Json(habits)))
}

pub async fn add_habit(
    State(shared_state): State<SharedState>,
    body: Result<Json<CreateHabitRequest>, JsonRejection>,
) -> Result<impl IntoResponse, AppError> {
    let Json(body) = body.map_err(AppError::InvalidJson)?;
    let storage = shared_state.write().expect("habit store lock was poisoned");
    let habit_name = body.name.trim().to_string();

    if habit_name.is_empty() {
        return Err(AppError::InvalidInput(
            "name must not be empty or whitespace".to_string(),
        ));
    }

    let habits = service::add_habit(&storage, &habit_name)?;
    Ok((StatusCode::CREATED, Json(habits)))
}

pub async fn delete_habit(
    State(shared_state): State<SharedState>,
    Path(name): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let name = name.trim().to_string();
    let storage = shared_state.write().expect("habit store lock was poisoned");

    service::delete_habit(&storage, &name)?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn complete_habit(
    State(shared_state): State<SharedState>,
    Path((name, date)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    let name = name.trim().to_string();
    let completion_date = NaiveDate::parse_from_str(date.trim(), DATE_FORMAT)
        .map_err(|_| AppError::InvalidInput(DATE_FORMAT_ERROR.to_string()))?;
    let storage = shared_state.write().expect("habit store lock was poisoned");
    let habit_response = service::complete_habit(&storage, &name, completion_date)?;

    Ok((StatusCode::OK, Json(habit_response)))
}

pub async fn delete_completion(
    State(shared_state): State<SharedState>,
    Path((name, date)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    let name = name.trim().to_string();
    let completion_date = NaiveDate::parse_from_str(date.trim(), DATE_FORMAT)
        .map_err(|_| AppError::InvalidInput(DATE_FORMAT_ERROR.to_string()))?;
    let storage = shared_state.write().expect("habit store lock was poisoned");
    let habit_response = service::delete_completion(&storage, &name, completion_date)?;

    Ok((StatusCode::OK, Json(habit_response)))
}
