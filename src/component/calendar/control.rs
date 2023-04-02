use crate::component::calendar::{
    view::{CalendarProps, CalendarViewProps},
    week,
};

pub const SCROLL_ELEMENT_ID: &str = "scroll-element-id"; // TODO unique
pub fn week_initial_scroll(props: CalendarViewProps) {
    let CalendarProps { now, .. } = props.calendar_props;
    if let Some(scroll) = gloo::utils::document().get_element_by_id(SCROLL_ELEMENT_ID) {
        if let Some(top) = week::Config::top(&now.time()) {
            // HACK this function should be called when first render, so this condition should be removed
            if scroll.scroll_top() == 0 {
                scroll.set_scroll_top(top as i32);
            }
        }
    }
}
