#![doc = include_str!("../README.md")]
#![allow(clippy::multiple_crate_versions)]

use dioxus::prelude::*;
use dioxus_web_component::{web_component, InjectedStyle};

/// The main application
///
/// This is a Dioxus standard component
#[component]
pub fn App() -> Element {
    rsx! {
        h1 { "You can use web-component in Dioxus" }
        h2 { "You can write web-component with Dioxus" }
        h3 { "Therefore you can write dioxus web-component in Dioxus" }
        div { id: "links",
            // We use the web-component inside dioxus
            plop-link { href: "https://dioxuslabs.com/learn/", "📚 Learn Dioxus" }
            plop-link { href: "https://docs.rs/dioxus-web-component/latest/dioxus_web_component/",
                "🕸️ Dioxus web components"
            }
        }
    }
}

/// A link component
///
/// This is a web-component `plop-link` build with Dioxus
#[web_component(tag = "plop-link", style = InjectedStyle::css(include_str!("link.css")))]
pub fn Link(
    /// The link href
    href: String,
) -> Element {
    rsx! {
        a { target: "_blank", href: "{href}",
            // See <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/slot>
            slot {}
        }
    }
}
