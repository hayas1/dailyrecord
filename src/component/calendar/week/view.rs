use super::Config;
use crate::{component::calendar::naive_today, event::Event};

use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveDateTime, Timelike};
use std::collections::BTreeMap;
use yew::prelude::*;

pub struct Week {}

#[derive(Properties, PartialEq, Clone)]
pub struct WeekProps {
    pub now: DateTime<Local>,
    pub start_day: NaiveDate,
    pub events: BTreeMap<NaiveDateTime, Event>,
}

impl Component for Week {
    type Message = ();
    type Properties = WeekProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            // base: https://tailwindcss.com/docs/overflow#scrolling-in-all-directions
            <div class="relative rounded-xl overflow-hidden bg-slate-400/25 dark:bg-slate-800/75">
                <div class="absolute inset-0 bg-grid-slate-100 dark:bg-grid-slate-700/25"></div>
                <div class="relative rounded-xl overflow-auto">
                    <div class="mx-4 shadow-xl overflow-hidden bg-white dark:bg-slate-800">
                        <div class={ format!("overflow-scroll h-screen grid grid-cols-[70px,repeat(7,minmax(0,1fr))] grid-rows-[auto,repeat({},50px)]", Config::rows()) }>
                            <CalendarHeader ..ctx.props().clone()/>
                            <CalendarFrame ..ctx.props().clone()/>
                            <CalendarEvents ..ctx.props().clone()/>
                        </div>
                    </div>
                </div>
                <div class="absolute inset-0 pointer-events-none rounded-xl border border-black/5 dark:border-white/5"></div>
            </div>
        }
    }
}

#[function_component(CalendarHeader)]
fn calendar_header(props: &WeekProps) -> Html {
    let WeekProps { now, .. } = props;
    let days = Config::days_in_week(naive_today());
    html! {
        <>
        // header leftmost space
        <div class="row-start-[1] col-start-[1] sticky z-10 top-0 bg-clip-padding bg-slate-100 dark:bg-slate-800 border-b border-slate-200 dark:border-black/10">
            <div class="pt-2 text-xl text-center text-slate-900 dark:text-slate-200">{ now.format("%m") }</div>
        </div>
        {
            // header weekday and date
            days.iter().enumerate().map(|(w, nd)| html!{
                <div class={ format!("row-start-[1] col-start-[{}] sticky z-10 top-0 bg-clip-padding bg-slate-100 dark:bg-slate-800 border-b border-slate-200 dark:border-black/10 text-sm py-2 text-center text-slate-900 dark:text-slate-200", w+2) }>
                    <div>{ nd.weekday() }</div>
                    <div class={ format!("{}", if &now.date_naive() == nd {"rounded-full bg-red-500 text-slate-50"} else {""} )}>{ nd.day() }</div>
                </div>
            }).collect::<Html>()
        }
        </>
    }
}

#[function_component(CalendarFrame)]
fn calendar_frame(props: &WeekProps) -> Html {
    let WeekProps { now, .. } = props;
    let days = Config::days_in_week(naive_today());
    let range = Config::hour_in_day();
    html! {
        range.map(|h| html!{
            <>
            // leftmost %H:%M text
            <div class={ format!("row-start-[{}] col-start-[1] sticky left-0 border-r border-slate-100 dark:border-slate-200/5 bg-white dark:bg-slate-800 text-xs p-1 text-right text-slate-400 font-medium", h+2) }>
                { format!("{}:00", h) } // TODO use NaiveDateTime, but NaiveDateTime cannot deal with 24:00
            </div>
            {
                // base frame
                days.iter().enumerate().map(|(w, _nd)| html!{
                    <div class={ format!("row-start-[{}] col-start-[{}] border-b border-r border-slate-100 dark:border-slate-200/5 bg-white dark:bg-slate-800", h+2, w+2) }></div>
                }).collect::<Html>()
            }
            // now frame
            <div class={ format!("row-start-[{}] col-start-[1] sticky left-0 border-r border-red-500 text-xs p-1 text-right text-red-500 bg-white dark:bg-slate-800 font-medium", now.naive_local().hour() + 1) }>
                { now.naive_local().format("%H:%M").to_string() }
            </div>
            </>
        }).collect::<Html>()
    }
}

#[function_component(CalendarEvents)]
fn calendar_events(props: &WeekProps) -> Html {
    let WeekProps { events, .. } = props;
    html! {
        events.iter().map(|(_nt, e)| view_event(&e)).collect::<Html>()
    }
}

pub fn view_event(event: &Event) -> Html {
    // TODO refactor
    let row = event.plan.start.naive_local().hour() + 1;
    let col = event.plan.start.weekday().num_days_from_sunday() + 2;
    let span = event.plan.duration.num_hours();
    let time_str = format!(
        "{}~{}",
        event.plan.start.naive_local().format("%H:%M"),
        (event.plan.start.naive_local() + event.plan.duration).format("%H:%M")
    );
    html! {
        <div class={ format!("row-start-[{}] col-start-{} row-span-{} bg-blue-400/20 dark:bg-sky-600/50 border border-blue-700/10 dark:border-sky-500 rounded-lg m-1 p-1 flex flex-col", row, col, span) }>
            <span class="text-xs text-blue-600 dark:text-sky-100">{ time_str }</span>
            <span class="text-xs font-medium text-blue-600 dark:text-sky-100">{ &event.title }</span>
            // <span class="text-xs text-blue-600 dark:text-sky-100">{ &event.description }</span>
        </div>
    }
}
