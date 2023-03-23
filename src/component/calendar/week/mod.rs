pub mod view;

use crate::supply::arith_duration;
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Weekday};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, ops::RangeInclusive, sync::Mutex};

// do not use const, but static (impact of yew thread?)
static CONFIG: Lazy<Mutex<RefCell<Config>>> = Lazy::new(|| Mutex::new(RefCell::new(Config::default()))); // TODO from storage layer

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde(with = "crate::supply::serde_duration")]
    time_unit: Duration,
    display_weekdays: Vec<Weekday>, // TODO BTreeSet ?
    display_hours: RangeInclusive<NaiveTime>,
}
impl Default for Config {
    fn default() -> Self {
        // TODO specified weekdays (e.g. exclude Sat and Sun), and validation
        let time_unit = Duration::minutes(15);
        let display_weekdays = std::iter::successors(Some(Weekday::Sun), |&wd| Some(wd.succ())).take(7).collect();
        let display_hours = NaiveTime::from_hms_opt(0, 0, 0).unwrap()..=NaiveTime::from_hms_opt(23, 0, 0).unwrap();
        Self { time_unit, display_weekdays, display_hours }
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
    pub fn set_display_hours(hours: RangeInclusive<NaiveTime>) -> anyhow::Result<()> {
        let (start, end) = hours.clone().into_inner();
        anyhow::ensure!(start <= end);
        // TODO how to get hour only NaiveTime
        anyhow::ensure!(
            start - Duration::hours(start.hour() as i64) == NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            "start NaiveTime should have only hour part",
        );
        anyhow::ensure!(
            end - Duration::hours(end.hour() as i64) == NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            "end NaiveTime should have only hour part",
        );

        if let Ok(config) = CONFIG.lock() {
            (*config.borrow_mut()).display_hours = hours.clone();
            // TODO save to storage layer
            Ok(())
        } else {
            anyhow::bail!("cannot get config lock")
        }
    }
    #[inline]
    pub fn display_hour_range() -> RangeInclusive<NaiveTime> {
        if let Ok(config) = CONFIG.lock() {
            config.borrow().display_hours.clone()
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
        let (start, end) = Config::display_hour_range().into_inner();
        let rows = arith_duration::div(&(end - start), &Config::time_unit());
        1 + rows as usize // row1: date header, row2..: event space
    }
    #[inline] // TODO cache
    pub fn cols() -> usize {
        1 + Config::num_display_days() // col1: time legend, col2..: event space
    }
    #[inline]
    pub fn row(time: &NaiveTime) -> Option<usize> {
        Config::display_hour_range().contains(time).then(|| {
            let &start = Config::display_hour_range().start();
            let row0 = arith_duration::div(&(time.clone() - start), &Config::time_unit());
            1 + row0 as usize // +1 by header
        })
    }
    #[inline] // TODO!!! cache!!!
    pub fn col(weekday: &Weekday) -> Option<usize> {
        let col0 = Config::display_weekdays().iter().position(|wd| wd == weekday)?;
        Some(1 + col0) // +1 by time col
    }
    #[inline]
    pub fn rowcol(dt: &NaiveDateTime) -> Option<(usize, usize)> {
        match (Config::row(&dt.time()), Config::col(&dt.weekday())) {
            (Some(row), Some(col)) => Some((row, col)),
            _ => None,
        }
    }
}

impl Config {
    #[inline]
    pub fn hours_in_day() -> Vec<NaiveTime> {
        // TODO NaiveTime cannot iterate by duration ?
        (Config::display_hour_range().start().hour()..)
            .map(|h| NaiveTime::from_hms_opt(h, 0, 0))
            .take_while(|oph| oph.and_then(|h| Config::display_hour_range().contains(&h).then(|| ())).is_some())
            .map(|oph| oph.expect("display hours should be validated"))
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
    pub fn hour_row_span() -> usize {
        arith_duration::div(&Duration::hours(1), &Config::time_unit()) as usize
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
        Config::set_display_hours(
            NaiveTime::from_hms_opt(9, 0, 0).unwrap()..=NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
        )
        .unwrap();
        assert_eq!(
            Config::hours_in_day(),
            (9..=18).map(|h| NaiveTime::from_hms_opt(h, 0, 0).unwrap()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_row_col() {
        //default config
        Config::set_default_config().unwrap();
        assert_eq!(Config::rows(), 24);
        assert_eq!(Config::cols(), 7);
    }
}
