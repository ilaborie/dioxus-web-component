#![doc = include_str!("../README.md")]
#![allow(clippy::multiple_crate_versions)]

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use dioxus_web_component::{
    register_dioxus_web_component, web_component, DioxusWebComponent, InjectedStyle, Message,
    Shared, StreamExt,
};

/// The main application
///
/// This is a Dioxus standard component
#[component]
pub fn App() -> Element {
    info!("🖼️ Render App");
    rsx! {
        head::Link { rel: "stylesheet", href: "main.css" }
        h1 { "You can use web-component in Dioxus" }
        h2 { "You can write web-component with Dioxus" }
        h3 { "Therefore you can write dioxus web-component in Dioxus" }
        div { id: "links",
            // We use the web-component instead of a link
            plop-link { href: "https://dioxuslabs.com/learn/0.5/", "📚 Learn Dioxus" }
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
            slot {}
        }
    }
}

/// Pure Dioxus equivalent
#[component]
pub fn Link2(
    /// The link href
    href: String,
    /// chlildren
    children: Element,
) -> Element {
    info!(%href, "🖼️🖼️ Render Link2");
    rsx! {
        a { target: "_blank", href: "{href}", {children} }
    }
}

// #[doc(hidden)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct LinkWebComponent {
//     href: Signal<String>,
// }

// impl DioxusWebComponent for LinkWebComponent {
//     fn set_attribute(&mut self, attribute: &str, value: Option<String>) {
//         if attribute == "href" {
//             let value = value.unwrap_or_default();
//             self.href.set(value);
//         }
//     }
// }

// fn link_builder() -> Element {
//     info!("🌀 LINK");
//     let mut wc = use_context::<Shared>();
//     let href = use_signal(String::default);
//     let mut link_wc = LinkWebComponent { href };

//     let coroutine = use_coroutine::<Message, _, _>(move |mut rx| async move {
//         while let Some(msg) = rx.next().await {
//             info!(?msg, "Message to handle");
//             link_wc.handle_message(msg);
//         }
//     });

//     use_effect(move || {
//         wc.set_tx(coroutine.tx());
//     });

//     rsx! {
//             Link2 { href: href, slot {}
//         }
//     }
// }

// /// TODO register
// pub fn register_link() {
//     register_dioxus_web_component(
//         "plop-link",
//         vec!["href".to_string()],
//         vec![],
//         InjectedStyle::None,
//         link_builder,
//     );
// }
