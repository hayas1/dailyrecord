pub mod week;

use std::collections::BTreeMap;

use crate::{
    component::calendar::week::view::WeekProps,
    event::{plan::Plan, Event},
};
use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, Weekday};
use yew::prelude::*;

// ref: https://qiita.com/alivelime/items/8ae9d63af8963bb8961d#%E3%83%A2%E3%82%B8%E3%83%A5%E3%83%BC%E3%83%AB%E9%83%A8%E5%88%86
pub fn now() -> DateTime<Local> {
    Local::now()
}
pub fn naive_now() -> NaiveDateTime {
    now().naive_local()
}
pub fn naive_today() -> NaiveDate {
    now().date_naive()
}

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
        let start_day = day.week(Weekday::Sun).first_day(); // TODO other scale
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
        ); // TODO from storage layer
        Self { scale, start_day, events }
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
            Scale::Week => {
                let props = WeekProps { now: now(), start_day: self.start_day, events: self.events.clone() };
                html! {<week::view::Week ..props />}
            }
            Scale::Day => todo!(),
        }
    }
}
