use yew::prelude::*;

use crate::component::{calendar::view::Calendar, header::view::Header};

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Header />
                <Calendar/>
            </>
        }
    }
}
