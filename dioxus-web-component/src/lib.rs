#![doc = include_str!("../README.md")]
#![allow(clippy::multiple_crate_versions)]

use std::borrow::Cow;

use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{Document, ShadowRoot};

use crate::rust_component::RustComponent;
pub use dioxus_web_component_macro::web_component;

mod event;
pub use self::event::*;

mod rust_component;

/// A message send to the dioxus component
#[non_exhaustive]
pub enum Message {
    /// An attribute value changed
    AttributeChanged {
        /// The attribute name
        name: String,
        /// The new value
        new_value: Option<String>,
    },
}

/// A context provided by the web component
#[derive(Clone)]
pub struct Context {
    /// The web component event target use to dispatch custom event
    pub event_target: web_sys::HtmlElement,
    /// The message receiver
    pub rx: Receiver<Message>,
}

/// Provide style to the web component
#[derive(Debug, Clone, Default)]
pub enum InjectedStyle {
    /// No style provided
    #[default]
    None,
    /// Raw CSS content to go in an HTML `<style>`
    Css(Cow<'static, str>),
    /// Url containing the stylesheet to go in an HTML `<link rel="stylesheet" href="...">`
    Stylesheet(Cow<'static, str>),
    /// Multiple styles
    Multiple(Vec<InjectedStyle>),
}

impl InjectedStyle {
    /// Build with a static CSS code
    #[must_use]
    pub const fn css(css: &'static str) -> Self {
        Self::Css(Cow::Borrowed(css))
    }

    /// Build with a static path to a stylesheet, e.g. an URL
    #[must_use]
    pub const fn stylesheet(url: &'static str) -> Self {
        Self::Stylesheet(Cow::Borrowed(url))
    }

    fn inject(&self, document: &Document, root: &ShadowRoot) {
        match self {
            Self::None => {}
            Self::Css(css) => {
                let style_el = document.create_element("style").unwrap_throw();
                style_el.set_inner_html(css);
                root.append_child(&style_el).unwrap_throw();
            }
            Self::Stylesheet(url) => {
                let link_el = document.create_element("link").unwrap_throw();
                link_el.set_attribute("rel", "stylesheet").unwrap_throw();
                link_el.set_attribute("href", url).unwrap_throw();
                root.append_child(&link_el).unwrap_throw();
            }
            Self::Multiple(styles) => {
                for style in styles {
                    style.inject(document, root);
                }
            }
        }
    }
}

/// Dioxus web component
pub trait DioxusWebComponent {
    /// Provide observable attributes
    #[must_use]
    fn attributes() -> &'static [&'static str] {
        &[]
    }

    /// Provide the dioxus element
    fn element() -> Element;

    /// Provide the CSS style
    #[must_use]
    fn style() -> InjectedStyle {
        InjectedStyle::default()
    }
}

/// Register a Dioxus web component
pub fn register_dioxus_web_component<E>(custom_tag: &str)
where
    E: DioxusWebComponent,
{
    let attributes = E::attributes().iter().map(ToString::to_string).collect();
    let dx_el_builder = E::element;
    let style = E::style();
    let rust_component = RustComponent {
        attributes,
        dx_el_builder,
        style,
    };
    register_web_component(custom_tag, rust_component);
}

#[wasm_bindgen(module = "/src/shim.js")]
extern "C" {
    #[allow(unsafe_code)]
    fn register_web_component(custom_tag: &str, rust_component: RustComponent);
}

#[doc(hidden)]
pub type Sender<T> = async_channel::Sender<T>;
#[doc(hidden)]
pub type Receiver<T> = async_channel::Receiver<T>;

pub(crate) fn create_channel<T>() -> (Sender<T>, Receiver<T>) {
    async_channel::unbounded()
}
