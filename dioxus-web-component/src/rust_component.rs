use std::sync::{Arc, RwLock};

use dioxus::hooks::UnboundedSender;
use dioxus::logger::tracing::{debug, warn};
use dioxus::prelude::LaunchBuilder;
use dioxus::web::Config;
use futures::channel::oneshot;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, HtmlElement, ShadowRoot};

use crate::{InjectedStyle, Message, Property, Shared, SharedEventTarget, SharedJsValue};

pub(crate) type DxElBuilder = fn() -> dioxus::dioxus_core::Element;

/// The Rust component
#[wasm_bindgen(skip_typescript)]
pub struct RustComponent {
    pub(crate) attributes: Vec<String>,
    pub(crate) properties: Vec<Property>,
    pub(crate) style: InjectedStyle,
    pub(crate) dx_el_builder: DxElBuilder,
}

#[wasm_bindgen]
impl RustComponent {
    #[wasm_bindgen(getter)]
    pub fn attributes(&self) -> Vec<String> {
        self.attributes.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn properties(&self) -> Vec<Property> {
        self.properties.clone()
    }

    #[wasm_bindgen(js_name = "newInstance")]
    pub fn new_instance(&self, root: &ShadowRoot) -> RustComponentInstance {
        debug!(?root, "new instance");
        let window = window().unwrap_throw();
        let document = window.document().unwrap_throw();
        self.style.inject(&document, root);

        // XXX Create an element to attach the dioxus component
        // Dioxus require a `web_sys::Element`, and ShadowRoot is not an Element
        // So we use a `<div class="dioxus"></div>` to wrap the component
        let inner_elt = document.create_element("div").unwrap_throw();
        inner_elt.set_class_name("dioxus");
        root.append_child(&inner_elt).unwrap_throw();

        RustComponentInstance {
            attributes: self.attributes(),
            inner: inner_elt.into(),
            dx_el_builder: self.dx_el_builder,
            tx: Arc::default(),
        }
    }
}

#[wasm_bindgen(skip_typescript)]
pub struct RustComponentInstance {
    attributes: Vec<String>,
    inner: web_sys::Node,
    dx_el_builder: DxElBuilder,
    tx: Arc<RwLock<Option<UnboundedSender<Message>>>>,
}

#[wasm_bindgen]
impl RustComponentInstance {
    pub fn connect(&mut self, event_target: &HtmlElement) {
        debug!(host = ?event_target, "Connect");
        let ctx = Shared {
            attributes: self.attributes.clone(),
            event_target: SharedEventTarget(event_target.clone()),
            tx: Arc::clone(&self.tx),
        };

        let node = self.inner.clone().unchecked_into();
        let config = Config::new().rootnode(node);
        LaunchBuilder::web()
            .with_cfg(config)
            .with_context(ctx)
            .launch(self.dx_el_builder);
    }

    fn send(&mut self, message: Message) {
        debug!(?message, "sending message");
        let tx = Arc::clone(&self.tx);
        spawn_local(async move {
            // Read (skip if poisoned)
            if let Ok(sender) = tx.try_read() {
                if let Some(sender) = sender.as_ref() {
                    let _ = sender.unbounded_send(message);
                }
            }
        });
    }

    #[wasm_bindgen(js_name = "attributeChanged")]
    #[allow(clippy::needless_pass_by_value)]
    pub fn attribute_changed(
        &mut self,
        name: String,
        old_value: Option<String>,
        new_value: Option<String>,
    ) {
        debug!(%name, ?old_value, ?new_value, "attribute changed");
        if old_value != new_value {
            self.send(Message::SetAttribute {
                name,
                value: new_value,
            });
        }
    }

    #[wasm_bindgen(js_name = "getProperty")]
    pub async fn get_property(&mut self, name: String) -> JsValue {
        debug!(%name, "get property");
        let (tx, rx) = oneshot::channel();
        self.send(Message::Get { name, tx });
        match rx.await {
            Ok(SharedJsValue(value)) => value,
            Err(error) => {
                warn!(?error, "Fail to get property");
                JsValue::undefined()
            }
        }
    }

    #[wasm_bindgen(js_name = "setProperty")]
    pub fn set_property(&mut self, name: String, value: JsValue) {
        debug!(%name, ?value, "set property");
        let value = SharedJsValue(value);
        let message = Message::Set { name, value };
        self.send(message);
    }

    pub fn disconnect(&mut self) {
        debug!("disconnect");
        // Skip if poisoned
        if let Ok(mut tx) = self.tx.write() {
            tx.take();
        }
    }
}
