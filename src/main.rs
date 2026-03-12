mod command;
mod habit;
mod storage;

use std::fmt::Display;
use std::path::PathBuf;

use clap::Parser;

use crate::command::{Cli, Commands};
use crate::habit::HabitError;
use crate::storage::{Storage, StorageError};

enum AppError {
    Habit(HabitError),
    Storage(StorageError),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Habit(e) => e.fmt(f),
            Self::Storage(e) => e.fmt(f),
        }
    }
}

impl From<HabitError> for AppError {
    fn from(error: HabitError) -> Self {
        Self::Habit(error)
    }
}

impl From<StorageError> for AppError {
    fn from(error: StorageError) -> Self {
        Self::Storage(error)
    }
}

fn run() -> Result<(), AppError> {
    let cli = Cli::parse();

    let storage = Storage::new(PathBuf::from("habits.json"));
    let mut habit_store = storage.load_habits()?;

    match cli.command {
        Commands::Add { name } => {
            habit_store.add_habit(&name)?;
            storage.save_habits(&habit_store)?;
            println!("✓ Added habit: {}", name)
        }
        Commands::List => {
            if !habit_store.habits.is_empty() {
                let habit_streaks = habit_store.list_habits();

                for streak in habit_streaks {
                    println!("{streak}");
                }
            } else {
                println!("No habits added yet. Try adding a habit first.")
            }
        }
        Commands::Complete { name } => {
            habit_store.complete_habit(&name)?;
            storage.save_habits(&habit_store)?;
            println!("✓ Habit completed");
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
