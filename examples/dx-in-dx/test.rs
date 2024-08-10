#![feature(prelude_import)]
/*!# Development

Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload
```

- Open the browser to <http://localhost:8080>
*/
#![allow(clippy::multiple_crate_versions)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use dioxus::prelude::*;
use dioxus_web_component::{web_component, InjectedStyle};
#[allow(non_snake_case)]
/// The main application
///
/// This is a Dioxus standard component
pub fn App() -> Element {
    {
        {
            Some({
                static TEMPLATE: dioxus_core::Template = dioxus_core::Template {
                    name: "examples/dx-in-dx/src/lib.rs:12:5:297",
                    roots: &[
                        dioxus_core::TemplateNode::Element {
                            tag: dioxus_elements::link::TAG_NAME,
                            namespace: dioxus_elements::link::NAME_SPACE,
                            attrs: &[
                                dioxus_core::TemplateAttribute::Static {
                                    name: dioxus_elements::link::rel.0,
                                    namespace: dioxus_elements::link::rel.1,
                                    value: "stylesheet",
                                },
                                dioxus_core::TemplateAttribute::Static {
                                    name: dioxus_elements::link::href.0,
                                    namespace: dioxus_elements::link::href.1,
                                    value: "main.css",
                                },
                            ],
                            children: &[],
                        },
                        dioxus_core::TemplateNode::Element {
                            tag: dioxus_elements::img::TAG_NAME,
                            namespace: dioxus_elements::img::NAME_SPACE,
                            attrs: &[
                                dioxus_core::TemplateAttribute::Static {
                                    name: dioxus_elements::img::src.0,
                                    namespace: dioxus_elements::img::src.1,
                                    value: "header.svg",
                                },
                                dioxus_core::TemplateAttribute::Static {
                                    name: dioxus_elements::img::id.0,
                                    namespace: dioxus_elements::img::id.1,
                                    value: "header",
                                },
                            ],
                            children: &[],
                        },
                        dioxus_core::TemplateNode::Element {
                            tag: dioxus_elements::h1::TAG_NAME,
                            namespace: dioxus_elements::h1::NAME_SPACE,
                            attrs: &[],
                            children: &[
                                dioxus_core::TemplateNode::Text {
                                    text: "You can use web-component in Dioxus",
                                },
                            ],
                        },
                        dioxus_core::TemplateNode::Element {
                            tag: dioxus_elements::h2::TAG_NAME,
                            namespace: dioxus_elements::h2::NAME_SPACE,
                            attrs: &[],
                            children: &[
                                dioxus_core::TemplateNode::Text {
                                    text: "You can write web-component with Dioxus",
                                },
                            ],
                        },
                        dioxus_core::TemplateNode::Element {
                            tag: dioxus_elements::h3::TAG_NAME,
                            namespace: dioxus_elements::h3::NAME_SPACE,
                            attrs: &[],
                            children: &[
                                dioxus_core::TemplateNode::Text {
                                    text: "Therefore you can write dioxus web-component in Dioxus",
                                },
                            ],
                        },
                        dioxus_core::TemplateNode::Element {
                            tag: dioxus_elements::div::TAG_NAME,
                            namespace: dioxus_elements::div::NAME_SPACE,
                            attrs: &[
                                dioxus_core::TemplateAttribute::Static {
                                    name: dioxus_elements::div::id.0,
                                    namespace: dioxus_elements::div::id.1,
                                    value: "links",
                                },
                            ],
                            children: &[
                                dioxus_core::TemplateNode::Element {
                                    tag: "plop-link",
                                    namespace: None,
                                    attrs: &[
                                        dioxus_core::TemplateAttribute::Static {
                                            name: "href",
                                            namespace: None,
                                            value: "https://dioxuslabs.com/learn/0.5/",
                                        },
                                    ],
                                    children: &[
                                        dioxus_core::TemplateNode::Text {
                                            text: "📚 Learn Dioxus",
                                        },
                                    ],
                                },
                                dioxus_core::TemplateNode::Element {
                                    tag: "plop-link",
                                    namespace: None,
                                    attrs: &[
                                        dioxus_core::TemplateAttribute::Static {
                                            name: "href",
                                            namespace: None,
                                            value: "https://dioxuslabs.com/awesome",
                                        },
                                    ],
                                    children: &[
                                        dioxus_core::TemplateNode::Text {
                                            text: "🚀 Awesome Dioxus",
                                        },
                                    ],
                                },
                                dioxus_core::TemplateNode::Element {
                                    tag: "plop-link",
                                    namespace: None,
                                    attrs: &[
                                        dioxus_core::TemplateAttribute::Static {
                                            name: "href",
                                            namespace: None,
                                            value: "https://github.com/dioxus-community/",
                                        },
                                    ],
                                    children: &[
                                        dioxus_core::TemplateNode::Text {
                                            text: "📡 Community Libraries",
                                        },
                                    ],
                                },
                                dioxus_core::TemplateNode::Element {
                                    tag: "plop-link",
                                    namespace: None,
                                    attrs: &[
                                        dioxus_core::TemplateAttribute::Static {
                                            name: "href",
                                            namespace: None,
                                            value: "https://github.com/DioxusLabs/dioxus-std",
                                        },
                                    ],
                                    children: &[
                                        dioxus_core::TemplateNode::Text {
                                            text: "⚙\u{fe0f} Dioxus Standard Library",
                                        },
                                    ],
                                },
                                dioxus_core::TemplateNode::Element {
                                    tag: "plop-link",
                                    namespace: None,
                                    attrs: &[
                                        dioxus_core::TemplateAttribute::Static {
                                            name: "href",
                                            namespace: None,
                                            value: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                                        },
                                    ],
                                    children: &[
                                        dioxus_core::TemplateNode::Text {
                                            text: "💫 VSCode Extension",
                                        },
                                    ],
                                },
                                dioxus_core::TemplateNode::Element {
                                    tag: "plop-link",
                                    namespace: None,
                                    attrs: &[
                                        dioxus_core::TemplateAttribute::Static {
                                            name: "href",
                                            namespace: None,
                                            value: "https://discord.gg/XgGxMSkvUM",
                                        },
                                    ],
                                    children: &[
                                        dioxus_core::TemplateNode::Text {
                                            text: "👋 Community Discord",
                                        },
                                    ],
                                },
                            ],
                        },
                    ],
                    node_paths: &[],
                    attr_paths: &[],
                };
                {
                    let __vnodes = dioxus_core::VNode::new(
                        None,
                        TEMPLATE,
                        Box::new([]),
                        Box::new([]),
                    );
                    __vnodes
                }
            })
        }
    }
}
///Properties for the [`Link`] component.
#[allow(non_camel_case_types)]
pub struct LinkProps {
    pub href: String,
}
impl LinkProps {
    /**
Create a builder for building `LinkProps`.
On the builder, call `.href(...)` to set the values of the fields.
Finally, call `.build()` to create the instance of `LinkProps`.
                    */
    #[allow(dead_code, clippy::type_complexity)]
    pub fn builder() -> LinkPropsBuilder<((),)> {
        LinkPropsBuilder {
            fields: ((),),
            _phantom: ::core::default::Default::default(),
        }
    }
}
#[must_use]
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
pub struct LinkPropsBuilder<TypedBuilderFields> {
    fields: TypedBuilderFields,
    _phantom: (),
}
impl dioxus_core::prelude::Properties for LinkProps
where
    Self: Clone,
{
    type Builder = LinkPropsBuilder<((),)>;
    fn builder() -> Self::Builder {
        LinkProps::builder()
    }
    fn memoize(&mut self, new: &Self) -> bool {
        let equal = self == new;
        if !equal {
            let new_clone = new.clone();
            self.href = new_clone.href;
        }
        equal
    }
}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
pub trait LinkPropsBuilder_Optional<T> {
    fn into_value<F: FnOnce() -> T>(self, default: F) -> T;
}
impl<T> LinkPropsBuilder_Optional<T> for () {
    fn into_value<F: FnOnce() -> T>(self, default: F) -> T {
        default()
    }
}
impl<T> LinkPropsBuilder_Optional<T> for (T,) {
    fn into_value<F: FnOnce() -> T>(self, _: F) -> T {
        self.0
    }
}
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl LinkPropsBuilder<((),)> {
    #[allow(clippy::type_complexity)]
    pub fn href(
        self,
        href: impl ::core::fmt::Display,
    ) -> LinkPropsBuilder<((String,),)> {
        let href = (href.to_string(),);
        let (_,) = self.fields;
        LinkPropsBuilder {
            fields: (href,),
            _phantom: self._phantom,
        }
    }
}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
pub enum LinkPropsBuilder_Error_Repeated_field_href {}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl LinkPropsBuilder<((String,),)> {
    #[deprecated(note = "Repeated field href")]
    #[allow(clippy::type_complexity)]
    pub fn href(
        self,
        _: LinkPropsBuilder_Error_Repeated_field_href,
    ) -> LinkPropsBuilder<((String,),)> {
        self
    }
}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
pub enum LinkPropsBuilder_Error_Missing_required_field_href {}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
impl LinkPropsBuilder<((),)> {
    #[deprecated(note = "Missing required field href")]
    pub fn build(
        self,
        _: LinkPropsBuilder_Error_Missing_required_field_href,
    ) -> LinkProps {
        {
            #[cold]
            #[track_caller]
            #[inline(never)]
            const fn panic_cold_explicit() -> ! {
                ::core::panicking::panic_explicit()
            }
            panic_cold_explicit();
        }
    }
}
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl LinkPropsBuilder<((String,),)> {
    pub fn build(self) -> LinkProps {
        let (href,) = self.fields;
        let href = href.0;
        LinkProps { href }
    }
}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::clone::Clone for LinkProps {
    #[inline]
    fn clone(&self) -> LinkProps {
        LinkProps {
            href: ::core::clone::Clone::clone(&self.href),
        }
    }
}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::marker::StructuralPartialEq for LinkProps {}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::cmp::PartialEq for LinkProps {
    #[inline]
    fn eq(&self, other: &LinkProps) -> bool {
        self.href == other.href
    }
}
#[allow(non_snake_case)]
/// A link component
///
/// This is a web-component `plop-link` build with Dioxus
pub fn Link(mut __props: LinkProps) -> Element {
    let LinkProps { mut href } = __props;
    {
        {
            Some({
                static TEMPLATE: dioxus_core::Template = dioxus_core::Template {
                    name: "examples/dx-in-dx/src/lib.rs:37:5:1536",
                    roots: &[
                        dioxus_core::TemplateNode::Element {
                            tag: dioxus_elements::a::TAG_NAME,
                            namespace: dioxus_elements::a::NAME_SPACE,
                            attrs: &[
                                dioxus_core::TemplateAttribute::Static {
                                    name: dioxus_elements::a::target.0,
                                    namespace: dioxus_elements::a::target.1,
                                    value: "_blank",
                                },
                                dioxus_core::TemplateAttribute::Dynamic {
                                    id: 0usize,
                                },
                            ],
                            children: &[
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::slot::TAG_NAME,
                                    namespace: dioxus_elements::slot::NAME_SPACE,
                                    attrs: &[],
                                    children: &[],
                                },
                            ],
                        },
                    ],
                    node_paths: &[],
                    attr_paths: &[&[0u8]],
                };
                {
                    let __vnodes = dioxus_core::VNode::new(
                        None,
                        TEMPLATE,
                        Box::new([]),
                        Box::new([
                            Box::new([
                                dioxus_core::Attribute::new(
                                    dioxus_elements::a::href.0,
                                    (href).to_string().to_string(),
                                    dioxus_elements::a::href.1,
                                    dioxus_elements::a::href.2,
                                ),
                            ]),
                        ]),
                    );
                    __vnodes
                }
            })
        }
    }
}
///Register the `<plop-link>` web-component
pub fn register_link() {
    let attributes = <[_]>::into_vec(
        #[rustc_box]
        ::alloc::boxed::Box::new(["href".to_string()]),
    );
    let properties = ::alloc::vec::Vec::new();
    let style = InjectedStyle::css(
        ":host {\n  margin-top: 20px;\n  margin: 10px;\n}\n\na {\n  color: white;\n  text-decoration: none;\n  border: white 1px solid;\n  border-radius: 5px;\n  padding: 10px;\n}\n\na:hover {\n  background-color: #1f1f1f;\n  cursor: pointer;\n}\n",
    );
    ::dioxus_web_component::register_dioxus_web_component(
        "plop-link",
        attributes,
        properties,
        style,
        link_builder,
    );
}
///The `LinkWebComponent` web-component that implement [`::dioxus_web_component::DioxusWebComponent`]
#[automatically_derived]
#[allow(dead_code)]
pub struct LinkWebComponent {
    href: ::dioxus::prelude::Signal<String>,
}
#[automatically_derived]
#[allow(dead_code)]
impl ::core::clone::Clone for LinkWebComponent {
    #[inline]
    fn clone(&self) -> LinkWebComponent {
        let _: ::core::clone::AssertParamIsClone<::dioxus::prelude::Signal<String>>;
        *self
    }
}
#[automatically_derived]
#[allow(dead_code)]
impl ::core::marker::Copy for LinkWebComponent {}
#[automatically_derived]
impl ::dioxus_web_component::DioxusWebComponent for LinkWebComponent {
    #[allow(clippy::single_match, clippy::redundant_closure)]
    fn set_attribute(&mut self, attribute: &str, new_value: Option<String>) {
        match attribute {
            "href" => {
                let value = new_value
                    .and_then(|value| value.parse().ok())
                    .unwrap_or_else(|| ::std::default::Default::default());
                self.href.set(value);
            }
            _ => {}
        }
    }
    #[allow(clippy::single_match, clippy::redundant_closure)]
    fn set_property(&mut self, property: &str, value: ::wasm_bindgen::JsValue) {
        match property {
            _ => {}
        }
    }
    #[allow(clippy::single_match, clippy::redundant_closure)]
    fn get_property(&mut self, property: &str) -> ::wasm_bindgen::JsValue {
        match property {
            _ => ::wasm_bindgen::JsValue::undefined(),
        }
    }
}
#[doc(hidden)]
#[automatically_derived]
#[allow(clippy::default_trait_access, clippy::clone_on_copy, clippy::redundant_closure)]
fn link_builder() -> ::dioxus::prelude::Element {
    let mut __wc = ::dioxus::prelude::use_context::<::dioxus_web_component::Shared>();
    let href = ::dioxus::prelude::use_signal(|| ::std::default::Default::default());
    let mut __link_web_component = LinkWebComponent { href };
    let __coroutine = ::dioxus::prelude::use_coroutine(move |mut rx| async move {
        use ::dioxus_web_component::{StreamExt, DioxusWebComponent};
        while let Some(message) = rx.next().await {
            ::dioxus::prelude::spawn
            __link_web_component.handle_message(message);
        }
    });
    ::dioxus::prelude::use_effect(move || {
        __wc.set_tx(__coroutine.tx());
    });
    {
        Some({
            static TEMPLATE: dioxus_core::Template = dioxus_core::Template {
                name: "examples/dx-in-dx/src/lib.rs:35:1:1488",
                roots: &[
                    dioxus_core::TemplateNode::Dynamic {
                        id: 0usize,
                    },
                ],
                node_paths: &[&[0u8]],
                attr_paths: &[],
            };
            {
                let __vnodes = dioxus_core::VNode::new(
                    None,
                    TEMPLATE,
                    Box::new([
                        dioxus_core::DynamicNode::Component({
                            use dioxus_core::prelude::Properties;
                            (fc_to_builder(Link).href(href().clone()).build())
                                .into_vcomponent(Link, "Link")
                        }),
                    ]),
                    Box::new([]),
                );
                __vnodes
            }
        })
    }
}
