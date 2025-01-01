#![allow(missing_docs)]
use std::convert::Infallible;

use dioxus::prelude::*;
use dioxus_web_component::{web_component, InjectedStyle};
use wasm_bindgen::JsValue;

#[test]
fn just_need_to_compile() {}

#[web_component(tag = "plop-wc")]
fn MyWebComponent(
    attr1: String,
    attr_option: Option<String>,
    event: EventHandler<i64>,
    on_snake_evt: EventHandler<bool>,
) -> Element {
    rsx!()
}

#[web_component(style = InjectedStyle::css(":host {display:flex;}"))]
fn MyWebComponent2(
    #[attribute] attr1: String,
    #[attribute] attr_option: Option<String>,
    // #[property] prop: MyProp,
    #[property] prop: String,
    #[event] event: EventHandler<i64>,
    #[event] on_snake_evt: EventHandler<bool>,
) -> Element {
    rsx!()
}

#[web_component(no_typescript)]
fn MyWebComponent3(
    #[attribute(name= "attr1", option = false, initial = String::new(), parse = |value| Some(value.to_string()))]
    attr1: String,
    #[attribute(name = "attr-option", option = true, initial = None, parse = |value| Some(value.to_string()))]
    attr_option: Option<String>,
    #[property(readonly)] prop: Option<String>,
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
    rsx!()
}

#[derive(Clone, PartialEq)]
struct MyProp(bool);
