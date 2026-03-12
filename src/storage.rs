use std::fmt::Display;
use std::{fs, path::PathBuf};

use crate::habit::HabitStore;

pub struct Storage {
    file_path: PathBuf,
}

impl Storage {
    pub fn new(file_path: PathBuf) -> Self {
        Storage { file_path }
    }
    pub fn load_habits(&self) -> Result<HabitStore, StorageError> {
        if self.file_path.exists() {
            let file = fs::read_to_string(&self.file_path)?;
            let habit_store = serde_json::from_str(&file)?;
            Ok(habit_store)
        } else {
            Ok(HabitStore { habits: Vec::new() })
        }
    }

    pub fn save_habits(&self, habit_store: &HabitStore) -> Result<(), StorageError> {
        let habits_json = serde_json::to_string_pretty(habit_store)?;
        fs::write(&self.file_path, habits_json)?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum StorageError {
    IoError(std::io::Error),
    ParseError(serde_json::Error),
}

impl std::error::Error for StorageError {}

impl Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(error) => {
                write!(f, "Error: {}", error)
            }
            Self::ParseError(error) => {
                write!(f, "Error parsing: {}", error)
            }
        }
    }
}

impl From<std::io::Error> for StorageError {
    fn from(error: std::io::Error) -> Self {
        StorageError::IoError(error)
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(value: serde_json::Error) -> Self {
        StorageError::ParseError(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Local, NaiveDate};
    use tempfile::NamedTempFile;

    use crate::habit::Habit;

    #[test]
    fn test_save_and_load_habits() {
        let file = NamedTempFile::new().unwrap();
        let path = file.path().to_path_buf();
        let storage = Storage::new(path);
        let today: NaiveDate = Local::now().date_naive();

        let habit_store = HabitStore {
            habits: vec![Habit {
                name: "exercise".to_string(),
                completions: vec![today],
            }],
        };

        storage.save_habits(&habit_store).unwrap();
        let loaded_habit_store = storage.load_habits().unwrap();

        assert_eq!(loaded_habit_store.habits.len(), 1);
        assert_eq!(loaded_habit_store.habits[0].name, "exercise");
    }

    #[test]
    fn test_load_habits_returns_empty_when_file_missing() {
        let path = PathBuf::from("this_file_does_not_exist.json");
        let storage = Storage::new(path);
        let habit_store = storage.load_habits().unwrap();
        assert_eq!(habit_store.habits.len(), 0);
    }

    #[test]
    fn test_load_habits_returns_error_on_malformed_json() {
        let mut file = NamedTempFile::new().unwrap();
        let path = file.path().to_path_buf();
        let storage = Storage::new(path);

        use std::io::Write;

        write!(file, "not valid json {{{{").unwrap();

        let result = storage.load_habits();
        assert!(matches!(result, Err(StorageError::ParseError(_))));
    }
}
