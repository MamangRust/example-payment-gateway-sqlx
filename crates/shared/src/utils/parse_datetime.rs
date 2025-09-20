use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use prost_types::Timestamp;
use serde::{Deserialize, Deserializer};

pub fn parse_datetime(value: &str) -> Option<String> {
    if value.is_empty() {
        None
    } else {
        DateTime::parse_from_rfc3339(value)
            .map(|dt| dt.with_timezone(&Utc).to_rfc3339())
            .ok()
    }
}

pub fn deserialize_date_only<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let dt = DateTime::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)?;
    Ok(dt.date_naive())
}

pub fn deserialize_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let dt = DateTime::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)?;
    Ok(dt.naive_utc())
}

pub fn parse_expiration_datetime(input: &str) -> Result<NaiveDateTime, chrono::ParseError> {
    NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S")
}

pub fn timestamp_to_naive_date(ts: Option<Timestamp>) -> Option<NaiveDate> {
    ts.and_then(|t| {
        Utc.timestamp_opt(t.seconds, t.nanos as u32)
            .single()
            .map(|dt| dt.date_naive())
    })
}

pub fn timestamp_to_naive_datetime(ts: Option<Timestamp>) -> Option<NaiveDateTime> {
    ts.and_then(|t| {
        Utc.timestamp_opt(t.seconds, t.nanos as u32)
            .single()
            .map(|dt| dt.naive_utc())
    })
}

pub fn naive_date_to_timestamp(date: NaiveDate) -> Timestamp {
    let dt = date.and_hms_opt(0, 0, 0).unwrap();
    let dt_utc: DateTime<Utc> = Utc.from_utc_datetime(&dt);

    Timestamp {
        seconds: dt_utc.timestamp(),
        nanos: dt_utc.timestamp_subsec_nanos() as i32,
    }
}

pub fn naive_datetime_to_timestamp(datetime: NaiveDateTime) -> Timestamp {
    let dt_utc: DateTime<Utc> = Utc.from_utc_datetime(&datetime);

    Timestamp {
        seconds: dt_utc.timestamp(),
        nanos: dt_utc.timestamp_subsec_nanos() as i32,
    }
}
