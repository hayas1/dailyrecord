use super::Config;
use crate::{
    components::{
        calendar::{control::SCROLL_ELEMENT_ID, view::CalendarProps},
        events::view::{ExpandEvent, ExpandEventProps},
        style,
    },
    repository::event::EventRepository,
};
use chrono::{Datelike, Duration, Weekday};
use yew::prelude::*;

#[function_component(Week)]
pub(crate) fn week(props: &CalendarProps) -> Html {
    html! {
        <WeekCalendar ..props.clone()/>
    }
}

#[function_component(WeekCalendar)]
fn week_calendar(props: &CalendarProps) -> Html {
    let grid_rows_cols = classes!(
        format!("grid-rows-[70px,auto]"),
        format!("grid-cols-[minmax(35px,70px),repeat({},minmax(70px,1fr))]", Config::cols() - 1),
    );
    let pps = props.clone().with_scale(super::super::state::Scale::Week);
    use_effect(move || super::super::control::week_initial_scroll(pps)); // TODO first render only
    html! {
        // base: https://tailwindcss.com/docs/overflow#scrolling-in-all-directions
        <div class={classes!("relative", "rounded-xl", "overflow-hidden", "bg-slate-400/25", "dark:bg-slate-800/75")}>
            <div class={classes!("absolute", "inset-0", "bg-grid-slate-100", "dark:bg-grid-slate-700/25")}></div>
            <div class={classes!("relative", "rounded-xl", "overflow-auto")}>
                <div class={classes!("mx-4", "shadow-xl", "overflow-hidden", "bg-white", "dark:bg-slate-800")}>
                    <div id={ SCROLL_ELEMENT_ID } class={classes!("overflow-scroll", "grid", grid_rows_cols, style::MAIN_HEIGHT.clone())}>
                        <WeekCalendarHeader ..props.clone()/>
                        <WeekCalendarLeftSide ..props.clone()/>
                        <WeekCalendarMainframe ..props.clone()/>
                    </div>
                </div>
            </div>
            <div class={classes!("absolute", "inset-0", "pointer-events-none", "rounded-xl", "border", "border-black/5", "dark:border-white/5")}></div>
        </div>
    }
}

#[function_component(WeekCalendarHeader)]
fn week_calendar_header(props: &CalendarProps) -> Html {
    let CalendarProps { now, inducing, .. } = props;
    let days = Config::days_in_week(inducing);
    let header_border = classes!("border-b", "border-slate-200", "dark:border-black/10");
    let header = classes!("sticky", style::top(&0), style::z(&10), style::CAL_HEADER.clone(), header_border);
    html! {
        <>
        // header leftmost space
        <div class={classes!("absolute", style::col_start(&0), style::row_start(&0), header.clone(), style::left(&0), style::z(&20))}>
            <div class={classes!("relative", style::HW_FULL.clone())}>
                <div class={classes!("absolute", style::HW_FULL.clone())}>
                    <style::CenterTop class={classes!(style::TEXT_CAL_HEADER.clone(), "text-xs", "pt-2", "pr-1")}>
                        <div>{ inducing.format("%Y") }</div>
                    </style::CenterTop>
                </div>
                <div class={classes!("absolute", style::HW_FULL.clone())}>
                    <style::Centering class={classes!(style::TEXT_CAL_HEADER.clone(), "text-xl", "text-center", "font-bold")}>
                        <div>{ inducing.format("%m") }</div>
                    </style::Centering>
                </div>
                <div class={classes!("absolute", style::HW_FULL.clone())}>
                    <style::RightBottom class={classes!("text-xs", "text-slate-400", "pb-1")}>
                        <div>{ now.format("%z") }</div>
                    </style::RightBottom>
                </div>
            </div>
        </div>
        {
            // header weekday and date
            days.iter().map(|&nd| html!{
                <div class={classes!("absolute", style::col_start(&Config::col(&nd.weekday()).unwrap()), style::row_start(&0), header.clone(), "text-sm")}>
                    <WeekCalendarHeaderDate ..CalendarProps { now: now.clone(), inducing: nd }/>
                </div>
            }).collect::<Html>()
        }
        </>
    }
}

#[function_component(WeekCalendarHeaderDate)]
fn week_calendar_header_date(props: &CalendarProps) -> Html {
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

#[function_component(WeekCalendarLeftSide)]
fn week_calendar_left_side(props: &CalendarProps) -> Html {
    let CalendarProps { now, .. } = props;
    let hours = Config::hours_in_day();
    let bg = style::BG_MAINFRAME.clone();
    let bga = bg.clone().into_iter().map(|c| format!("{}/50", c)).collect::<Classes>();
    let border = classes!("w-full", "border-r", style::BORDER_MAINFRAME.clone());
    let text = classes!("text-xs", "text-right", "font-medium");
    let side_bar = classes!("sticky", style::left(&0), style::z(&10));
    let side_now = classes!(side_bar.clone(), text.clone(), bg.clone(), "border-r", "border-pink-500", "text-pink-500");
    html! {
        <div class={classes!("relative", style::col_start(&0), style::row_start(&1), side_bar.clone(), text.clone(), "text-slate-400")}>
            {
                hours.iter().map(|&nt| html!{
                    // leftmost %H:%M text
                    <div class={classes!("absolute", border.clone(), bga.clone(), style::right(&0), style::top_px(&Config::top(&nt).expect("should be rendered")), style::h_px(&Config::span(&Duration::hours(1))))}>
                        <div>{ nt.format("%H:00") }</div>
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
    }
}

#[function_component(WeekCalendarMainframe)]
fn week_calendar_mainframe(props: &CalendarProps) -> Html {
    let CalendarProps { inducing, .. } = props;
    let (hours, days) = (Config::hours_in_day(), Config::days_in_week(inducing));
    let border = classes!("w-full", "border-r", "border-b", style::BORDER_MAINFRAME.clone());
    html! {
        {
            days.iter().map(|&nd| html!{
                <div class={classes!("relative", style::col_start(&Config::col(&nd.weekday()).expect("should be rendered")), style::row_start(&1))}>
                    {
                        hours.iter().map(|&nt| html!{
                            // base frame
                            <div class={classes!("absolute", style::top_px(&Config::top(&nt).expect("should be rendered")), style::h_px(&Config::span(&Duration::hours(1))),  border.clone())}></div>
                        }).collect::<Html>()
                    }
                    <WeekCalendarEvents ..props.clone().focus(nd) />
                </div>
            }).collect::<Html>()
        }
    }
}

#[function_component(WeekCalendarEvents)]
fn week_calendar_events(props: &CalendarProps) -> Html {
    let CalendarProps { inducing, .. } = props;
    let events = EventRepository::search(inducing).expect("should access");
    html! {
        events.iter().map(|(_nt, e)| {
            let event = e.clone();
            let (top, span) = (Config::top(&event.plan.start.time()).expect("should be rendered"), Config::span(&event.plan.duration));
            html! {
                <div class={classes!("relative")}>
                    <div class={classes!("absolute", style::top_px(&top), style::h_px(&span), "w-full")}>
                        <ExpandEvent ..ExpandEventProps{event} />
                    </div>
                </div>
            }
        }).collect::<Html>()
    }
}
