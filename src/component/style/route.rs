use yew::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq, Properties, Clone, Debug)]
pub struct HrefLinkProps<T: Routable + PartialEq> {
    pub to: T,
    #[prop_or_default]
    pub class: Classes,
    pub children: Children,
}

#[function_component(HrefLink)]
/// FIXME should not use this component, but use `yew_router::prelude::Link` .
/// However, it seems to not support struct component...? (did not re-render)
/// Further investigation is needed
///
/// versions: yew 0.20.0, yew_router 0.17.0
pub fn href_link<T: Routable + PartialEq>(props: &HrefLinkProps<T>) -> Html {
    let HrefLinkProps { to, class, children } = props;
    html! {
        <a href={ to.to_path() } class={classes!(class.clone())}>
            { for children.iter() }
        </a>
    }
}
