#![doc = include_str!("../README.md")]
#![allow(clippy::multiple_crate_versions)]

use dioxus::logger::tracing::Level;
use dioxus::{logger, prelude::*};
use dioxus_web_component::{web_component, InjectedStyle};
use wasm_bindgen::prelude::*;

/// Install (register) the web component
#[wasm_bindgen(start)]
pub fn register() {
    let _ = logger::init(Level::INFO);
    register_greetings();
}

#[web_component(tag = "plop-greeting", style = InjectedStyle::css(include_str!("./style.css"))  )]
fn Greetings(
    // The name can be set as an attribute of the plop-greeting HTML element
    #[attribute]
    #[property]
    name: String,
) -> Element {
    rsx! {
        p { "Hello {name}!" }
    }
}
