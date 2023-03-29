use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LogoProps {
    #[prop_or_default]
    pub class: Classes,
}
#[function_component(Logo)]
pub fn logo(_props: &LogoProps) -> Html {
    html! {
        <span class="font-semibold text-xl tracking-tight">{"Daily Record"}</span>
    }
}
