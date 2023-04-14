// TODO logic

use chrono::Weekday;
use once_cell::sync::Lazy;
use std::{cell::RefCell, ops::RangeInclusive, sync::Mutex};

use crate::domain::entity::config::WeekConfig;

#[derive(Default)]
pub struct WeekConfigRepository(pub(crate) WeekConfig); // TODO Rc<RefCell<>>?
static CONFIG: Lazy<Mutex<RefCell<WeekConfigRepository>>> =
    Lazy::new(|| Mutex::new(RefCell::new(WeekConfigRepository(Default::default())))); // TODO from storage layer

impl WeekConfigRepository {
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
            (*config.borrow_mut()).0.display_weekdays = weekdays.clone().into();
            // TODO save to storage layer
            Ok(())
        } else {
            anyhow::bail!("cannot get config lock")
        }
    }
    #[inline]
    pub fn display_weekdays() -> Vec<Weekday> {
        if let Ok(config) = CONFIG.lock() {
            config.borrow().0.display_weekdays.clone()
        } else {
            panic!("cannot get config lock")
        }
    }
    #[inline]
    pub fn num_display_days() -> usize {
        if let Ok(config) = CONFIG.lock() {
            config.borrow().0.display_weekdays.len()
        } else {
            panic!("cannot get config lock")
        }
    }
    #[inline]
    pub fn first_weekday() -> Weekday {
        if let Ok(config) = CONFIG.lock() {
            let first = config.borrow().0.display_weekdays.first().cloned();
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
            (*config.borrow_mut()).0.display_hours = hours.clone();
            // TODO save to storage layer
            Ok(())
        } else {
            anyhow::bail!("cannot get config lock")
        }
    }
    #[inline]
    pub fn display_hour_range() -> RangeInclusive<u32> {
        if let Ok(config) = CONFIG.lock() {
            config.borrow().0.display_hours.clone()
        } else {
            panic!("cannot get config lock")
        }
    }
    #[inline]
    pub fn set_hour_px(px: &u32) -> anyhow::Result<()> {
        if let Ok(config) = CONFIG.lock() {
            (*config.borrow_mut()).0.hour_px = px.clone();
            // TODO save to storage layer
            Ok(())
        } else {
            anyhow::bail!("cannot get config lock")
        }
    }
    #[inline]
    pub fn hour_px() -> u32 {
        if let Ok(config) = CONFIG.lock() {
            config.borrow().0.hour_px
        } else {
            panic!("cannot get config lock")
        }
    }
}
