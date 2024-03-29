use super::calendar::view::CalendarViewProps;
use crate::components::{
    calendar::view::Calendar,
    header::view::{Header, HeaderProps},
};
use yew::prelude::*;

pub enum App {
    Calendar(super::calendar::state::Calendar),
    Export(),
    Analytics(),
    Settings(),
    NotFound,
}
#[derive(PartialEq, Clone, Debug)]
pub enum Content {
    Calendar(CalendarViewProps),
    Export(),
    Analytics(),
    Settings(),
    NotFound,
}

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct AppProps {
    pub content: Content,
}

pub enum AppMsg {
    Step(Step),
}
pub enum Step {
    Next,
    Prev,
}
impl std::fmt::Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Next => write!(f, "Next"),
            Self::Prev => write!(f, "Prev"),
        }
    }
}

impl Component for App {
    type Message = AppMsg;
    type Properties = AppProps;

    fn create(ctx: &Context<Self>) -> Self {
        // TODO implement as From trait ?
        match ctx.props().clone().content {
            Content::Calendar(cal) => Self::Calendar(cal.into()),
            Content::Export() => Self::Export(),
            Content::Analytics() => Self::Analytics(),
            Content::Settings() => Self::Settings(),
            Content::NotFound => Self::NotFound,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match (msg, self) {
            (Self::Message::Step(Step::Prev), Self::Calendar(cal)) => cal.inducing = cal.scale.prev(&cal.inducing),
            (Self::Message::Step(Step::Next), Self::Calendar(cal)) => cal.inducing = cal.scale.next(&cal.inducing),
            _ => (),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let callback = ctx.link().callback(move |msg| msg);
        let header = html! { <Header ..HeaderProps { step: callback.clone() }/> };
        let content = match self {
            Self::Calendar(cal) => html! { <Calendar ..cal.to_props() /> },
            Self::Export() => html! { <div>{"Coming soon!"}</div> },
            Self::Analytics() => html! { <div>{"Unimplemented yet..."}</div> },
            Self::Settings() => html! { <div>{"Coming soon!"}</div> },
            Self::NotFound => html! { <h1>{ "404 Not Found" }</h1> },
        };
        html! {
            <>
                { header }
                { content }
            </>
        }
    }
}
