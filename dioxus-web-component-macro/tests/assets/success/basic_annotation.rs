use dioxus::prelude::*;
use dioxus_web_component::web_component;

fn main() {
}

#[web_component]
fn MyWebComponent(
    #[attribute] attr1: String,
    #[attribute] attr_option: Option<String>,
    #[event] event: EventHandler<i64>,
    #[event] on_snake_evt: EventHandler<bool>,
) -> Element {
    None
}