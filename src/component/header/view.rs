use crate::component::{
    app::{AppMsg, Step},
    route::Route,
    style,
};
use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::prelude::*;

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
    html! {
        // base: https://v1.tailwindcss.com/components/navigation#responsive-header
        <nav class={classes!("flex", "items-center", "justify-between", "flex-wrap", "bg-slate-600", "dark:bg-slate-900", "py-3", "px-6", style::HEADER_HEIGHT.clone())}>
            <div class="flex items-center flex-shrink-0 text-white mr-6">
                <button class="pt-2 mx-2" onclick={move |_| prev_callback.clone().emit(AppMsg::Step(Step::Prev))}>
                    <Icon icon_id={IconId::HeroiconsSolidChevronLeft} height="1rem"/>
                </button>
                <Link<Route> to={Route::Index}>
                    <style::Logo/>
                </Link<Route>>
                <button class="pt-2 mx-2" onclick={move |_| next_callback.clone().emit(AppMsg::Step(Step::Next))}>
                    <Icon icon_id={IconId::HeroiconsSolidChevronRight} height="1rem"/>
                </button>
                </div>
            <div class="block lg:hidden">
                <button class="flex items-center px-3 py-2 border rounded text-slate-200 border-slate-500 hover:text-white hover:border-white">
                <svg class="fill-current h-3 w-3" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><title>{"Menu"}</title><path d="M0 3h20v2H0V3zm0 6h20v2H0V9zm0 6h20v2H0v-2z"/></svg>
                </button>
            </div>
            <div class="w-full block flex-grow lg:flex lg:items-center lg:w-auto">
                <div class="text-sm lg:flex-grow">
                    <div class="block mt-4 lg:inline-block lg:mt-0 text-slate-200 hover:text-white mr-4">
                        <Link<Route> to={Route::Export}>{"Export"}</Link<Route>>
                    </div>
                    <div class="block mt-4 lg:inline-block lg:mt-0 text-slate-200 hover:text-white mr-4">
                        <Link<Route> to={Route::Calendar}>{"Select"}</Link<Route>>
                    </div>
                </div>
                <div>
                    <div class="inline-block text-sm px-4 py-2 leading-none border rounded text-white border-white hover:border-transparent hover:text-slate-500 hover:bg-white mt-4 lg:mt-0">
                        <Link<Route> to={Route::Settings}>{"Settings"}</Link<Route>>
                    </div>
                </div>
            </div>
        </nav>
    }
}
