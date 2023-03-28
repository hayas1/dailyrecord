use std::collections::BTreeMap;

use crate::{
    component::calendar::props::CalendarProps,
    event::{plan::Plan, Event},
};
use chrono::{Duration, Local, NaiveDate, NaiveDateTime, Weekday};
use yew::prelude::*;

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
}

pub struct Calendar {
    pub scale: Scale,
    pub start_day: NaiveDate,
    pub events: BTreeMap<NaiveDateTime, Event>,
}

impl Component for Calendar {
    type Message = Msg;
    type Properties = CalendarProps;

    fn create(ctx: &Context<Self>) -> Self {
        let CalendarProps { inducing, events, .. } = ctx.props().clone();
        let scale = Scale::Week;
        let start_day = inducing;
        Self { scale, start_day, events }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeScale(scale) => self.scale = scale,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let props = CalendarProps { now: super::now(), inducing: self.start_day, events: self.events.clone() };
        match self.scale {
            Scale::Year => todo!(),
            Scale::Month => todo!(),
            Scale::Week => {
                html! {<super::week::view::Week ..ctx.props().clone() />}
            }
            Scale::Day => todo!(),
        }
    }
}
