use chrono::{Local, NaiveDate};
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
    let today: NaiveDate = Local::now().date_naive();

    habit_store
        .habits
        .iter()
        .map(|h| HabitResponse {
            name: h.name.clone(),
            streak: h.calculate_streak(today),
        })
        .collect()
}

pub fn list_habits(storage: &Storage) -> Result<Vec<HabitResponse>, AppError> {
    let habit_store = storage.load_habits()?;
    let habit_response = to_habit_response(&habit_store);

    Ok(habit_response)
}

pub fn add_habit(storage: &Storage, name: &str) -> Result<HabitResponse, AppError> {
    let mut habit_store = storage.load_habits()?;

    habit_store.add_habit(name)?;
    storage.save_habits(&habit_store)?;

    Ok(HabitResponse {
        name: name.to_string(),
        streak: 0,
    })
}

pub fn complete_habit(storage: &Storage, name: &str) -> Result<HabitResponse, AppError> {
    let mut habit_store = storage.load_habits()?;
    let today: NaiveDate = Local::now().date_naive();

    let habit = habit_store.complete_habit(name)?;
    let habit_response = HabitResponse {
        name: habit.name.clone(),
        streak: habit.calculate_streak(today),
    };

    storage.save_habits(&habit_store)?;

    Ok(habit_response)
}
