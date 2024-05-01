use dioxus::prelude::LaunchBuilder;
use dioxus::web::Config;
use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlElement, ShadowRoot};

use crate::{create_channel, Context, InjectedStyle, Message, Sender};

pub(crate) type DxElBuilder = fn() -> dioxus::dioxus_core::Element;

/// The Rust component
#[wasm_bindgen(skip_typescript)]
pub struct RustComponent {
    pub(super) attributes: Vec<String>,
    pub(super) dx_el_builder: DxElBuilder,
    pub(super) style: InjectedStyle,
}

#[wasm_bindgen]
impl RustComponent {
    #[wasm_bindgen(getter)]
    pub fn attributes(&self) -> Vec<String> {
        self.attributes.clone()
    }

    #[wasm_bindgen(js_name = "newInstance")]
    pub fn new_instance(&self, root: &ShadowRoot) -> RustComponentInstance {
        // XXX Create an element to attach the dioxus component
        // Dioxus require a `web_sys::Element`, and ShadowRoot is not an Element
        // So create a `<div class="dioxus"></div>` element
        let window = window().unwrap_throw();
        let document = window.document().unwrap_throw();
        let inner = document.create_element("div").unwrap_throw();
        inner.set_class_name("dioxus");

        // Inject style
        match &self.style {
            InjectedStyle::None => {}
            InjectedStyle::Css(css) => {
                let style_el = document.create_element("style").unwrap_throw();
                style_el.set_inner_html(css);
                root.append_child(&style_el).unwrap_throw();
            }
            InjectedStyle::Stylesheet(url) => {
                let link_el = document.create_element("link").unwrap_throw();
                link_el.set_attribute("rel", "stylesheet").unwrap_throw();
                link_el.set_attribute("href", url).unwrap_throw();
                root.append_child(&link_el).unwrap_throw();
            }
        }

        root.append_child(&inner).unwrap_throw();

        RustComponentInstance {
            attributes: self.attributes.clone(),
            dx_el_builder: self.dx_el_builder,
            inner,
            tx: None,
        }
    }
}

#[wasm_bindgen(skip_typescript)]
pub struct RustComponentInstance {
    attributes: Vec<String>,
    dx_el_builder: DxElBuilder,
    inner: web_sys::Element,
    tx: Option<Sender<Message>>,
}

#[wasm_bindgen]
impl RustComponentInstance {
    pub fn connect(&mut self, event_target: HtmlElement) {
        if self.tx.is_some() {
            // Connect require the injected state
            return;
        };

        let (tx, rx) = create_channel();
        self.tx = Some(tx.clone());

        // Initial state
        for attr in &self.attributes {
            let initial_value = event_target.get_attribute(attr);
            let msg = Message::AttributeChanged {
                name: attr.to_owned(),
                new_value: initial_value,
            };
            tx.try_send(msg).unwrap_throw();
        }

        // Bootstrap Dioxus component
        let ctx = Context { event_target, rx };
        let config = Config::new().rootelement(self.inner.clone());
        LaunchBuilder::web()
            .with_cfg(config)
            .with_context(ctx)
            .launch(self.dx_el_builder);
    }

    pub fn disconnect(&mut self) {
        let _ = self.tx.take();
    }

    #[wasm_bindgen(js_name = "attributeChanged")]
    #[allow(clippy::needless_pass_by_value)]
    pub fn attribute_changed(
        &self,
        name: String,
        old_value: Option<String>,
        new_value: Option<String>,
    ) {
        if old_value != new_value {
            let Some(tx) = &self.tx else {
                return;
            };
            // Notify Dioxus component
            let msg = Message::AttributeChanged { name, new_value };
            tx.try_send(msg).unwrap_throw();
        }
    }
}
