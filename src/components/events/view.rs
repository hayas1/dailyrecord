use super::models::event::Event;
use yew::prelude::*;

#[derive(PartialEq, Properties, Clone, Debug)]
pub struct ExpandEventProps {
    pub event: Event,
}

#[function_component(ExpandEvent)]
pub fn expand_event(props: &ExpandEventProps) -> Html {
    let ExpandEventProps { event } = props;
    let time_str = format!(
        "{}~{}",
        event.plan.start.naive_local().format("%H:%M"),
        (event.plan.start.naive_local() + event.plan.duration).format("%H:%M"),
    );
    let outline = classes!("border", "border-blue-700/10", "dark:border-sky-500", "rounded-lg", "mx-1", "h-full",);
    let bg = classes!("bg-blue-400/20", "dark:bg-sky-600/50");
    html! {
        <div class={classes!(bg, outline, "p-1", "text-xs", "truncate", "hover:text-clip")}>
            <span class={classes!("text-blue-600", "dark:text-sky-100")}>{ time_str }</span>
            <br/>
            <span class={classes!("font-medium", "text-blue-600", "dark:text-sky-100")}>{ &event.title }</span>
            <br/>
            <span class={classes!("text-xs", "text-blue-400")}>{ &event.description }</span>
        </div>
    }
}
