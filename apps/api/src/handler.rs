use axum::{
    Json,
    extract::{Path, State, rejection::JsonRejection},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;

use crate::error::AppError;
use crate::service;
use crate::storage::SharedState;

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

pub async fn complete_habit(
    State(shared_state): State<SharedState>,
    Path(name): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let storage = shared_state.write().expect("habit store lock was poisoned");

    let habit_response = service::complete_habit(&storage, &name)?;
    Ok((StatusCode::OK, Json(habit_response)))
}
