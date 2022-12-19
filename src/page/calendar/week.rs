use super::Calendar;
use chrono::{Datelike, Days, Duration, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Weekday};
use yew::prelude::*;

pub const START_WEEKDAY: Weekday = Weekday::Sun;

impl Calendar {
    pub fn view_week(&self, ctx: &Context<Self>) -> Html {
        let days = self.days_in_week();
        let (start, end) = (9, 18);
        let unit = Duration::hours(1);
        html! {
            <div>
                <button class="button">{ format!("{:?}", self.scale) }</button>
                <div class="table-responsive">
                    <table class="table" style="table-layout:fixed;">
                        <thead>
                            <tr>
                                <th class="text-center" style="width:75px;">
                                    <p>{ self.start_day.month() }{"月"}</p>
                                    <p>{"第3週"}</p>
                                </th>
                                {
                                    days.iter().map(|nd| html!{
                                        <th class="text-center" style="width:100px;">
                                            <p>{nd.weekday()}</p>
                                            <p>{nd.day()}</p>
                                        </th>
                                    }).collect::<Html>()
                                }
                            </tr>
                        </thead>
                        <tbody>
                            {
                                (start..=end).map(|h| html!{
                                    <tr>
                                        <td class="text-center" style="height:1rem;">
                                            { NaiveTime::from_hms_opt(h, 0, 0).unwrap().hour() }{ ":00" }
                                        </td>
                                        {
                                            days.iter().map(|nd| html!{
                                                <td class="text-center">
                                                    { self.view_unit(ctx, nd.and_hms_opt(h, 0, 0).unwrap(), unit) }
                                                </td>
                                            }).collect::<Html>()
                                        }
                                    </tr>
                                }).collect::<Html>()
                            }
                        </tbody>
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
