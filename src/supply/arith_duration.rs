use chrono::Duration;

pub fn div(a: &Duration, b: &Duration) -> i64 {
    // TODO Duration / Duration seem to be not supported in chrono v0.4
    a.num_milliseconds() / b.num_milliseconds()
}
