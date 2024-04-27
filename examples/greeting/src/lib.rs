#![doc = include_str!("../README.md")]
#![allow(clippy::multiple_crate_versions)]

use dioxus::prelude::*;
use dioxus_web_component::{register_dioxus_web_component, Context, DioxusWebComponent, Message};
use wasm_bindgen::prelude::*;

/// Install (register) the web component
///
/// # Errors
///
/// Registering the web-component may fail
#[wasm_bindgen(start)]
pub fn register() -> Result<(), JsValue> {
    register_dioxus_web_component::<GreetingsWebComponent>("plop-greeting");

    Ok(())
}

/// The Dioxus component
#[component]
fn Greetings(name: String) -> Element {
    rsx! {
        p { "Hello {name}!" }
    }
}

struct GreetingsWebComponent;

impl DioxusWebComponent for GreetingsWebComponent {
    fn attributes() -> &'static [&'static str] {
        &["name"]
    }

    fn element() -> Element {
        let mut name_signal = use_signal(String::new);
        let Context { rx, .. } = use_context();
        let _change_handler = use_coroutine::<(), _, _>(|_| async move {
            while let Ok(Message::AttributeChanged { new_value, .. }) = rx.recv().await {
                let value = new_value.unwrap_or_else(|| "World".to_owned());
                name_signal.set(value);
            }
        });

        rsx! {
            Greetings {
                name: name_signal
            }
        }
    }
}
