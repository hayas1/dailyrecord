pub mod day;
pub mod month;
pub mod week;

use std::collections::BTreeMap;

use crate::event::{plan::Plan, Event};
use chrono::{Duration, Local, NaiveDate, NaiveDateTime};
use yew::prelude::*;

use self::week::START_WEEKDAY;

pub enum Msg {
    ChangeScale(Scale),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Scale {
    Year,
    Month,
    Week,
    Day,
}

pub struct Calendar {
    scale: Scale,
    unit: Duration,
    start_day: NaiveDate,
    events: BTreeMap<NaiveDateTime, Event>,
}

impl Component for Calendar {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let (scale, mut events) = (Scale::Week, BTreeMap::new());
        let now = Local::now().timestamp_millis();
        let day = NaiveDateTime::from_timestamp_millis(now).unwrap().date();
        let start_day = day.week(START_WEEKDAY).first_day(); // TODO other scale
        let start = Local::now().date_naive().and_hms_opt(13, 0, 0).unwrap();
        events.insert(
            start,
            Event::new(
                "title".to_string(),
                "description".to_string(),
                None,
                Plan::new(start.and_local_timezone(Local).unwrap(), Duration::hours(1), false),
            ),
        );
        events.insert(
            start + Duration::hours(1),
            Event::new(
                "event2".to_string(),
                "event's description".to_string(),
                None,
                Plan::new(
                    start.and_local_timezone(Local).unwrap() + Duration::days(1) + Duration::hours(1),
                    Duration::hours(1),
                    false,
                ),
            ),
        ); // TODO from local storage
        Self { scale, unit: Duration::minutes(15), start_day, events }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeScale(scale) => self.scale = scale,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.scale {
            Scale::Year => todo!(),
            Scale::Month => self.view_month(ctx),
            Scale::Week => self.view_week(ctx),
            Scale::Day => self.view_day(ctx),
        }
    }
}