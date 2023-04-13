use super::state::Scale;
use chrono::{DateTime, Local, NaiveDate};
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
}
impl CalendarProps {
    pub fn with_scale(self, scale: Scale) -> CalendarViewProps {
        CalendarViewProps { scale, calendar_props: self }
    }

    pub fn focus(self, date: NaiveDate) -> Self {
        let inducing = date;
        Self { inducing, ..self }
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
