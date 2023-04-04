use super::view::{CalendarProps, CalendarViewProps};
use crate::event::Event;
use chrono::{Duration, NaiveDate, NaiveDateTime};
use std::{collections::BTreeMap, ops::RangeBounds};

pub struct Calendar {
    pub scale: Scale,
    pub inducing: NaiveDate,
    pub events: BTreeMap<NaiveDateTime, Event>,
}
impl From<Calendar> for CalendarProps {
    fn from(value: Calendar) -> Self {
        CalendarViewProps::from(value).calendar_props
    }
}
impl From<Calendar> for CalendarViewProps {
    fn from(value: Calendar) -> Self {
        let now = crate::supply::now().into();
        let Calendar { inducing, events, scale } = value;
        let calendar_props = CalendarProps { now, inducing, events };
        Self { scale, calendar_props }
    }
}

impl From<CalendarViewProps> for Calendar {
    fn from(value: CalendarViewProps) -> Self {
        let CalendarViewProps { scale, calendar_props } = value;
        let CalendarProps { inducing, events, .. } = calendar_props;
        Self { events, inducing, scale }
    }
}
impl Calendar {
    // TODO trait ? (with some arguments for performance)
    pub fn to_props(&self) -> CalendarViewProps {
        let Calendar { scale, inducing, .. } = self;
        let (scale, inducing) = (scale.clone(), inducing.clone()); // TODO better clone solution...?
        let limited = self.events.range(scale.bound(&inducing));
        let events = limited.map(|(t, e)| (t.clone(), e.clone())).collect();
        Calendar { scale, inducing, events }.into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Scale {
    Year,
    Month,
    Week,
    Day,
}
impl Default for Scale {
    fn default() -> Self {
        Self::Week
    }
}
impl Scale {
    pub fn prev(&self, &inducing: &NaiveDate) -> NaiveDate {
        match self {
            Self::Year => todo!("`Duration::years` is not implemented?"),
            Self::Month => todo!("`Duration::months` is not implemented?"),
            Self::Week => inducing - Duration::weeks(1),
            Self::Day => inducing - Duration::days(1),
        }
    }
    pub fn next(&self, &inducing: &NaiveDate) -> NaiveDate {
        match self {
            Self::Year => todo!("`Duration::years` is not implemented?"),
            Self::Month => todo!("`Duration::months` is not implemented?"),
            Self::Week => inducing + Duration::weeks(1),
            Self::Day => inducing + Duration::days(1),
        }
    }

    // TODO more precise (use Config data)
    pub fn bound(&self, &inducing: &NaiveDate) -> impl RangeBounds<NaiveDateTime> {
        let (prev, next) = (self.prev(&inducing).and_hms_opt(0, 0, 0), self.next(&inducing).and_hms_opt(23, 59, 59));
        prev.expect("exist hms 00:00:00")..=next.expect("exist hms 23:59:59")
    }
}
