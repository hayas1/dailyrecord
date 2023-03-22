use crate::event::Event;

use chrono::{DateTime, Local, NaiveDate, NaiveDateTime};
use std::collections::BTreeMap;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Default)]
pub struct CalendarProps {
    pub now: DateTime<Local>,
    pub inducing: NaiveDate,
    pub events: BTreeMap<NaiveDateTime, Event>,
}
