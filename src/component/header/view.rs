use super::state::Step;
use crate::component::style;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq, Clone, Default)]
pub struct HeaderProps {
    pub step: Callback<Step>,
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
                <button class="pt-2 mx-2" onclick={move |_| prev_callback.clone().emit(Step::Prev)}>
                    <Icon icon_id={IconId::HeroiconsSolidChevronLeft} height="1rem"/>
                </button>
                <button> // TODO onclick return home
                    <style::Logo/>
                </button>
                <button class="pt-2 mx-2" onclick={move |_| next_callback.clone().emit(Step::Next)}>
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
                <a href="#responsive-header" class="block mt-4 lg:inline-block lg:mt-0 text-slate-200 hover:text-white mr-4">
                    {"Export"}
                </a>
                <a href="#responsive-header" class="block mt-4 lg:inline-block lg:mt-0 text-slate-200 hover:text-white mr-4">
                    {"Select"}
                </a>
                <a href="#responsive-header" class="block mt-4 lg:inline-block lg:mt-0 text-slate-200 hover:text-white">
                    {"Information"}
                </a>
                </div>
                <div>
                <a href="#" class="inline-block text-sm px-4 py-2 leading-none border rounded text-white border-white hover:border-transparent hover:text-slate-500 hover:bg-white mt-4 lg:mt-0">{"Settings"}</a>
                </div>
            </div>
        </nav>
    }
}
