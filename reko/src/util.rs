use chrono::{Days, NaiveDateTime, Utc};

pub fn db_page(page: i32, max: u8) -> u8 {
    if page <= 1 {
        0
    } else if page > max as i32 {
        max
    } else {
        page as u8 - 1
    }
}

pub fn days_ago(days: u64) -> NaiveDateTime {
    now().checked_sub_days(Days::new(days)).unwrap()
}

pub fn now() -> NaiveDateTime {
    Utc::now().naive_utc()
}
