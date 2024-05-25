use std::borrow::BorrowMut;
use std::rc::Rc;

use dioxus::prelude::LaunchBuilder;
use dioxus::web::Config;
use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlElement, ShadowRoot};

use crate::{InjectedStyle, InnerComponent, Shared};

pub(crate) type DxElBuilder = fn() -> dioxus::dioxus_core::Element;

/// The Rust component
#[wasm_bindgen(skip_typescript)]
pub struct RustComponent {
    pub(crate) attributes: Vec<String>,
    pub(crate) style: InjectedStyle,
    pub(crate) dx_el_builder: DxElBuilder,
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
        // So we use a `<div class="dioxus"></div>` to wrap the component
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
            inner: InnerComponent::default(),
        }
    }
}

#[wasm_bindgen(skip_typescript)]
pub struct RustComponentInstance {
    attributes: Vec<String>,
    inner_elt: web_sys::Element,
    dx_el_builder: DxElBuilder,
    inner: InnerComponent,
}

#[wasm_bindgen]
impl RustComponentInstance {
    pub fn connect(&mut self, event_target: HtmlElement) {
        let ctx = Shared {
            attributes: self.attributes.clone(),
            event_target,
            component: Rc::clone(&self.inner),
            initialized: false,
        };

        let config = Config::new().rootelement(self.inner_elt.clone());
        LaunchBuilder::web()
            .with_cfg(config)
            .with_context(ctx)
            .launch(self.dx_el_builder);
    }

    #[wasm_bindgen(js_name = "attributeChanged")]
    #[allow(clippy::needless_pass_by_value)]
    pub fn attribute_changed(
        &self,
        name: String,
        old_value: Option<String>,
        new_value: Option<String>,
    ) {
        let mut inner = Rc::clone(&self.inner);
        let inner = inner.borrow_mut();
        let mut boxed = inner.as_ref().borrow_mut();
        let Some(component) = boxed.as_mut() else {
            return;
        };

        if old_value != new_value {
            component.set_attribute(&name, new_value);
        }
    }

    pub fn disconnect(&mut self) {
        let mut inner = Rc::clone(&self.inner);
        inner.borrow_mut().take();
    }
}
