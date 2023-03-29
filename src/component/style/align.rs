use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AlignProps {
    #[prop_or_default]
    pub class: Classes,
    pub children: Children,
}
#[function_component(Centering)]
pub fn centering(props: &AlignProps) -> Html {
    let AlignProps { class, children } = props;
    let center = classes!("flex", "items-center", "justify-center", super::HW_FULL.clone(), class.clone());
    html! {
        <div class={classes!(center.clone())}>
            { for children.iter() }
        </div>
    }
}

#[function_component(RightBottom)]
pub fn right_bottom(props: &AlignProps) -> Html {
    let AlignProps { class, children } = props;
    let center = classes!("flex", "items-end", "justify-end", super::HW_FULL.clone(), class.clone());
    html! {
        <div class={classes!(center.clone())}>
            { for children.iter() }
        </div>
    }
}

#[function_component(LeftTop)]
pub fn left_top(props: &AlignProps) -> Html {
    let AlignProps { class, children } = props;
    let center = classes!("flex", "items-start", "justify-start", super::HW_FULL.clone(), class.clone());
    html! {
        <div class={classes!(center.clone())}>
            { for children.iter() }
        </div>
    }
}
