use std::fmt::Display;

use chrono::{Duration, Local, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HabitStore {
    pub habits: Vec<Habit>,
}

impl HabitStore {
    pub fn list_habits(&self) -> Vec<String> {
        let today: NaiveDate = Local::now().date_naive();
        let mut habit_streaks: Vec<String> = vec![];

        for habit in &self.habits {
            let streak = habit.calculate_streak(today);
            let habit_name = &habit.name;
            let habit_string = format!("{habit_name}: {streak} day streak");
            habit_streaks.push(habit_string);
        }

        habit_streaks
    }

    pub fn add_habit(&mut self, habit_name: &str) -> Result<(), HabitError> {
        if self.habits.iter().any(|h| h.name == habit_name) {
            return Err(HabitError::DuplicateHabit(habit_name.to_owned()));
        }
        let habit = Habit {
            name: habit_name.to_owned(),
            completions: Vec::new(),
        };

        self.habits.push(habit);

        Ok(())
    }

    pub fn complete_habit(&mut self, habit_name: &str) -> Result<(), HabitError> {
        let today: NaiveDate = Local::now().date_naive();
        let habit = self
            .habits
            .iter_mut()
            .find(|h| h.name == habit_name)
            .ok_or_else(|| HabitError::HabitNotFound(habit_name.to_owned()))?;

        if habit.completions.contains(&today) {
            return Err(HabitError::DuplicateCompletion(habit_name.to_owned()));
        }

        habit.add_completion(today);

        Ok(())
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

    /// Calculates the current streak of consecutive daily completions
    /// Returns 0 if the habit was not completed today or yesterday
    pub fn calculate_streak(&self, today: NaiveDate) -> u32 {
        let mut streak: u32 = 0;

        let mut sorted_completion_dates = self.completions.to_vec();
        sorted_completion_dates.sort_by(|a, b| b.cmp(a)); // descending: most recent first

        for (i, &date) in sorted_completion_dates.iter().enumerate() {
            if i == 0 {
                // A streak is only valid if it completed today or yesterday.
                // Completing it two days ago (no entry since) means the streak is broken.
                let is_recent = if let Some(yesterday) = today.pred_opt() {
                    date == today || date == yesterday
                } else {
                    // Edge case: today is the minimum possible date
                    // Only valid if completed today
                    date == today
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

#[derive(Debug)]
pub enum HabitError {
    HabitNotFound(String),
    DuplicateHabit(String),
    DuplicateCompletion(String),
}

impl std::error::Error for HabitError {}

impl Display for HabitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HabitNotFound(habit) => {
                write!(f, "Habit '{}' not found", habit)
            }
            Self::DuplicateCompletion(habit) => {
                write!(f, "{} already completed for today", habit)
            }
            Self::DuplicateHabit(habit) => {
                write!(f, "{} already exists", habit)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Days;

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
    fn test_add_completion() {
        let today: NaiveDate = Local::now().date_naive();

        let mut habit = Habit {
            name: "exercise".to_string(),
            completions: vec![],
        };

        habit.add_completion(today);

        assert_eq!(habit.completions.len(), 1);
        assert_eq!(habit.completions[0], today);
    }

    #[test]
    fn test_list_habits() {
        let today: NaiveDate = Local::now().date_naive();
        let yesterday = today.pred_opt().unwrap();

        let habit_one = Habit {
            name: "exercise".to_string(),
            completions: vec![yesterday, today],
        };
        let habit_two = Habit {
            name: "floss".to_string(),
            completions: vec![],
        };
        let habit_store = HabitStore {
            habits: vec![habit_one, habit_two],
        };

        let habit_streaks = habit_store.list_habits();
        assert_eq!(habit_streaks.len(), 2);
        assert_eq!(habit_streaks[0], "exercise: 2 day streak");
        assert_eq!(habit_streaks[1], "floss: 0 day streak");
    }
}
