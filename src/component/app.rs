use std::collections::BTreeMap;

use chrono::{Duration, Local, NaiveDateTime, Weekday};
use yew::prelude::*;

use crate::{
    component::{
        calendar::view::Calendar,
        header::{props::HeaderProps, view::Header},
    },
    event::{plan::Plan, Event},
};

use super::calendar::{props::CalendarProps, view::Scale};

pub enum Step {
    Next,
    Prev,
}
pub struct App {
    calendar: Calendar,
}

impl Component for App {
    type Message = Step;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // // edited config of hours
        // week::Config::set_display_hours(9..=18).unwrap();
        // // edited config of weekdays
        // week::Config::set_display_weekdays(&vec![
        //     chrono::Weekday::Mon,
        //     chrono::Weekday::Tue,
        //     chrono::Weekday::Wed,
        //     chrono::Weekday::Thu,
        //     chrono::Weekday::Fri,
        // ])
        // .unwrap();
        // assert_eq!(
        //     week::Config::days_in_week(&NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()),
        //     NaiveDate::from_ymd_opt(2019, 12, 30).unwrap().iter_days().take(5).collect::<Vec<_>>(),
        // );
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
                Plan::new(start.and_local_timezone(Local).unwrap(), Duration::hours(1) + Duration::minutes(30), false),
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
        Self { calendar: Calendar { scale, start_day, events } }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let inducing = match msg {
            Self::Message::Prev => self.calendar.scale.prev(&self.calendar.start_day),
            Self::Message::Next => self.calendar.scale.next(&self.calendar.start_day),
        };
        self.calendar.start_day = inducing;
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let callback = ctx.link().callback(move |msg| msg);
        let props = CalendarProps {
            now: crate::supply::now().into(),
            inducing: self.calendar.start_day,
            events: self.calendar.events.clone(),
        };
        html! {
            <>
                <Header ..HeaderProps { step: callback.clone() }/>
                <Calendar ..props/>
            </>
        }
    }
}
