use super::{super::Calendar, START_WEEKDAY};
use crate::event::Event;
use chrono::{DateTime, Datelike, Days, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Weekday};
use yew::prelude::*;

impl Calendar {
    pub fn view_week(&self, ctx: &Context<Self>) -> Html {
        let days = self.days_in_week();
        let (start, end) = (1, 22); // TODO (0, 23)
        let range = std::iter::successors(NaiveTime::from_hms_opt(start, 0, 0), |&nt| Some(nt + Duration::hours(1)))
            .take_while(|&nt| nt <= NaiveTime::from_hms_opt(end, 0, 0).unwrap())
            .collect::<Vec<_>>();
        let naive_now = Local::now().naive_local() - Duration::hours(5);
        // TODO refactor
        html! {
            <div>
                <button class="bg-gray-400 hover:bg-gray-300 text-white rounded px-4 py-2">{ format!("{:?}", self.scale) }</button>
                // base https://tailwindcss.com/docs/overflow#scrolling-in-all-directions
                <div class="relative rounded-xl overflow-hidden bg-slate-400/25 dark:bg-slate-800/75">
                    <div class="absolute inset-0 bg-grid-slate-100 dark:bg-grid-slate-700/25"></div>
                        <div class="relative rounded-xl overflow-auto">
                            <div class="mx-4 shadow-xl overflow-hidden bg-white dark:bg-slate-800">
                                <div class={ format!("overflow-scroll h-screen grid grid-cols-[70px,repeat(7,minmax(0,1fr))] grid-rows-[auto,repeat({},50px)]", end-start) }>
                                    // calender header
                                    <div class="row-start-[1] col-start-[1] sticky z-10 top-0 bg-clip-padding bg-slate-100 dark:bg-slate-800 border-b border-slate-200 dark:border-black/10"></div>
                                    {
                                        days.iter().enumerate().map(|(w, nd)| html!{
                                            <div class={ format!("row-start-[1] col-start-[{}] sticky z-10 top-0 bg-clip-padding bg-slate-100 dark:bg-slate-800 border-b border-slate-200 dark:border-black/10 text-sm py-2 text-center text-slate-900 dark:text-slate-200", w+2) }>
                                                <div>{ nd.weekday() }</div>
                                                <div class={ format!("{}", if &naive_now.date() == nd {"rounded-full bg-red-500 text-slate-200"} else {""} )}>{ nd.day() }</div>
                                            </div>
                                        }).collect::<Html>()
                                    }
                                    // calender frame
                                    {
                                        range.iter().enumerate().map(|(h, nt)| html!{
                                            <>
                                            <div class={ format!("row-start-[{}] col-start-[1] sticky left-0 border-r border-slate-100 dark:border-slate-200/5 bg-white dark:bg-slate-800 text-xs p-1 text-right text-slate-400 font-medium", h+2) }>
                                                { nt.format("%H:%M").to_string() }
                                            </div>
                                            {
                                                days.iter().enumerate().map(|(w, _nd)| html!{
                                                    <div class={ format!("row-start-[{}] col-start-[{}] border-b border-r border-slate-100 dark:border-slate-200/5 bg-white dark:bg-slate-800", h+2, w+2) }></div>
                                                }).collect::<Html>()
                                            }
                                            </>
                                        }).collect::<Html>()
                                    }
                                    // calendar now
                                    <div class={ format!("row-start-[{}] col-start-[1] sticky left-0 border-r border-red-500 text-xs p-1 text-right text-red-500 bg-white dark:bg-slate-800 font-medium", naive_now.hour() + 1) }>
                                        { naive_now.format("%H:%M").to_string() }
                                    </div>
                                    // calendar event
                                    {
                                        self.events.iter().map(|(_nt, e)| self.view_event(ctx, &e)).collect::<Html>()
                                    }
                                </div>
                            </div>
                        </div>
                    <div class="absolute inset-0 pointer-events-none rounded-xl border border-black/5 dark:border-white/5"></div>
                </div>
            </div>
        }
    }

    pub fn days_in_week(&self) -> Vec<NaiveDate> {
        let week = self.start_day.week(START_WEEKDAY);
        self.start_day.iter_days().take_while(|d| week.days().contains(d)).collect()
    }

    pub fn view_event(&self, _ctx: &Context<Self>, event: &Event) -> Html {
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

    pub fn view_unit(&self, _ctx: &Context<Self>, start: NaiveDateTime, unit: Duration) -> Html {
        let events = self.events.range(start..start + unit);
        html! {
            <div class="text-center">{
                events.map(|(_t, e)| html!{
                    {e.title.to_string()}
                }).collect::<Html>()
            }</div>
        }
    }
}
