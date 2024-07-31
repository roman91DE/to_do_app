use chrono::{NaiveDate, Utc};
use std::error::Error;

pub struct Task {
    id: u64,
    description: String,
    created: i64,
    deadline: i64,
    completed: bool,
}

impl Task {
    pub fn new(id: u64, description: &str, deadline: &str) -> Self {
        let created_ts: i64 = Utc::now().timestamp();
        let deadline_ts: i64 = match Task::parse_date(deadline) {
            Ok(mut d) => {
                if d <= created_ts {
                    eprintln!(
                        "Deadline '{}' has already passed. Defaulting to the next day.",
                        deadline
                    );
                    d = created_ts + 86_400; // add one day in seconds
                }
                d
            }
            Err(e) => {
                eprintln!(
                    "Failed to parse deadline '{}'. Defaulting to the next day. Error: {}",
                    deadline, e
                );
                created_ts + 86_400 // add one day in seconds
            }
        };

        let completed: bool = false;

        Task {
            id,
            description: String::from(description),
            created: created_ts,
            deadline: deadline_ts,
            completed,
        }
    }

    fn parse_date(s: &str) -> Result<i64, Box<dyn Error>> {
        let format = "%Y-%m-%d";
        let naive_date = NaiveDate::parse_from_str(s, format)?;
        let naive_datetime = naive_date
            .and_hms_opt(0, 0, 0)
            .ok_or("Failed to create datetime from date")?;
        let timestamp = naive_datetime.and_utc().timestamp();
        Ok(timestamp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_task_creation_with_valid_date() {
        let description = "Test task";
        let deadline = "2024-12-31";
        let task = Task::new(1, description, deadline);

        assert_eq!(task.id, 1);
        assert_eq!(task.description, description);
        assert!(task.created <= Utc::now().timestamp());
        assert!(task.deadline > task.created);
        assert!(!task.completed);
    }

    #[test]
    fn test_task_creation_with_past_date() {
        let description = "Test task";
        let past_date = "2000-01-01";
        let task = Task::new(2, description, past_date);

        assert_eq!(task.id, 2);
        assert_eq!(task.description, description);
        assert!(task.created <= Utc::now().timestamp());
        assert!(task.deadline > task.created);
        assert!(!task.completed);
    }

    #[test]
    fn test_task_creation_with_invalid_date() {
        let description = "Test task";
        let invalid_date = "invalid-date";
        let task = Task::new(3, description, invalid_date);

        assert_eq!(task.id, 3);
        assert_eq!(task.description, description);
        assert!(task.created <= Utc::now().timestamp());
        assert!(task.deadline > task.created);
        assert!(!task.completed);
    }

    #[test]
    fn test_parse_date_with_valid_date() {
        let valid_date = "2024-12-31";
        let parsed_date = Task::parse_date(valid_date).unwrap();
        let expected_date = NaiveDate::from_ymd_opt(2024, 12, 31)
            .and_then(|d| d.and_hms_opt(0, 0, 0))
            .unwrap()
            .and_utc()
            .timestamp();
        
        assert_eq!(parsed_date, expected_date);
    }

    #[test]
    fn test_parse_date_with_invalid_date() {
        let invalid_date = "invalid-date";
        let parsed_date = Task::parse_date(invalid_date);

        assert!(parsed_date.is_err());
    }
}
