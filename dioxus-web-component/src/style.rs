use std::borrow::Cow;

use wasm_bindgen::UnwrapThrowExt as _;
use web_sys::{Document, ShadowRoot};

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

    pub(crate) fn inject(&self, document: &Document, root: &ShadowRoot) {
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
