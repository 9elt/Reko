use std::{thread, time::Duration};
use chrono::{Days, NaiveDateTime, Utc};

pub const SHF_HASH: u8 = 16;
 
const SM_HASH_SIZE: u8 = 64 - SHF_HASH;

/// max hamming distance
const MAX_HD: i32 = 64 + SM_HASH_SIZE as i32;

pub fn similarity(distance: i32) -> i32 {
    100 - (distance * 100 / MAX_HD)
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
