use serde::Serialize;

use crate::error::AppError;
use crate::habit::HabitStore;
use crate::storage::Storage;

#[derive(Serialize)]
pub struct HabitResponse {
    pub name: String,
    pub streak: u32,
}

fn to_habit_response(habit_store: &HabitStore) -> Vec<HabitResponse> {
    habit_store
        .habits
        .iter()
        .map(|h| HabitResponse {
            name: h.name.clone(),
            streak: h.calculate_streak(),
        })
        .collect()
}

pub fn list_habits(storage: &Storage) -> Result<Vec<HabitResponse>, AppError> {
    let habit_store = storage.load_habits()?;
    let habit_response = to_habit_response(&habit_store);

    Ok(habit_response)
}

pub fn add_habit(storage: &Storage, name: &str) -> Result<Vec<HabitResponse>, AppError> {
    let mut habit_store = storage.load_habits()?;

    habit_store.add_habit(name)?;
    storage.save_habits(&habit_store)?;

    let habit_response = to_habit_response(&habit_store);
    Ok(habit_response)
}

pub fn complete_habit(storage: &Storage, name: &str) -> Result<HabitResponse, AppError> {
    let mut habit_store = storage.load_habits()?;
    habit_store.complete_habit(name)?;
    storage.save_habits(&habit_store)?;

    let habit_response = HabitResponse {
        name: name.to_string(),
        streak: habit_store
            .habits
            .iter()
            .find(|h| h.name == name)
            .map(|h| h.calculate_streak())
            .unwrap_or(0),
    };

    Ok(habit_response)
}
