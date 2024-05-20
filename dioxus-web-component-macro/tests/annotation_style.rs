use dioxus::prelude::*;
use dioxus_web_component::web_component;

#[test]
fn just_need_to_compile() {}

#[web_component]
fn MyWebComponent(
    attr1: String,
    attr_option: Option<String>,
    event: EventHandler<i64>,
    on_snake_evt: EventHandler<bool>,
) -> Element {
    None
}

#[web_component]
fn MyWebComponent2(
    #[attribute] attr1: String,
    #[attribute] attr_option: Option<String>,
    #[event] event: EventHandler<i64>,
    #[event] on_snake_evt: EventHandler<bool>,
) -> Element {
    None
}

#[web_component]
fn MyWebComponent3(
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
