use dioxus::prelude::*;
use dioxus_web_component::web_component;

fn main() {
}

#[web_component]
fn MyWebComponent(
    attr1: String,
    attr_option: Option<String>,
    event: EventHandler<i64>,
    on_snake_evt: EventHandler<bool>,
) -> Element {
    None
}