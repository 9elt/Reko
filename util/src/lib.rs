use chrono::{Days, NaiveDateTime, Utc};
use std::{thread, time::Duration};

pub const MAX_MAL_REQ_PER_SECOND: u16 = 3;

pub const MAX_PAGE_SIMILAR_USERS: u8 = 40;

pub const MAX_PAGE_RECOMMENDATIONS: u8 = 20;

pub const HASH_MASK: u64 = 0b1110010101000110101000000110000001000011000000000001000010010001;

pub const MAX_HAMMING_DISTANCE: i32 = 64 + HASH_MASK.count_ones() as i32;

pub fn similarity(distance: i32) -> i32 {
    100 - (distance * 100 / MAX_HAMMING_DISTANCE)
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

pub fn days_from(date: NaiveDateTime) -> usize {
    now().signed_duration_since(date).num_days() as usize
}

pub fn now() -> NaiveDateTime {
    Utc::now().naive_utc()
}

pub fn sleep(ms: u16) {
    thread::sleep(Duration::from_millis(ms as u64));
}

pub fn clamp(n: usize, min: usize, max: usize) -> usize {
    if n < min {
        min
    } else if n > max {
        max
    } else {
        n
    }
}
