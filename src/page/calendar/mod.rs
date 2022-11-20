pub mod day;
pub mod week;

use chrono::{Local, NaiveDate, NaiveDateTime};
use yew::prelude::*;

use self::week::START_WEEKDAY;

type Event = String; // TODO make Event struct

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
    events: Vec<Event>,
}

impl Component for Calendar {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let (scale, events) = (Scale::Week, Vec::new());
        let now = Local::now().timestamp_millis();
        let day = NaiveDateTime::from_timestamp_millis(now).unwrap().date();
        let start_day = day.week(START_WEEKDAY).first_day(); // TODO other scale
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
