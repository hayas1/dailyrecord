use once_cell::sync::Lazy;
use yew::{classes, Classes};

pub const BG_CAL_HEADER: Lazy<Classes> = Lazy::new(|| classes!("bg-slate-100", "dark:bg-slate-900"));
pub const TEXT_CAL_HEADER: Lazy<Classes> = Lazy::new(|| classes!("text-slate-900", "dark:text-slate-200"));
pub const CAL_HEADER: Lazy<Classes> = Lazy::new(|| classes!((*BG_CAL_HEADER).clone(), (*TEXT_CAL_HEADER).clone()));

pub const BG_MAIN: Lazy<Classes> = Lazy::new(|| classes!("bg-white", "dark:bg-slate-800"));
pub const TEXT_MAIN: Lazy<Classes> = Lazy::new(|| classes!("text-slate-800", "dark:text-slate-300"));
pub const MAIN: Lazy<Classes> = Lazy::new(|| classes!((*BG_MAIN).clone(), (*TEXT_MAIN).clone()));

pub const BG_MUTED: Lazy<Classes> = Lazy::new(|| classes!("bg-slate-200", "dark:bg-slate-700"));
pub const TEXT_MUTED: Lazy<Classes> = Lazy::new(|| classes!("text-slate-400"));

#[inline]
pub fn row_start(row: &usize) -> Classes {
    classes!(format!("row-start-[{}]", row + 1)) // row start from 1
}
#[inline]
pub fn col_start(col: &usize) -> Classes {
    classes!(format!("col-start-[{}]", col + 1)) // col start from 1
}
#[inline]
pub fn rowcol_start((row, col): &(usize, usize)) -> Classes {
    classes!(row_start(row), col_start(col))
}
