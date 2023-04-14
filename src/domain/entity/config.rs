use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Weekday};
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;

use crate::repository::config::WeekConfigRepository;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeekConfig {
    pub hour_px: u32,
    pub display_weekdays: Vec<Weekday>, // TODO BTreeSet ?
    pub display_hours: RangeInclusive<u32>,
}
impl Default for WeekConfig {
    fn default() -> Self {
        let hour_px = 60;
        let display_weekdays = std::iter::successors(Some(Weekday::Sun), |&wd| Some(wd.succ())).take(7).collect();
        let display_hours = 0..=23;
        Self { hour_px, display_weekdays, display_hours }
    }
}

impl WeekConfig {
    #[inline] // TODO cache
    pub fn cols() -> usize {
        1 + WeekConfigRepository::num_display_days() // col1: time legend, col2..: episode space
    }
    #[inline] // TODO cache
    pub fn height() -> u32 {
        let (start, end) = WeekConfigRepository::display_hour_range().into_inner();
        let rows = (end - start) * WeekConfigRepository::hour_px();
        rows
    }
    #[inline] // TODO!!! cache!!!
    pub fn col(weekday: &Weekday) -> Option<usize> {
        let col0 = WeekConfigRepository::display_weekdays().iter().position(|wd| wd == weekday)?;
        Some(1 + col0) // +1 by time col
    }
    #[inline]
    pub fn top(time: &NaiveTime) -> Option<u32> {
        WeekConfigRepository::display_hour_range().contains(&time.hour()).then(|| {
            let start = NaiveTime::from_hms_opt(WeekConfigRepository::display_hour_range().start().clone(), 0, 0)
                .expect("display hours should be validated");
            (time.clone() - start).num_minutes() as u32 * WeekConfigRepository::hour_px() / 60
        })
    }
    #[inline]
    pub fn pos(dt: &NaiveDateTime) -> Option<(usize, u32)> {
        match (WeekConfig::col(&dt.weekday()), WeekConfig::top(&dt.time())) {
            (Some(col), Some(top)) => Some((col, top)),
            _ => None,
        }
    }

    #[inline]
    pub fn hours_in_day() -> Vec<NaiveTime> {
        WeekConfigRepository::display_hour_range()
            .map(|h| NaiveTime::from_hms_opt(h, 0, 0).expect("display hours should be validated"))
            .collect()
    }

    #[inline]
    pub fn days_in_week(day: &NaiveDate) -> Vec<NaiveDate> {
        let (weekdays, first) = (WeekConfigRepository::display_weekdays(), WeekConfigRepository::first_weekday());
        let (mut days, mut dit) = (Vec::new(), day.week(first).first_day().iter_days().peekable());
        // two pointer method
        for weekday in weekdays.into_iter() {
            while let Some(day) = dit.next() {
                if day.weekday() == weekday {
                    days.push(day);
                    break;
                }
            }
        }
        days
    }

    #[inline]
    pub fn span(duration: &Duration) -> i64 {
        duration.num_minutes() * WeekConfigRepository::hour_px() as i64 / 60
    }
    #[inline]
    pub fn duration(span: &i64) -> Duration {
        Duration::minutes(span * 60 / WeekConfigRepository::hour_px() as i64) // TODO e.g. each 15 min
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Weekday::{Fri, Mon, Thu, Tue, Wed};

    #[test]
    fn test_days_in_week() {
        // default config
        WeekConfigRepository::set_default_config().unwrap();
        assert_eq!(
            WeekConfig::days_in_week(&NaiveDate::from_ymd_opt(2023, 03, 21).unwrap()),
            (19..=25).map(|d| NaiveDate::from_ymd_opt(2023, 03, d).unwrap()).collect::<Vec<_>>(),
        );

        // edited config
        WeekConfigRepository::set_display_weekdays(&vec![Mon, Tue, Wed, Thu, Fri]).unwrap();
        assert_eq!(
            WeekConfig::days_in_week(&NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()),
            NaiveDate::from_ymd_opt(2019, 12, 30).unwrap().iter_days().take(5).collect::<Vec<_>>(),
        );
    }

    #[test]
    fn test_hours_in_day() {
        // default config
        WeekConfigRepository::set_default_config().unwrap();
        assert_eq!(
            WeekConfig::hours_in_day(),
            (0..24).map(|h| NaiveTime::from_hms_opt(h, 0, 0).unwrap()).collect::<Vec<_>>()
        );

        // edited config
        WeekConfigRepository::set_display_hours(9..=18).unwrap();
        assert_eq!(
            WeekConfig::hours_in_day(),
            (9..=18).map(|h| NaiveTime::from_hms_opt(h, 0, 0).unwrap()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_row_col() {
        //default config
        WeekConfigRepository::set_default_config().unwrap();
        assert_eq!(WeekConfig::height(), 24);
        assert_eq!(WeekConfig::cols(), 7);
    }
}
