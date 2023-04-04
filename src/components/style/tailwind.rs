use once_cell::sync::Lazy;
use yew::prelude::*;

#[inline]
pub fn row_start(row: &usize) -> Classes {
    classes!(format!("row-start-[{}]", row + 1)) // row start from 1
}
#[inline]
pub fn col_start(col: &usize) -> Classes {
    classes!(format!("col-start-[{}]", col + 1)) // col start from 1
}
// #[inline]
// pub fn rowcol_start((row, col): &(usize, usize)) -> Classes {
//     classes!(row_start(row), col_start(col))
// }
// #[inline]
// pub fn row_span(cols: &usize) -> Classes {
//     classes!(format!("row-span-{}", cols))
// }
// #[inline]
// pub fn col_span(cols: &usize) -> Classes {
//     classes!(format!("col-span-{}", cols))
// }

#[inline]
pub fn top_px(px: &u32) -> Classes {
    classes!(format!("top-[{}px]", px))
}
#[inline]
pub fn h_px(px: &u32) -> Classes {
    classes!(format!("h-[{}px]", px))
}
#[inline]
pub fn col_top((col, top): &(usize, u32)) -> Classes {
    classes!(col_start(col), top_px(top))
}

#[inline]
pub fn top(p: &u32) -> Classes {
    classes!(format!("top-{}", p))
}
#[inline]
pub fn bottom(p: &u32) -> Classes {
    classes!(format!("bottom-{}", p))
}
#[inline]
pub fn left(p: &u32) -> Classes {
    classes!(format!("left-{}", p))
}
#[inline]
pub fn right(p: &u32) -> Classes {
    classes!(format!("right-{}", p))
}

#[inline]
pub fn z(p: &u32) -> Classes {
    classes!(format!("z-{}", p))
}

pub const H_FULL: Lazy<Classes> = Lazy::new(|| classes!("h-full"));
pub const W_FULL: Lazy<Classes> = Lazy::new(|| classes!("w-full"));
pub const HW_FULL: Lazy<Classes> = Lazy::new(|| classes!(H_FULL.clone(), W_FULL.clone()));
