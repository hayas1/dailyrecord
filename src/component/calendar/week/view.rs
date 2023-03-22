use super::Config;
use crate::{
    component::{calendar::props::CalendarProps, style},
    event::Event,
};

use chrono::{Datelike, NaiveTime, Timelike, Weekday};
use yew::prelude::*;

pub struct Week {}

impl Component for Week {
    type Message = ();
    type Properties = CalendarProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let grid_rows_cols = classes!(
            format!("grid-rows-[auto,repeat({},50px)]", Config::rows()),
            "grid-cols-[70px,repeat(7,minmax(0,1fr))]",
        );
        html! {
            // base: https://tailwindcss.com/docs/overflow#scrolling-in-all-directions
            <div class={classes!("relative", "rounded-xl", "overflow-hidden", "bg-slate-400/25", "dark:bg-slate-800/75")}>
                <div class={classes!("absolute", "inset-0", "bg-grid-slate-100", "dark:bg-grid-slate-700/25")}></div>
                <div class={classes!("relative", "rounded-xl", "overflow-auto")}>
                    <div class={classes!("mx-4", "shadow-xl", "overflow-hidden", "bg-white", "dark:bg-slate-800")}>
                        <div class={classes!("overflow-scroll", "h-screen", "grid", grid_rows_cols)}>
                            <CalendarHeader ..ctx.props().clone()/>
                            <CalendarFrame ..ctx.props().clone()/>
                            <CalendarEvents ..ctx.props().clone()/>
                        </div>
                    </div>
                </div>
                <div class={classes!("absolute", "inset-0", "pointer-events-none", "rounded-xl", "border", "border-black/5", "dark:border-white/5")}></div>
            </div>
        }
    }
}

#[function_component(CalendarHeader)]
fn calendar_header(props: &CalendarProps) -> Html {
    let CalendarProps { now, inducing, .. } = props;
    let days = Config::days_in_week(inducing);
    let header_border = classes!("border-b", "border-slate-200", "dark:border-black/10");
    let text = classes!("text-center", style::TEXT_CAL_HEADER.clone());
    let header = classes!("sticky", "z-10", "top-0", style::BG_CAL_HEADER.clone(), header_border, text);
    html! {
        <>
        // header leftmost space
        <div class={classes!(style::rowcol_start(&(0, 0)), header.clone(), "text-xl")}>
            <style::Centering>
                <div class={classes!(style::TEXT_CAL_HEADER.clone())}>{ now.format("%m") }</div>
            </style::Centering>
        </div>
        {
            // header weekday and date
            days.iter().map(|&nd| html!{
                <div class={classes!(style::rowcol_start(&(0, Config::col(&nd.weekday()).unwrap())), header.clone(), "text-sm")}>
                    <CalendarHeaderDate ..CalendarProps { now: now.clone(), inducing: nd, events: Default::default() }/>
                </div>
            }).collect::<Html>()
        }
        </>
    }
}

#[function_component(CalendarHeaderDate)]
fn calendar_header_date(props: &CalendarProps) -> Html {
    let CalendarProps { now, inducing, .. } = props;
    let mut text = classes!("text-center", "py-2");
    match inducing.weekday() {
        Weekday::Sun => text.push(classes!("text-rose-500")),
        Weekday::Sat => text.push(classes!("text-cyan-500")),
        _ => text.push(style::TEXT_CAL_HEADER.clone()),
    }
    let today;
    if &now.date_naive() == inducing {
        text.push(classes!("font-bold"));
        today = classes!("h-8", "w-8", "pb-0.5", "rounded-full", "bg-pink-500", "text-slate-50");
    } else {
        today = classes!("h-8", "w-8");
    }
    html! {
        <div class={classes!(text)}>
            <div>{ inducing.weekday() }</div>
            <style::Centering>
                <div class={classes!(today)}>
                <style::Centering>{ inducing.day() }</style::Centering>
                </div>
            </style::Centering>
        </div>
    }
}

#[function_component(CalendarFrame)]
fn calendar_frame(props: &CalendarProps) -> Html {
    let CalendarProps { now, inducing, .. } = props;
    let (hours, days) = (Config::hours_in_day(), Config::days_in_week(inducing));
    let border = classes!("border-r", "border-slate-100", "dark:border-slate-200/5");
    let bg = classes!("bg-white", "dark:bg-slate-800");
    let time = classes!(border, bg.clone());
    let text = classes!("text-xs", "text-right", "font-medium");
    let side_bar = classes!("sticky", "left-0");
    let side_now = classes!(side_bar.clone(), text.clone(), bg.clone(), "border-r", "border-pink-500", "text-pink-500");
    html! {
        <>
        {
            hours.iter().map(|&nt| html!{
                <>
                // leftmost %H:%M text
                <div class={classes!(style::rowcol_start(&(Config::row(&nt).expect("should display"), 0)), side_bar.clone(), time.clone(), text.clone(), "text-slate-400")}>
                    { nt.format("%H:00") }
                </div>
                {
                    // base frame
                    days.iter().map(|nd| html!{
                        <div class={classes!(style::rowcol_start(&(Config::rowcol(&nd.and_time(nt)).expect("should display"))), "border-b", time.clone())}></div>
                    }).collect::<Html>()
                }
                </>
            }).collect::<Html>()
        }
        // now frame
        if let Some(now_row) = Config::row(&now.naive_local().time()) {
            <div class={classes!(style::rowcol_start(&(now_row, 0)), side_now.clone())}>
                { now.naive_local().format("%H:%M").to_string() }
            </div>
        }
        </>
    }
}

#[function_component(CalendarEvents)]
fn calendar_events(props: &CalendarProps) -> Html {
    let CalendarProps { events, .. } = props;
    html! {
        events.iter().map(|(_nt, e)| view_event(&e)).collect::<Html>()
    }
}

pub fn view_event(event: &Event) -> Html {
    // TODO refactor
    let row = event.plan.start.naive_local().hour() + 2;
    let col = event.plan.start.weekday().num_days_from_sunday() + 2;
    let span = event.plan.duration.num_hours();
    let time_str = format!(
        "{}~{}",
        event.plan.start.naive_local().format("%H:%M"),
        (event.plan.start.naive_local() + event.plan.duration).format("%H:%M")
    );
    html! {
        <div class={ format!("row-start-[{}] col-start-[{}] row-span-{} bg-blue-400/20 dark:bg-sky-600/50 border border-blue-700/10 dark:border-sky-500 rounded-lg m-1 p-1 flex flex-col", row, col, span) }>
            <span class="text-xs text-blue-600 dark:text-sky-100">{ time_str }</span>
            <span class="text-xs font-medium text-blue-600 dark:text-sky-100">{ &event.title }</span>
            // <span class="text-xs text-blue-600 dark:text-sky-100">{ &event.description }</span>
        </div>
    }
}
