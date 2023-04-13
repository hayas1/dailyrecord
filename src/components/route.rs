use super::app::{App, AppProps};
use super::calendar::state::Scale;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/calendar/*")]
    Calendar,
    #[at("/export")]
    Export,
    #[at("/analytics")]
    Analytics,
    #[at("/settings")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound,
}
impl std::fmt::Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Home => write!(f, "Home"),
            Self::Calendar => write!(f, "Calendar"),
            Self::Export => write!(f, "Export"),
            Self::Analytics => write!(f, "Analytics"),
            Self::Settings => write!(f, "Settings"),
            Self::NotFound => write!(f, "Not Found"),
        }
    }
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
    let scale = Scale::Week;
    let inducing = crate::supply::naive_today().week(chrono::Weekday::Sun).first_day(); // TODO other scale

    let calendar = super::calendar::state::Calendar { scale, inducing };
    let props = match routes {
        Route::Home | Route::Calendar => AppProps { content: super::app::Content::Calendar(calendar.to_props()) },
        Route::Export => AppProps { content: super::app::Content::Export() },
        Route::Analytics => AppProps { content: super::app::Content::Analytics() },
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
