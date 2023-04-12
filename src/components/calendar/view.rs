use super::state::Scale;
use crate::components::events::models::event::Event;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime};
use std::collections::BTreeMap;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct CalendarViewProps {
    pub scale: Scale,
    pub calendar_props: CalendarProps,
}
#[derive(Properties, PartialEq, Clone, Debug)]
pub struct CalendarProps {
    pub now: DateTime<Local>,
    pub inducing: NaiveDate,
    pub events: BTreeMap<NaiveDateTime, Event>,
}
impl CalendarProps {
    pub fn with_scale(self, scale: Scale) -> CalendarViewProps {
        CalendarViewProps { scale, calendar_props: self }
    }
    pub fn focus(self, date: NaiveDate) -> Self {
        let inducing = date;
        let (from, to) = (
            date.and_hms_opt(0, 0, 0).expect("exist hms 00:00:00"),
            date.and_hms_opt(23, 59, 59).expect("exist hms 23:59:59"),
        );
        let events = self.events.range(from..=to).map(|(t, e)| (t.clone(), e.clone())).collect();
        Self { inducing, events, ..self }
    }
}

#[function_component(Calendar)]
pub(crate) fn calendar(props: &CalendarViewProps) -> Html {
    let CalendarViewProps { scale, calendar_props } = props;
    match scale {
        Scale::Year => todo!(),
        Scale::Month => todo!(),
        Scale::Week => {
            html! {<super::week::view::Week ..calendar_props.clone() />}
        }
        Scale::Day => todo!(),
    }
}
