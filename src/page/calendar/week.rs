use super::Calendar;
use chrono::{Datelike, Days, Duration, NaiveDate, NaiveTime, Timelike, Weekday};
use yew::prelude::*;

pub const START_WEEKDAY: Weekday = Weekday::Sun;

impl Calendar {
    pub fn view_week(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let days = self.days_in_week();
        html! {
            <div>
                <button class="button">{ format!("{:?}", self.scale) }</button>
                <div class="tile is-ancestor">
                    <div class="tile is-horizontal">
                        <div class="tile is-parent is-1">
                            <div class="tile is-child box has-text-centered">
                                <p>{ self.start_day.month() }{"月"}</p>
                                <p>{"第3週"}</p>
                            </div>
                        </div>
                        <div class="tile is-parent">{
                            days.iter().map(|nd| html!{
                                <div class="tile is-child box has-text-centered">
                                    <p>{nd.weekday()}</p>
                                    <p>{nd.day()}</p>
                                </div>
                            }).collect::<Html>()
                        }</div>
                    </div>
                </div>
                <div class="tile is-ancestor">
                    <div class="tile is-parent is-1">
                        <div class="tile is-child box has-text-centered">{
                            (0..=23).map(|h| html!{
                                <p>{ NaiveTime::from_hms_opt(h, 0, 0).unwrap().hour() }{ ":00" }</p>
                            }).collect::<Html>()
                        }</div>
                    </div>
                    <div class="tile is-horizontal">
                        <div class="tile is-parent">{
                            days.iter().map(|wd| self.view_weekday(ctx, wd)).collect::<Html>()
                        }</div>
                    </div>
                </div>
            </div>
        }
    }

    pub fn days_in_week(&self) -> Vec<NaiveDate> {
        let week = self.start_day.week(START_WEEKDAY);
        self.start_day
            .iter_days()
            .take_while(|d| week.days().contains(d))
            .collect()
    }

    pub fn view_weekday(&self, _ctx: &Context<Self>, day: &NaiveDate) -> Html {
        let day_0h0m0s = day.and_hms_opt(0, 0, 0).unwrap();
        let next_0h0m0s = day_0h0m0s + Days::new(1);
        let events = self.events.range(day_0h0m0s..next_0h0m0s);
        html! {
            <div class="tile is-child box">{
                events.map(|(_t, e)| html!{
                    <p>{e.title.to_string()}</p>
                }).collect::<Html>()
            }</div>
        }
    }
}
