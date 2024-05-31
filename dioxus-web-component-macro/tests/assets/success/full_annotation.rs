use std::convert::Infallible;

use dioxus::prelude::*;
use dioxus_web_component::web_component;
use wasm_bindgen::JsValue;

fn main() {}

#[web_component]
fn MyWebComponent(
    #[attribute(name= "attr1", option = false, initial = String::new(), parse = |value| Some(value.to_string()))]
    attr1: String,
    #[attribute(name = "attr-option", option = true, initial = None, parse = |value| Some(value.to_string()))]
    attr_option: Option<String>,
    #[property(name = "plop", readonly)] prop: Option<String>,
    #[property(
        initial = MyProp(true),
        try_into_js = |prop| {
            let js_value = if prop.0 {
                JsValue::TRUE
            } else {
                JsValue::FALSE
            };
            Ok::<_, Infallible>(js_value)
        },
        try_from_js= |value| Ok::<_, Infallible>(MyProp(value.is_truthy())),
    )]
    prop2: MyProp,
    #[event(name = "event", no_bubble = false, no_cancel = false)] event: EventHandler<i64>,
    #[event(name = "snake-evt", no_bubble = false, no_cancel = false)] on_snake_evt: EventHandler<
        bool,
    >,
) -> Element {
    None
}

#[derive(Clone, PartialEq)]
struct MyProp(bool);
