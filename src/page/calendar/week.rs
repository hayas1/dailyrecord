use super::Calendar;
use chrono::{Datelike, Days, Weekday};
use yew::prelude::*;

pub const START_WEEKDAY: Weekday = Weekday::Sun;

impl Calendar {
    pub fn view_week(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let week = self.start_day.week(START_WEEKDAY);
        let days = self
            .start_day
            .iter_days()
            .take_while(|d| week.days().contains(d));
        let (weekdays, tabs): (Vec<_>, Vec<_>) = days.map(|d| (d.weekday(), d)).unzip();

        html! {
            <div>
                <p>{ format!("{:?}", self.scale) }</p>
                <table>
                    <thead><tr>{ weekdays.iter().map(|wd| html!{<td>{wd}</td>}).collect::<Html>() }</tr></thead>
                    <thead><tr>{ tabs.iter().map(|dt| html!{<td>{dt}</td>}).collect::<Html>() }</tr></thead>
                    <tbody><tr>{ tabs.iter().map(|dt| html!{<td>{
                        self.events
                        .range(
                            dt.and_hms_opt(0, 0, 0).unwrap()
                                ..dt.checked_add_days(Days::new(1))
                                    .unwrap()
                                    .and_hms_opt(0, 0, 0)
                                    .unwrap(),
                        )
                        .map(|(_t, ev)| html! {<p>{ev.title.to_string()}</p>})
                        .collect::<Html>()
                    }</td>}).collect::<Html>() }</tr></tbody>
                </table>
            </div>
        }
    }
}
