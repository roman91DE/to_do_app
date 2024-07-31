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
