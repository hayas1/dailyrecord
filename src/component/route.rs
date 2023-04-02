use super::app::{App, AppProps};
use super::calendar::state::Scale;
use crate::event::{plan::Plan, Event};
use chrono::{Duration, Local, NaiveDateTime, Weekday};
use std::collections::BTreeMap;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Index,
    #[at("/home")]
    Home,
    #[at("/calendar/*")]
    Calendar,
    #[at("/export")]
    Export,
    #[at("/settings")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
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
    let calendar = super::calendar::state::Calendar { scale, inducing: start_day, events };
    let props = match routes {
        Route::Index | Route::Home | Route::Calendar => {
            AppProps { content: super::app::Content::Calendar(calendar.to_props()) }
        }
        Route::Export => AppProps { content: super::app::Content::Export() },
        Route::Settings => AppProps { content: super::app::Content::Settings() },
        Route::NotFound => AppProps { content: super::app::Content::NotFound },
    };
    html! { <App ..props /> }
}

#[function_component(Router)]
pub fn router() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}