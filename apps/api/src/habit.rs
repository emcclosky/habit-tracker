use chrono::{Duration, NaiveDate};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct HabitStore {
    pub habits: Vec<Habit>,
}

impl HabitStore {
    pub fn add_habit(&mut self, habit_name: &str) -> Result<&mut Habit, HabitError> {
        if self.habits.iter().any(|h| h.name == habit_name) {
            return Err(HabitError::DuplicateHabit(habit_name.to_owned()));
        }
        let habit = Habit {
            name: habit_name.to_owned(),
            completions: Vec::new(),
        };

        self.habits.push(habit);
        Ok(self
            .habits
            .last_mut()
            .expect("habits vec cannot be empty after push"))
    }

    pub fn delete_habit(&mut self, habit_name: &str) -> Result<(), HabitError> {
        let index = self
            .habits
            .iter()
            .position(|h| h.name == habit_name)
            .ok_or_else(|| HabitError::HabitNotFound(habit_name.to_owned()))?;

        self.habits.remove(index);

        Ok(())
    }

    pub fn complete_habit(
        &mut self,
        habit_name: &str,
        completion_date: NaiveDate,
    ) -> Result<&mut Habit, HabitError> {
        let habit = self
            .habits
            .iter_mut()
            .find(|h| h.name == habit_name)
            .ok_or_else(|| HabitError::HabitNotFound(habit_name.to_owned()))?;

        if habit.completions.contains(&completion_date) {
            return Err(HabitError::DuplicateCompletion(habit_name.to_owned()));
        }

        habit.add_completion(completion_date);

        Ok(habit)
    }

    pub fn delete_completion(
        &mut self,
        habit_name: &str,
        completion_date: NaiveDate,
    ) -> Result<&mut Habit, HabitError> {
        let habit = self
            .habits
            .iter_mut()
            .find(|h| h.name == habit_name)
            .ok_or_else(|| HabitError::HabitNotFound(habit_name.to_owned()))?;

        habit.remove_completion(completion_date)?;

        Ok(habit)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Habit {
    pub name: String,
    pub completions: Vec<NaiveDate>,
}

impl Habit {
    pub fn add_completion(&mut self, completion_date: NaiveDate) {
        self.completions.push(completion_date);
    }

    pub fn remove_completion(&mut self, completion_date: NaiveDate) -> Result<(), HabitError> {
        let index = self
            .completions
            .iter()
            .position(|&d| d == completion_date)
            .ok_or_else(|| HabitError::CompletionNotFound(completion_date.to_string()))?;

        self.completions.remove(index);

        Ok(())
    }

    /// Calculates the current streak of consecutive daily completions
    /// Returns 0 if the habit was not completed today or yesterday
    pub fn calculate_streak(&self, completion_date: NaiveDate) -> u32 {
        let mut streak: u32 = 0;

        let mut sorted_completion_dates = self.completions.to_vec();
        sorted_completion_dates.sort_by(|a, b| b.cmp(a)); // descending: most recent first
        sorted_completion_dates.dedup(); // remove duplicate entries for the same day

        for (i, &date) in sorted_completion_dates.iter().enumerate() {
            if i == 0 {
                // A streak is only valid if it completed today or yesterday.
                // Completing it two days ago (no entry since) means the streak is broken.
                let is_recent = if let Some(yesterday) = completion_date.pred_opt() {
                    date == completion_date || date == yesterday
                } else {
                    // Edge case: today is the minimum possible date
                    // Only valid if completed today
                    date == completion_date
                };

                if !is_recent {
                    break;
                }
                streak += 1;
                continue;
            }

            // Check this date is exactly one day before the previous entry.
            // Any gap larger than one day means the streak is broken.
            let previous_date = sorted_completion_dates[i - 1];
            if previous_date - date == Duration::days(1) {
                streak += 1;
            } else {
                break;
            }
        }

        streak
    }
}

#[derive(Debug, Error)]
pub enum HabitError {
    #[error("Habit '{0}' not found")]
    HabitNotFound(String),
    #[error("Habit '{0}' already exists")]
    DuplicateHabit(String),
    #[error("Habit '{0}' already has a completion for that date")]
    DuplicateCompletion(String),
    #[error("Completion '{0}' not found")]
    CompletionNotFound(String),
}

#[cfg(test)]
mod tests {
    use chrono::{Days, Local};

    use super::*;

    #[test]
    fn test_streak_is_zero_when_last_completion_was_two_days_ago() {
        let today: NaiveDate = Local::now().date_naive();
        let three_days_ago = today.checked_sub_days(Days::new(3)).unwrap();
        let two_days_ago = today.checked_sub_days(Days::new(2)).unwrap();

        let habit = Habit {
            completions: vec![two_days_ago, three_days_ago],
            name: "exercise".to_string(),
        };

        let streak = habit.calculate_streak(today);
        assert_eq!(streak, 0);
    }

    #[test]
    fn test_streak_counts_yesterday_as_active() {
        let today: NaiveDate = Local::now().date_naive();
        let yesterday = today.pred_opt().unwrap();

        let habit = Habit {
            completions: vec![yesterday],
            name: "exercise".to_string(),
        };

        let streak = habit.calculate_streak(today);
        assert_eq!(streak, 1);
    }

    #[test]
    fn test_streak_counts_consecutive_today_and_yesterday() {
        let today: NaiveDate = Local::now().date_naive();
        let yesterday = today.pred_opt().unwrap();

        let habit = Habit {
            completions: vec![yesterday, today],
            name: "exercise".to_string(),
        };

        let streak = habit.calculate_streak(today);
        assert_eq!(streak, 2);
    }

    #[test]
    fn test_add_habit_to_store() {
        let existing_habit = Habit {
            name: String::from("floss"),
            completions: vec![],
        };
        let new_habit = Habit {
            name: String::from("exercise"),
            completions: vec![],
        };

        let mut habit_store = HabitStore {
            habits: vec![existing_habit],
        };

        habit_store.add_habit(&new_habit.name).unwrap();

        assert_eq!(habit_store.habits.len(), 2);
        assert_eq!(habit_store.habits[1].name, "exercise");
    }

    #[test]
    fn test_add_habit_to_empty_store() {
        let habit = Habit {
            name: String::from("exercise"),
            completions: vec![],
        };
        let mut habit_store = HabitStore { habits: vec![] };

        habit_store.add_habit(&habit.name).unwrap();

        assert_eq!(habit_store.habits.len(), 1);
        assert_eq!(habit_store.habits[0].name, "exercise");
    }

    #[test]
    fn test_delete_habit_from_store() {
        let habit_1_name = String::from("floss");
        let habit_2_name = String::from("exercise");

        let habit_1 = Habit {
            name: habit_1_name.clone(),
            completions: vec![],
        };
        let habit_2 = Habit {
            name: habit_2_name.clone(),
            completions: vec![],
        };

        let mut habit_store = HabitStore {
            habits: vec![habit_1, habit_2],
        };

        habit_store.delete_habit(&habit_1_name).unwrap();

        assert_eq!(habit_store.habits.len(), 1);
        assert_eq!(habit_store.habits[0].name, "exercise");
    }

    #[test]
    fn test_delete_habit_returns_error_when_not_found() {
        let mut habit_store = HabitStore { habits: vec![] };
        let result = habit_store.delete_habit("exercise");
        assert!(matches!(result, Err(HabitError::HabitNotFound(_))));
    }

    #[test]
    fn test_remove_completion_returns_error_when_habit_not_found() {
        let today: NaiveDate = Local::now().date_naive();

        let mut habit_store = HabitStore { habits: vec![] };

        let result = habit_store.delete_completion("exercise", today);

        assert!(matches!(result, Err(HabitError::HabitNotFound(_))));
    }

    #[test]
    fn test_complete_habit() {
        let today: NaiveDate = Local::now().date_naive();

        let habit = Habit {
            name: "exercise".to_string(),
            completions: vec![],
        };

        let mut habit_store = HabitStore {
            habits: vec![habit],
        };

        let result = habit_store.complete_habit("exercise", today).unwrap();

        assert_eq!(result.completions.len(), 1);
        assert_eq!(result.completions[0], today);
    }

    #[test]
    fn test_complete_habit_returns_error_on_duplicate() {
        let today: NaiveDate = Local::now().date_naive();

        let habit = Habit {
            name: "exercise".to_string(),
            completions: vec![today],
        };

        let mut habit_store = HabitStore {
            habits: vec![habit],
        };

        let result = habit_store.complete_habit("exercise", today);

        assert!(matches!(result, Err(HabitError::DuplicateCompletion(_))));
    }

    #[test]
    fn test_remove_completion() {
        let today: NaiveDate = Local::now().date_naive();
        let yesterday = today.pred_opt().unwrap();

        let habit = Habit {
            completions: vec![yesterday, today],
            name: "exercise".to_string(),
        };

        let mut habit_store = HabitStore {
            habits: vec![habit],
        };

        let result = habit_store.delete_completion("exercise", today).unwrap();

        assert!(!result.completions.contains(&today));
        assert!(result.completions.contains(&yesterday));
    }

    #[test]
    fn test_remove_completion_returns_error_when_no_completion_exists() {
        let today: NaiveDate = Local::now().date_naive();

        let habit = Habit {
            completions: vec![],
            name: "exercise".to_string(),
        };

        let mut habit_store = HabitStore {
            habits: vec![habit],
        };

        let result = habit_store.delete_completion("exercise", today);

        assert!(matches!(result, Err(HabitError::CompletionNotFound(_))));
    }
}
