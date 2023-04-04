use crate::components::{
    app::{AppMsg, Step},
    route::Route,
    style,
};
use yew::prelude::*;
use yew_icons::{Icon, IconId};
// use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct HeaderProps {
    pub step: Callback<AppMsg>,
}

#[function_component(Header)]
pub(crate) fn header(props: &HeaderProps) -> Html {
    html! {
        <Navbar ..HeaderProps { step: props.step.clone() }/>
    }
}

#[function_component(Navbar)]
fn navbar(props: &HeaderProps) -> Html {
    let prev_callback = props.step.clone();
    let next_callback = props.step.clone();
    let chevron_class = classes!("pt-2", "mx-2", "text-slate-300", "hover:text-white");
    let item_classes = classes!("inline-block", "mt-2", "mx-4",);
    let icon_hover =
        classes!("rounded-full", "text-white", "hover:border-transparent", "hover:text-slate-500", "hover:bg-white");
    let icon_hover_anim =
        classes!("transition", "ease-in-out", "hover:-translate-y-1", "hover:scale-150", "duration-300");
    let icon_classes = classes!(icon_hover.clone(), icon_hover_anim.clone());
    html! {
        // base: https://v1.tailwindcss.com/components/navigation#responsive-header
        <nav class={classes!("flex", "items-center", "justify-between", "flex-wrap", "text-white", "bg-slate-600", "dark:bg-slate-900", "py-3", "px-6", style::HEADER_HEIGHT.clone())}>
            <div class={classes!("flex", "items-center", "flex-shrink-0", "mr-6")}>
                <button class={classes!(chevron_class.clone(), icon_hover_anim.clone())}
                    onclick={move |_| prev_callback.clone().emit(AppMsg::Step(Step::Prev))}
                    title={Step::Prev.to_string()}>
                    <Icon icon_id={IconId::HeroiconsSolidChevronLeft} height="1rem"/>
                </button>
                <style::HrefLink<Route> to={Route::Home} title={Route::Home.to_string()}>
                    <style::Logo/>
                </style::HrefLink<Route>>
                <button class={classes!(chevron_class.clone(), icon_hover_anim.clone())}
                    onclick={move |_| next_callback.clone().emit(AppMsg::Step(Step::Next))}
                    title={Step::Next.to_string()}>
                    <Icon icon_id={IconId::HeroiconsSolidChevronRight} height="1rem"/>
                </button>
            </div>
            <div class={classes!("block", "flex", "flex-grow", "flex-center", "items-center", "w-auto")}>
                <div class={classes!("flex-grow")}>
                    <div class={classes!(item_classes.clone(), icon_classes.clone())} title={Route::Export.to_string()}>
                        <style::HrefLink<Route> to={Route::Export}>
                            <Icon icon_id={IconId::HeroiconsSolidRocketLaunch}/>
                        </style::HrefLink<Route>>
                    </div>
                    <div class={classes!(item_classes.clone(), icon_classes.clone())} title={Route::Calendar.to_string()}>
                        <style::HrefLink<Route> to={Route::Calendar}>
                            <Icon icon_id={IconId::HeroiconsSolidCalendarDays}/>
                        </style::HrefLink<Route>>
                    </div>
                    <div class={classes!(item_classes.clone(), icon_classes.clone())} title={Route::Analytics.to_string()}>
                        <style::HrefLink<Route> to={Route::Analytics}>
                            <Icon icon_id={IconId::HeroiconsSolidCubeTransparent}/>
                        </style::HrefLink<Route>>
                    </div>
                </div>
                <div>
                    <div class={classes!(item_classes.clone(), icon_classes.clone())} title={Route::Settings.to_string()}>
                        <style::HrefLink<Route> to={Route::Settings}>
                            <Icon icon_id={IconId::HeroiconsSolidCog8Tooth}/>
                        </style::HrefLink<Route>>
                    </div>
                </div>
            </div>
        </nav>
    }
}
