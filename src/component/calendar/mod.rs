pub mod props;
pub mod view;
pub mod week;

use chrono::{DateTime, Local, NaiveDate, NaiveDateTime};
// ref: https://qiita.com/alivelime/items/8ae9d63af8963bb8961d#%E3%83%A2%E3%82%B8%E3%83%A5%E3%83%BC%E3%83%AB%E9%83%A8%E5%88%86
pub fn now() -> DateTime<Local> {
    Local::now()
}
pub fn naive_now() -> NaiveDateTime {
    now().naive_local()
}
pub fn naive_today() -> NaiveDate {
    now().date_naive()
}
