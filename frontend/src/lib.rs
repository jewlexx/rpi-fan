use yew::prelude::*;
use yew_router::Switch;

mod index;

use index::Index;

pub enum FanMsg {
    Toggle,
}

#[derive(Switch, Clone, Copy, Debug)]
pub enum Routes {
    #[to = "/"]
    Index,
}

#[function_component]
fn App() -> Html {
    html! { <Index /> }
}
