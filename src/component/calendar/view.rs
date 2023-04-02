use super::state::Scale;
use crate::event::Event;
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
