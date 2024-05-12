use dioxus::prelude::*;
use dioxus_web_component::{web_component, DioxusWebComponent};

#[test]
fn should_work_without_attributes() {
    let attributes = MyWebComponentWebComponent::attributes();
    assert_eq!(attributes, &["attr1", "attr-option"]);
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

#[test]
fn should_work_with_marker_attributes() {
    let attributes = MyWebComponent2WebComponent::attributes();
    assert_eq!(attributes, &["attr1", "attr-option"]);
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

#[test]
fn should_work_with_full_attributes() {
    let attributes = MyWebComponent3WebComponent::attributes();
    assert_eq!(attributes, &["attr1", "attr-option"]);
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
