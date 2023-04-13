use crate::domain::entity::episode::episode::Episode;

use yew::prelude::*;

#[derive(PartialEq, Properties, Clone, Debug)]
pub struct ExpandEpisodeProps {
    pub episode: Episode,
}

#[function_component(ExpandEpisode)]
pub fn expand_episode(props: &ExpandEpisodeProps) -> Html {
    let ExpandEpisodeProps { episode } = props;
    let time_str = format!(
        "{}~{}",
        episode.plan.start.naive_local().format("%H:%M"),
        (episode.plan.start.naive_local() + episode.plan.duration).format("%H:%M"),
    );
    let outline = classes!("border", "border-blue-700/10", "dark:border-sky-500", "rounded-lg", "mx-1", "h-full",);
    let bg = classes!("bg-blue-400/20", "dark:bg-sky-600/50");
    html! {
        <div class={classes!(bg, outline, "p-1", "text-xs", "truncate", "hover:text-clip")}>
            <span class={classes!("text-blue-600", "dark:text-sky-100")}>{ time_str }</span>
            <br/>
            <span class={classes!("font-medium", "text-blue-600", "dark:text-sky-100")}>{ &episode.title }</span>
            <br/>
            <span class={classes!("text-xs", "text-blue-400")}>{ &episode.description }</span>
        </div>
    }
}
