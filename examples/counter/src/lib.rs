#![doc = include_str!("../README.md")]
#![allow(clippy::multiple_crate_versions)]

use dioxus::prelude::*;
use dioxus_web_component::{web_component, InjectedStyle};
use wasm_bindgen::prelude::*;

/// Install (register) the web component
///
/// # Errors
///
/// Registering the web-component may fail
#[wasm_bindgen(start)]
pub fn register() -> Result<(), JsValue> {
    // The register counter is generated by the `#[web_component(...)]` macro
    register_counter();
    Ok(())
}

/// The Dioxus component
#[web_component(tag = "plop-counter", style = InjectedStyle::stylesheet("./style.css"))]
fn Counter(
    // The label is only available with a property
    #[property] label: String,
    // This component can trigger a custom 'count' event
    on_count: EventHandler<i32>,
) -> Element {
    let mut counter = use_signal(|| 0);

    rsx! {
        span { "{label}" }
        button {
            onclick: move |_| {
                counter += 1;
                on_count(counter());
            },
            "+"
        }
        output { "{counter}" }
    }
}
