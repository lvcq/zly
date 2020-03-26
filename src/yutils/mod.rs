use chrono::{NaiveDateTime, Utc};

pub mod short_id;

pub fn current_naive_datetime() -> NaiveDateTime {
    Utc::now().naive_utc()
}