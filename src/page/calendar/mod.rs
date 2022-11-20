pub mod day;
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
        let start = Local::now() + Duration::days(2);
        events.insert(
            start.naive_local(),
            Event::new(
                "title".to_string(),
                "description".to_string(),
                None,
                Plan::new(start, Duration::hours(1), false),
            ),
        ); // TODO from local storage
        Self {
            scale,
            start_day,
            events,
        }
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
            Scale::Month => todo!(),
            Scale::Week => self.view_week(ctx),
            Scale::Day => self.view_day(ctx),
        }
    }
}
