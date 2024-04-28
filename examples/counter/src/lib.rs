#![doc = include_str!("../README.md")]
#![allow(clippy::multiple_crate_versions)]

use dioxus::prelude::*;
use dioxus_web_component::{
    custom_event_handler, register_dioxus_web_component, Context, CustomEventOptions,
    DioxusWebComponent,
};
use wasm_bindgen::prelude::*;

/// Install (register) the web component
///
/// # Errors
///
/// Registering the web-component may fail
#[wasm_bindgen(start)]
pub fn register() -> Result<(), JsValue> {
    register_dioxus_web_component::<CounterWebComponent>("plop-counter");

    Ok(())
}

/// The Dioxus component
#[component]
fn Counter(on_count: EventHandler<i32>) -> Element {
    let mut counter = use_signal(|| 0);

    rsx! {

        button {
            margin: ".24rem 0",
            onclick: move |_| {
                counter += 1;
                on_count(*counter.read());
            },
            "+"
        }
        output { margin: "0 1ch", "{counter}" }
    }
}

struct CounterWebComponent;

impl DioxusWebComponent for CounterWebComponent {
    fn attributes() -> &'static [&'static str] {
        &["name"]
    }

    fn element() -> Element {
        let Context { event_target, .. } = use_context();
        let on_count = custom_event_handler(event_target, "count", CustomEventOptions::default());

        rsx! { Counter { on_count } }
    }
}
