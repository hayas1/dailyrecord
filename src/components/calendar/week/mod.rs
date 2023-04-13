pub mod view;

use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Weekday};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, ops::RangeInclusive, sync::Mutex};

// do not use const, but static (impact of yew thread?)
static CONFIG: Lazy<Mutex<RefCell<Config>>> = Lazy::new(|| Mutex::new(RefCell::new(Config::default()))); // TODO from storage layer

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    hour_px: u32,
    display_weekdays: Vec<Weekday>, // TODO BTreeSet ?
    display_hours: RangeInclusive<u32>,
}
impl Default for Config {
    fn default() -> Self {
        let hour_px = 60;
        let display_weekdays = std::iter::successors(Some(Weekday::Sun), |&wd| Some(wd.succ())).take(7).collect();
        let display_hours = 0..=23;
        Self { hour_px, display_weekdays, display_hours }
    }
}
impl Config {
    #[inline]
    pub fn set_default_config() -> anyhow::Result<()> {
        if let Ok(config) = CONFIG.lock() {
            *config.borrow_mut() = Default::default();
            // TODO save to storage layer
            Ok(())
        } else {
            anyhow::bail!("cannot get config lock")
        }
    }
    #[inline]
    pub fn set_display_weekdays(weekdays: &[Weekday]) -> anyhow::Result<()> {
        anyhow::ensure!(
            weekdays.windows(2).all(|w| (w[0] as u32) < (w[1] as u32)),
            "display weekdays should be sorted" //TODO allow cycle, like [Wed, Fri, Mon]
        );

        if let Ok(config) = CONFIG.lock() {
            (*config.borrow_mut()).display_weekdays = weekdays.clone().into();
            // TODO save to storage layer
            Ok(())
        } else {
            anyhow::bail!("cannot get config lock")
        }
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
    pub fn set_display_hours(hours: RangeInclusive<u32>) -> anyhow::Result<()> {
        let (start, end) = hours.clone().into_inner();
        anyhow::ensure!(start <= end);
        anyhow::ensure!(0 <= start);
        anyhow::ensure!(end <= 23);

        if let Ok(config) = CONFIG.lock() {
            (*config.borrow_mut()).display_hours = hours.clone();
            // TODO save to storage layer
            Ok(())
        } else {
            anyhow::bail!("cannot get config lock")
        }
    }
    #[inline]
    pub fn display_hour_range() -> RangeInclusive<u32> {
        if let Ok(config) = CONFIG.lock() {
            config.borrow().display_hours.clone()
        } else {
            panic!("cannot get config lock")
        }
    }
    #[inline]
    pub fn set_hour_px(px: &u32) -> anyhow::Result<()> {
        if let Ok(config) = CONFIG.lock() {
            (*config.borrow_mut()).hour_px = px.clone();
            // TODO save to storage layer
            Ok(())
        } else {
            anyhow::bail!("cannot get config lock")
        }
    }
    #[inline]
    pub fn hour_px() -> u32 {
        if let Ok(config) = CONFIG.lock() {
            config.borrow().hour_px
        } else {
            panic!("cannot get config lock")
        }
    }
}

impl Config {
    #[inline] // TODO cache
    pub fn cols() -> usize {
        1 + Config::num_display_days() // col1: time legend, col2..: episode space
    }
    #[inline] // TODO cache
    pub fn height() -> u32 {
        let (start, end) = Config::display_hour_range().into_inner();
        let rows = (end - start) * Config::hour_px();
        rows
    }
    #[inline] // TODO!!! cache!!!
    pub fn col(weekday: &Weekday) -> Option<usize> {
        let col0 = Config::display_weekdays().iter().position(|wd| wd == weekday)?;
        Some(1 + col0) // +1 by time col
    }
    #[inline]
    pub fn top(time: &NaiveTime) -> Option<u32> {
        Config::display_hour_range().contains(&time.hour()).then(|| {
            let start = NaiveTime::from_hms_opt(Config::display_hour_range().start().clone(), 0, 0)
                .expect("display hours should be validated");
            (time.clone() - start).num_minutes() as u32 * Config::hour_px() / 60
        })
    }
    #[inline]
    pub fn pos(dt: &NaiveDateTime) -> Option<(usize, u32)> {
        match (Config::col(&dt.weekday()), Config::top(&dt.time())) {
            (Some(col), Some(top)) => Some((col, top)),
            _ => None,
        }
    }

    #[inline]
    pub fn hours_in_day() -> Vec<NaiveTime> {
        Config::display_hour_range()
            .map(|h| NaiveTime::from_hms_opt(h, 0, 0).expect("display hours should be validated"))
            .collect()
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

    #[inline]
    pub fn span(duration: &Duration) -> u32 {
        duration.num_minutes() as u32 * Config::hour_px() / 60
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
    fn test_hours_in_day() {
        // default config
        Config::set_default_config().unwrap();
        assert_eq!(
            Config::hours_in_day(),
            (0..24).map(|h| NaiveTime::from_hms_opt(h, 0, 0).unwrap()).collect::<Vec<_>>()
        );

        // edited config
        Config::set_display_hours(9..=18).unwrap();
        assert_eq!(
            Config::hours_in_day(),
            (9..=18).map(|h| NaiveTime::from_hms_opt(h, 0, 0).unwrap()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_row_col() {
        //default config
        Config::set_default_config().unwrap();
        assert_eq!(Config::height(), 24);
        assert_eq!(Config::cols(), 7);
    }
}
