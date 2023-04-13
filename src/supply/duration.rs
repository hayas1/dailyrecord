use std::ops::{Add, Range};

use chrono::{Duration, NaiveDate, NaiveDateTime, Timelike};

pub fn range_date(date: &NaiveDate) -> Range<NaiveDateTime> {
    let midnight = date.and_time(Default::default());
    duration_range(midnight, Duration::days(1))
}

pub fn duration_range<T: Timelike + Clone + Add<Duration, Output = T>>(dt: T, duration: Duration) -> Range<T> {
    dt.clone()..(dt + duration)
}
