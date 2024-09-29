use std::rc::Rc;
use std::sync::{mpsc, RwLock};

use dioxus::hooks::UnboundedSender;
use dioxus::prelude::LaunchBuilder;
use dioxus::web::Config;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, HtmlElement, ShadowRoot};

use crate::{InjectedStyle, Message, Property, Shared};

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
        // XXX Create an element to attach the dioxus component
        // Dioxus require a `web_sys::Element`, and ShadowRoot is not an Element
        // So we use a `<div class="dioxus"></div>` to wrap the component
        // See https://github.com/DioxusLabs/dioxus/pull/3012
        let window = window().unwrap_throw();
        let document = window.document().unwrap_throw();
        let inner_elt = document.create_element("div").unwrap_throw();
        inner_elt.set_class_name("dioxus");

        self.style.inject(&document, root);
        root.append_child(&inner_elt).unwrap_throw();

        RustComponentInstance {
            attributes: self.attributes(),
            inner_elt,
            dx_el_builder: self.dx_el_builder,
            tx: Rc::default(),
        }
    }
}

#[wasm_bindgen(skip_typescript)]
pub struct RustComponentInstance {
    attributes: Vec<String>,
    inner_elt: web_sys::Element,
    dx_el_builder: DxElBuilder,
    tx: Rc<RwLock<Option<UnboundedSender<Message>>>>,
}

#[wasm_bindgen]
impl RustComponentInstance {
    pub fn connect(&mut self, event_target: &HtmlElement) {
        let ctx = Shared {
            attributes: self.attributes.clone(),
            event_target: event_target.clone(),
            tx: Rc::clone(&self.tx),
        };

        let config = Config::new().rootelement(self.inner_elt.clone());
        LaunchBuilder::web()
            .with_cfg(config)
            .with_context(ctx)
            .launch(self.dx_el_builder);
    }

    fn send(&mut self, message: Message) {
        let tx = Rc::clone(&self.tx);
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
        if old_value != new_value {
            self.send(Message::SetAttribute {
                name,
                value: new_value,
            });
        }
    }

    #[wasm_bindgen(js_name = "getProperty")]
    pub fn get_property(&mut self, name: String) -> JsValue {
        let (tx, rx) = mpsc::sync_channel(0); // oneshot
        self.send(Message::Get { name, tx });
        rx.recv().unwrap_or(JsValue::undefined())
    }

    #[wasm_bindgen(js_name = "setProperty")]
    pub fn set_property(&mut self, name: String, value: JsValue) {
        self.send(Message::Set { name, value });
    }

    pub fn disconnect(&mut self) {
        // Skip if poisoned
        if let Ok(mut tx) = self.tx.write() {
            tx.take();
        }
    }
}
