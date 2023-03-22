pub mod view;

use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, Weekday};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, ops::RangeInclusive, sync::Mutex};

// do not use const, but static (impact of yew thread?)
static CONFIG: Lazy<Mutex<RefCell<Config>>> = Lazy::new(|| Mutex::new(RefCell::new(Config::default()))); // TODO from storage layer

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde(with = "crate::supply::serde_duration")]
    time_unit: Duration,
    display_weekdays: Vec<Weekday>,
}
impl Default for Config {
    fn default() -> Self {
        // TODO specified weekdays (e.g. exclude Sat and Sun), and validation
        let time_unit = Duration::minutes(60);
        let display_weekdays = std::iter::successors(Some(Weekday::Sun), |&wd| Some(wd.succ())).take(7).collect();
        Self { time_unit, display_weekdays }
    }
}
impl Config {
    #[inline]
    pub fn set_default_config() -> anyhow::Result<()> {
        if let Ok(config) = CONFIG.lock() {
            *config.borrow_mut() = Default::default();
            // TODO save to storage layer
            // Ok(())
        } else {
            anyhow::bail!("cannot get config lock");
        }
        Ok(())
    }
    #[inline]
    pub fn set_display_weekdays(weekdays: &[Weekday]) -> anyhow::Result<()> {
        if let Ok(config) = CONFIG.lock() {
            (*config.borrow_mut()).display_weekdays = weekdays.clone().into();
            // TODO save to storage layer
            // Ok(())
        } else {
            anyhow::bail!("cannot get config lock");
        }
        Ok(())
    }
    #[inline]
    pub fn display_weekdays() -> Vec<Weekday> {
        if let Ok(config) = CONFIG.lock() {
            config.borrow().display_weekdays.clone()
        } else {
            panic!("cannot get config lock")
        }
    }
    #[inline]
    pub fn num_display_days() -> usize {
        if let Ok(config) = CONFIG.lock() {
            config.borrow().display_weekdays.len()
        } else {
            panic!("cannot get config lock")
        }
    }
    #[inline]
    pub fn first_weekday() -> Weekday {
        if let Ok(config) = CONFIG.lock() {
            let first = config.borrow().display_weekdays.first().cloned();
            first.expect("display weekdays should be validated (at least one weekday)")
        } else {
            panic!("cannot get config lock")
        }
    }
    #[inline]
    pub fn set_time_unit(unit: &Duration) -> anyhow::Result<()> {
        if let Ok(config) = CONFIG.lock() {
            (*config.borrow_mut()).time_unit = unit.clone();
            // TODO save to storage layer
            Ok(())
        } else {
            anyhow::bail!("cannot get config lock")
        }
    }
    #[inline]
    pub fn time_unit() -> Duration {
        if let Ok(config) = CONFIG.lock() {
            config.borrow().time_unit
        } else {
            panic!("cannot get config lock")
        }
    }
    #[inline] // TODO cache
    pub fn rows() -> usize {
        // TODO Duration / Duration seem to be not supported in chrono v0.4
        let (start, end) = Config::hours_in_day().into_inner();
        let rows = Duration::hours((end - start) as i64).num_minutes() / Config::time_unit().num_minutes();
        1 + rows as usize // row1: date header, row2..: event space
    }
    #[inline] // TODO cache
    pub fn cols() -> usize {
        1 + Config::num_display_days() // col1: time legend, col2..: event space
    }
    #[inline]
    pub fn row(time: &NaiveTime) -> usize {
        // FIXME -> Option<usize> (see hours_in_day)
        // TODO Duration / Duration seem to be not supported in chrono v0.4
        let start = NaiveTime::from_hms_opt(*Config::hours_in_day().start() as u32, 0, 0).unwrap();
        let row0 = (*time - start).num_minutes() / Config::time_unit().num_minutes();
        1 + row0 as usize // +1 by header
    }
    #[inline]
    pub fn col(weekday: &Weekday) -> usize {
        // FIXME -> Option<usize> (see days_in_week)
        1 + weekday.num_days_from_sunday() as usize // and +1 by time col
    }
    #[inline]
    pub fn rowcol(dt: &NaiveDateTime) -> (usize, usize) {
        // FIXME -> Option<(usize, usize)>
        (Config::row(&dt.time()), Config::col(&dt.weekday()))
    }
}

impl Config {
    #[inline]
    pub fn hours_in_day() -> RangeInclusive<usize> {
        0..=23 // TODO chrono::NaiveTime cannot deal with 24:00:00
    }

    #[inline]
    pub fn days_in_week(day: &NaiveDate) -> Vec<NaiveDate> {
        let (weekdays, first) = (Config::display_weekdays(), Config::first_weekday());
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Weekday::{Fri, Mon, Thu, Tue, Wed};

    #[test]
    fn test_days_in_week() {
        // default config
        Config::set_default_config().unwrap();
        assert_eq!(
            Config::days_in_week(&NaiveDate::from_ymd_opt(2023, 03, 21).unwrap()),
            (19..=25).map(|d| NaiveDate::from_ymd_opt(2023, 03, d).unwrap()).collect::<Vec<_>>(),
        );

        // edited config
        Config::set_display_weekdays(&vec![Mon, Tue, Wed, Thu, Fri]).unwrap();
        assert_eq!(
            Config::days_in_week(&NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()),
            NaiveDate::from_ymd_opt(2019, 12, 30).unwrap().iter_days().take(5).collect::<Vec<_>>(),
        );
    }

    #[test]
    fn test_row_col() {
        //default config
        Config::set_default_config().unwrap();
        assert_eq!(Config::rows(), 24);
        assert_eq!(Config::cols(), 8);
    }
}
