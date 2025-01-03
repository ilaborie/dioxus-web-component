#![doc = include_str!("../README.md")]
#![allow(clippy::multiple_crate_versions)]

use std::sync::Arc;
use std::sync::RwLock;

use dioxus::dioxus_core::Element;
use dioxus::hooks::UnboundedSender;
use dioxus::logger::tracing::debug;
use futures::channel::oneshot;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::rust_component::RustComponent;

pub use dioxus_web_component_macro::web_component;

mod event;
pub use self::event::*;

mod style;
pub use self::style::*;

mod rust_component;

/// Re-export, use this trait in the coroutine
pub use futures::StreamExt;

/// Message from web component to dioxus
#[derive(Debug)]
#[non_exhaustive]
pub enum Message {
    /// Set attribute
    SetAttribute {
        /// Attribute name
        name: String,
        /// Attribute value
        value: Option<String>,
    },
    /// Get property
    Get {
        /// Property name
        name: String,
        /// reply channel
        tx: oneshot::Sender<SharedJsValue>,
    },
    /// Set property
    Set {
        /// Property name
        name: String,
        /// Property value
        value: SharedJsValue,
    },
}

#[derive(Clone)]
struct SharedEventTarget(web_sys::HtmlElement);

#[allow(unsafe_code)]
// SAFETY:
// In a Web WASM context, without thread.
// This only be used to display an event, no update are made here
unsafe impl Send for SharedEventTarget {}

#[allow(unsafe_code)]
// SAFETY:
// In a Web WASM context, without thread.
// This only be used to display an event, no update are made here
unsafe impl Sync for SharedEventTarget {}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct SharedJsValue(JsValue);

#[allow(unsafe_code)]
// SAFETY:
// In a Web WASM context, without thread.
// This only be used to display an event, no update are made here
unsafe impl Send for SharedJsValue {}

#[allow(unsafe_code)]
// SAFETY:
// In a Web WASM context, without thread.
// This only be used to display an event, no update are made here
unsafe impl Sync for SharedJsValue {}

/// A context provided by the web component
#[derive(Clone)]
pub struct Shared {
    attributes: Vec<String>,
    event_target: SharedEventTarget,
    tx: Arc<RwLock<Option<UnboundedSender<Message>>>>,
}

impl Shared {
    /// The web component event target use to dispatch custom event
    #[must_use]
    pub fn event_target(&self) -> &HtmlElement {
        &self.event_target.0
    }

    /// Set the receiver
    pub fn set_tx(&mut self, tx: UnboundedSender<Message>) {
        // initial state
        let trg = self.event_target();
        for attr in &self.attributes {
            let Some(value) = trg.get_attribute(attr) else {
                continue;
            };
            let _ = tx.unbounded_send(Message::SetAttribute {
                name: attr.to_string(),
                value: Some(value),
            });
        }

        // Keep sender (skip if poisoned)
        if let Ok(mut cell) = self.tx.write() {
            *cell = Some(tx);
        }
    }
}

/// Dioxus web component
pub trait DioxusWebComponent {
    /// Set an HTML attribute
    fn set_attribute(&mut self, attribute: &str, value: Option<String>) {
        let _ = value;
        let _ = attribute;
    }

    /// Set a property
    fn set_property(&mut self, property: &str, value: JsValue) {
        let _ = value;
        let _ = property;
    }

    /// Get a property
    fn get_property(&mut self, property: &str) -> JsValue {
        let _ = property;
        JsValue::undefined()
    }

    /// Handle a message
    fn handle_message(&mut self, message: Message) {
        debug!(?message, "handle message");
        match message {
            Message::SetAttribute { name, value } => self.set_attribute(&name, value),
            Message::Get { name, tx } => {
                let value = self.get_property(&name);
                let _ = tx.send(SharedJsValue(value));
            }
            Message::Set { name, value } => self.set_property(&name, value.0),
        }
    }
}

/// Property
#[wasm_bindgen(skip_typescript)]
#[derive(Debug, Clone)]
pub struct Property {
    /// Name
    name: String,
    /// Readonly
    readonly: bool,
}

impl Property {
    /// Create a property
    pub fn new(name: impl Into<String>, readonly: bool) -> Self {
        let name = name.into();
        Self { name, readonly }
    }
}

#[wasm_bindgen]
impl Property {
    /// Get name
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Is property readonly
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn readonly(&self) -> bool {
        self.readonly
    }
}

/// Register a Dioxus web component
pub fn register_dioxus_web_component(
    custom_tag: &str,
    attributes: Vec<String>,
    properties: Vec<Property>,
    style: InjectedStyle,
    dx_el_builder: fn() -> Element,
) {
    let rust_component = RustComponent {
        attributes,
        properties,
        style,
        dx_el_builder,
    };
    register_web_component(custom_tag, rust_component);
}

#[wasm_bindgen(module = "/src/shim.js")]
extern "C" {
    #[allow(unsafe_code)]
    fn register_web_component(custom_tag: &str, rust_component: RustComponent);
}
