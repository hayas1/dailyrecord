use super::Config;
use crate::{
    component::{calendar::props::CalendarProps, style},
    event::Event,
};

use chrono::{Datelike, Duration, Weekday};
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
            format!("grid-rows-[70px,auto]"),
            format!("grid-cols-[minmax(35px,70px),repeat({},minmax(70px,1fr))]", Config::cols() - 1),
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
    let header = classes!("sticky", style::top(&0), style::z(&10), style::CAL_HEADER.clone(), header_border);
    html! {
        <>
        // header leftmost space
        <div class={classes!("absolute", style::col_start(&0), style::row_start(&0), header.clone(), style::left(&0), style::z(&20), "text-xl")}>
            <style::Centering>
                <div class={classes!(style::TEXT_CAL_HEADER.clone(), "font-bold")}>{ now.format("%m") }</div>
            </style::Centering>
        </div>
        {
            // header weekday and date
            days.iter().map(|&nd| html!{
                <div class={classes!("absolute", style::col_start(&Config::col(&nd.weekday()).unwrap()), style::row_start(&0), header.clone(), "text-sm")}>
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
    // TODO refactor
    let CalendarProps { now, inducing, .. } = props;
    let (hours, days) = (Config::hours_in_day(), Config::days_in_week(inducing));
    let bg = classes!("bg-white", "dark:bg-slate-800");
    let bga = bg.clone().into_iter().map(|c| format!("{}/50", c)).collect::<Classes>();
    let border = classes!("border-r", "border-slate-100", "dark:border-slate-200/5", "w-full");
    let text = classes!("text-xs", "text-right", "font-medium");
    let side_bar = classes!("sticky", style::left(&0), style::z(&10));
    let side_now = classes!(side_bar.clone(), text.clone(), bg.clone(), "border-r", "border-pink-500", "text-pink-500");
    html! {
        <>
        <div class={classes!("relative", style::col_start(&0), style::row_start(&1), side_bar.clone(), text.clone(), "text-slate-400")}>
            {
                hours.iter().map(|&nt| html!{
                    // leftmost %H:%M text
                    <div class={classes!("absolute", border.clone(), bga.clone(), style::right(&0), style::top_px(&Config::top(&nt).expect("should render")), style::h_px(&Config::span(&Duration::hours(1))))}>
                        <div class={classes!("-mt-2")}>{ nt.format("%H:00") }</div>
                    </div>
                }).collect::<Html>()
            }
            // now frame
            if let Some(now_top) = Config::top(&now.naive_local().time()) {
                <div class={classes!("absolute", style::right(&0), style::top_px(&now_top))}>
                    <div class={classes!(side_now.clone())}>{ now.naive_local().format("%H:%M").to_string() }</div>
                </div>
            }
        </div>
        {
            days.iter().map(|&nd| html!{
                <div class={classes!("relative", style::col_start(&Config::col(&nd.weekday()).expect("should render")), style::row_start(&1))}>
                    {
                        hours.iter().map(|&nt| html!{
                        // base frame
                        <div class={classes!("absolute", style::top_px(&Config::top(&nt).expect("should render")), style::h_px(&Config::span(&Duration::hours(1))), "border-b", border.clone())}>
                        </div>
                        }).collect::<Html>()
                    }
                </div>
            }).collect::<Html>()
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
    let col = Config::col(&event.plan.start.weekday()).expect("should render");
    let top = Config::top(&event.plan.start.time()).expect("should render");
    let span = Config::span(&event.plan.duration);
    let time_str = format!(
        "{}~{}",
        event.plan.start.naive_local().format("%H:%M"),
        (event.plan.start.naive_local() + event.plan.duration).format("%H:%M"),
    );
    let outline = classes!("border", "border-blue-700/10", "dark:border-sky-500", "rounded-lg", "mx-1", "h-full",);
    let bg = classes!("bg-blue-400/20", "dark:bg-sky-600/50");
    html! {
        <div class={classes!("relative", style::col_start(&col), style::row_start(&1))}>
            <div class={classes!("absolute", style::top_px(&top), style::h_px(&span), "w-full")}>
                <div class={classes!(bg, outline, "p-1", "text-xs", "truncate", "hover:text-clip")}>
                    <span class="text-blue-600 dark:text-sky-100">{ time_str }</span>
                    <br/>
                    <span class="font-medium text-blue-600 dark:text-sky-100">{ &event.title }</span>
                    <br/>
                    <span class="text-xs text-blue-400">{ &event.description }</span>
                </div>
            </div>
        </div>
    }
}
