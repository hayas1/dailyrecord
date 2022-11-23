use super::Calendar;
use chrono::{Datelike, Days, NaiveDate, Weekday};
use yew::prelude::*;

pub const START_WEEKDAY: Weekday = Weekday::Sun;

impl Calendar {
    pub fn view_week(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let days = self.days_in_week();

        html! {
            <div>
                <button class="button">{ format!("{:?}", self.scale) }</button>
                <div class="container">
                    <div class="notification is-text">
                        { self.start_day.month() }{"月"}
                    </div>
                </div>
                <div class="table-container">
                    <table class="table is-fullwidth">
                        <thead>
                            <tr>{
                                days.iter().map(|nd| html!{
                                    <td class="has-text-centered">
                                        <p>{nd.weekday()}</p>
                                        <p>{nd.day()}</p>
                                    </td>
                                }).collect::<Html>()
                            }</tr>
                        </thead>
                        <tbody><tr>{ days.iter().map(|wd| self.view_weekday(ctx, wd)).collect::<Html>() }</tr></tbody>
                    </table>
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
            <td>
                <p>{ events.map(|(_t, e)| e.title.to_string()).collect::<Html>() }</p>
            </td>
        }
    }
}
