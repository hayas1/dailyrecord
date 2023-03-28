use yew::prelude::*;

use crate::component::app::Step;

#[derive(Properties, PartialEq, Clone, Default)]
pub struct HeaderProps {
    pub step: Callback<Step>,
}
