use dioxus::prelude::*;
use dioxus_web_component::web_component;

fn main() {
}

#[web_component]
fn MyWebComponent(
    #[attribute(name= "attr1", option = false, initial = String::new(), parse = |value| Some(value.to_string()))]
    attr1: String,
    #[attribute(name = "attr-option", option = true, initial = None, parse = |value| Some(value.to_string()))]
    attr_option: Option<String>,
    #[event(name = "event", no_bubble = false, no_cancel = false)] event: EventHandler<i64>,
    #[event(name = "snake-evt", no_bubble = false, no_cancel = false)] on_snake_evt: EventHandler<
        bool,
    >,
) -> Element {
    None
}
