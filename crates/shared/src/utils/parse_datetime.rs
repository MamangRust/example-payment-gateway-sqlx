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
pub fn parse_expiration_datetime(input: &str) -> Result<NaiveDateTime, chrono::ParseError> {
    NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S")
}
