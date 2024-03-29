use once_cell::sync::Lazy;
use yew::prelude::*;

pub const HEADER_HEIGHT: Lazy<Classes> = Lazy::new(|| classes!("h-70px"));
pub const MAIN_HEIGHT: Lazy<Classes> = Lazy::new(|| classes!("h-[calc(100vh-55px)]"));

pub const BG_CAL_HEADER: Lazy<Classes> = Lazy::new(|| classes!("bg-slate-100", "dark:bg-slate-800"));
pub const TEXT_CAL_HEADER: Lazy<Classes> = Lazy::new(|| classes!("text-slate-900", "dark:text-slate-200"));
pub const CAL_HEADER: Lazy<Classes> = Lazy::new(|| classes!(BG_CAL_HEADER.clone(), TEXT_CAL_HEADER.clone()));

pub const BG_MAINFRAME: Lazy<Classes> = Lazy::new(|| classes!("bg-white", "dark:bg-slate-800"));
pub const BORDER_MAINFRAME: Lazy<Classes> = Lazy::new(|| classes!("border-slate-100", "dark:border-slate-200/5"));

pub const BG_MUTED: Lazy<Classes> = Lazy::new(|| classes!("bg-slate-200", "dark:bg-slate-700"));
pub const TEXT_MUTED: Lazy<Classes> = Lazy::new(|| classes!("text-slate-400"));
