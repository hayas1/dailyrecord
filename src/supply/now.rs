// ref: https://qiita.com/alivelime/items/8ae9d63af8963bb8961d#%E3%83%A2%E3%82%B8%E3%83%A5%E3%83%BC%E3%83%AB%E9%83%A8%E5%88%86
#[cfg(not(test))]
pub fn now() -> chrono::DateTime<chrono::FixedOffset> {
    chrono::Local::now().into()
}
#[cfg(test)]
pub fn now() -> chrono::DateTime<chrono::FixedOffset> {
    // TODO set arbitrary date time
    chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339("2023-04-01T15:00:00+09:00").unwrap()
}
pub fn naive_now() -> chrono::NaiveDateTime {
    now().naive_local()
}
pub fn naive_today() -> chrono::NaiveDate {
    now().date_naive()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Local};

    #[test]
    fn test_now() {
        let now: DateTime<Local> = now().into();
        assert_eq!(
            now,
            chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339("2023-04-01T15:00:00+09:00").unwrap()
        );
    }
}
