use chrono::{DateTime, NaiveDateTime, Utc};

pub fn parse_datetime(value: &str) -> Option<String> {
    if value.is_empty() {
        None
    } else {
        DateTime::parse_from_rfc3339(value)
            .map(|dt| dt.with_timezone(&Utc).to_rfc3339())
            .ok()
    }
}
