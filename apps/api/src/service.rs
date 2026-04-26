use chrono::{Local, NaiveDate};
use serde::Serialize;

use crate::error::AppError;
use crate::habit::{Habit, HabitStore};
use crate::storage::Storage;

#[derive(Serialize)]
pub struct HabitResponse {
    pub name: String,
    pub streak: u32,
    pub completions: Vec<NaiveDate>,
}

fn build_habits_response(habit_store: &HabitStore) -> Vec<HabitResponse> {
    let today: NaiveDate = Local::now().date_naive();

    habit_store
        .habits
        .iter()
        .map(|h| build_habit_response(h, today))
        .collect()
}

fn build_habit_response(habit: &Habit, today: NaiveDate) -> HabitResponse {
    HabitResponse {
        name: habit.name.clone(),
        streak: habit.calculate_streak(today),
        completions: habit.completions.clone(),
    }
}

pub fn list_habits(storage: &Storage) -> Result<Vec<HabitResponse>, AppError> {
    let habit_store = storage.load_habits()?;
    let habit_response = build_habits_response(&habit_store);

    Ok(habit_response)
}

pub fn add_habit(storage: &Storage, name: &str) -> Result<HabitResponse, AppError> {
    let mut habit_store = storage.load_habits()?;
    let today: NaiveDate = Local::now().date_naive();

    let habit = habit_store.add_habit(name)?;
    let habit_response = build_habit_response(habit, today);

    storage.save_habits(&habit_store)?;

    Ok(habit_response)
}

pub fn complete_habit(
    storage: &Storage,
    name: &str,
    date: NaiveDate,
) -> Result<HabitResponse, AppError> {
    let mut habit_store = storage.load_habits()?;
    let today: NaiveDate = Local::now().date_naive();

    let habit = habit_store.complete_habit(name, date)?;
    let habit_response = build_habit_response(habit, today);

    storage.save_habits(&habit_store)?;

    Ok(habit_response)
}

pub fn delete_completion(
    storage: &Storage,
    name: &str,
    date: NaiveDate,
) -> Result<HabitResponse, AppError> {
    let mut habit_store = storage.load_habits()?;
    let today: NaiveDate = Local::now().date_naive();

    let habit = habit_store.delete_completion(name, date)?;
    let habit_response = build_habit_response(habit, today);

    storage.save_habits(&habit_store)?;

    Ok(habit_response)
}
