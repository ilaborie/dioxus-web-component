#![doc = include_str!("../README.md")]
#![allow(clippy::multiple_crate_versions)]

use std::cell::RefCell;
use std::rc::Rc;

use dioxus::dioxus_core::Element;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::rust_component::RustComponent;
pub use dioxus_web_component_macro::web_component;

mod event;
pub use self::event::*;

mod style;
pub use self::style::*;

mod rust_component;

/// A context provided by the web component
#[derive(Clone)]
pub struct Shared {
    event_target: web_sys::HtmlElement,
    attributes: Vec<String>,
    component: InnerComponent,
}

type InnerComponent = Rc<RefCell<Option<Box<dyn DioxusWebComponent + 'static>>>>;

impl Shared {
    /// The web component event target use to dispatch custom event
    #[must_use]
    pub fn event_target(&self) -> HtmlElement {
        self.event_target.clone()
    }

    /// Initialize the component
    pub fn init_component(&self, mut wc: impl DioxusWebComponent + 'static) {
        // Initial state
        for attr in &self.attributes {
            let initial_value = self.event_target.get_attribute(attr);
            wc.set_attribute(attr, initial_value);
        }
        // Update shared component
        let component = Rc::clone(&self.component);
        let mut component = component.borrow_mut();
        *component = Some(Box::new(wc));
    }
}

/// Dioxus web component
pub trait DioxusWebComponent {
    /// Set an HTML attribute
    fn set_attribute(&mut self, attribute: &str, value: Option<String>);

    // TODO get/set properties (See issue #18)
}

/// Register a Dioxus web component
// TODO attributes, dx_el_builder, style
pub fn register_dioxus_web_component(
    custom_tag: &str,
    attributes: Vec<String>,
    style: InjectedStyle,
    dx_el_builder: fn() -> Element,
) {
    let rust_component = RustComponent {
        attributes,
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
