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
    register_greetings();

    Ok(())
}

#[web_component(tag = "plop-greeting", style = InjectedStyle::css(include_str!("./style.css"))  )]
fn Greetings(name: String) -> Element {
    rsx! { p { "Hello {name}!" } }
}
