use chrono::{Days, NaiveDateTime, Utc};
use std::{thread, time::Duration};

pub const MAX_PAGE_SIMILAR_USERS: u8 = 40;

pub const MAX_PAGE_RECOMMENDATIONS: u8 = 20;

pub const HASH_SHIFT: u8 = 16;

const SM_HASH_SIZE: u8 = 64 - HASH_SHIFT;

/// max hamming distance
pub const MAX_HD: i32 = 64 + SM_HASH_SIZE as i32;

pub fn similarity(distance: i32) -> i32 {
    100 - (distance * 100 / MAX_HD)
}

pub fn pub_page(page: u8) -> u8 {
    page + 1
}

pub fn db_page(page: i32, max: u8) -> u8 {
    (if page <= 1 {
        1
    } else if page > max as i32 {
        max
    } else {
        page as u8
    }) - 1
}

pub fn days_ago(days: u64) -> NaiveDateTime {
    now().checked_sub_days(Days::new(days)).unwrap()
}

pub fn now() -> NaiveDateTime {
    Utc::now().naive_utc()
}

pub fn sleep(ms: u16) {
    thread::sleep(Duration::from_millis(ms as u64));
}
