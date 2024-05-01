#![allow(clippy::min_ident_chars)]

use core::panic;
use std::fmt::Debug;

use darling::ast::NestedMeta;
use darling::{Error, FromMeta};
use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::ext::IdentExt;
use syn::{FnArg, Ident, ItemFn, Pat, PatIdent, PatType, Type};

use crate::{Attribute, Event};

#[derive(Debug, FromMeta)]
struct WebComponentReceiver {
    tag: Option<String>,
    css: Option<String>,        // TODO with include_str!()
    stylesheet: Option<String>, // TODO use vec
}

pub(crate) struct WebComponent {
    tag: String,
    css: Option<String>,        // TODO with include_str!()
    stylesheet: Option<String>, // TODO use vec
    attributes: Vec<Attribute>,
    events: Vec<Event>,
    item_fn: ItemFn,
}

impl WebComponent {
    pub(crate) fn parse(args: TokenStream, mut item_fn: ItemFn) -> Result<Self, Error> {
        let attr_args = NestedMeta::parse_meta_list(args)?;
        let WebComponentReceiver {
            tag,
            css,
            stylesheet,
        } = WebComponentReceiver::from_list(&attr_args)?;

        let tag = tag.unwrap_or_else(|| {
            let ident = item_fn.sig.ident.unraw();
            ident.to_string()
        });

        let mut parsed_events = vec![];
        let mut parsed_attributes = vec![];
        for arg in &mut item_fn.sig.inputs {
            let FnArg::Typed(arg) = arg else {
                continue;
            };

            let PatType { attrs, pat, ty, .. } = arg;
            let Pat::Ident(PatIdent { ident, .. }) = pat.as_ref() else {
                panic!("Expected an ident, got {pat:#?}");
            };

            let ident = ident.clone();
            let ty = Type::clone(ty);

            // Parse argument attributes
            attrs.retain(|attr| {
                if attr.path().is_ident("event") {
                    let event = Event::parse(attr, ident.clone(), ty.clone());
                    parsed_events.push(event);
                    false
                } else if attr.path().is_ident("attribute") {
                    let attr = Attribute::parse(attr, ident.clone(), ty.clone());
                    parsed_attributes.push(attr);
                    false
                } else {
                    true
                }
            });
            // TODO what behavior if no attribute?
        }

        let attributes = parsed_attributes
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;
        let events = parsed_events.into_iter().collect::<Result<Vec<_>, _>>()?;

        let result = Self {
            tag,
            css,
            stylesheet,
            attributes,
            events,
            item_fn,
        };
        Ok(result)
    }
}

impl WebComponent {
    pub fn dioxus_component(&self) -> TokenStream {
        let item_fn = &self.item_fn;
        quote! {
            #[component]
            #item_fn
        }
    }

    pub fn register_fn(&self) -> TokenStream {
        let visibility = &self.item_fn.vis;
        let name = self.item_fn.sig.ident.to_string();
        let fn_name = format_ident!("register_{}", name.to_snake_case());
        let wc_name = self.web_component_name();
        let tag = &self.tag;
        quote! {
            #visibility fn #fn_name() {
                ::dioxus_web_component::register_dioxus_web_component::<#wc_name>(#tag);
            }
        }
    }

    pub fn web_component(&self) -> TokenStream {
        let visibility = &self.item_fn.vis;
        let name = self.web_component_name();

        let mut attributes = vec![];
        attributes.extend(self.attributes.iter().map(Attribute::struct_attribute));
        attributes.extend(self.events.iter().map(Event::struct_attribute));

        quote! {
            #[derive(Clone, Copy, PartialEq)]
            #visibility struct #name {
                #(#attributes),*
            }
        }
    }

    pub fn impl_web_component_watch(&self) -> TokenStream {
        let name = self.web_component_name();
        let attribute_patterns = self
            .attributes
            .iter()
            .map(Attribute::pattern_attribute_changed);

        quote! {
            impl #name {
                async fn watch(mut self, rx: ::dioxus_web_component::Receiver<::dioxus_web_component::Message>) {
                    while let Ok(msg) = rx.recv().await {
                        if let ::dioxus_web_component::Message::AttributeChanged { name, new_value } = msg {
                            match name.as_str() {
                                #(#attribute_patterns)*
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn web_component_style(&self) -> TokenStream {
        let css = if let Some(css_path) = &self.css {
            quote! {
                let css = include_str!(#css_path);
                let css = ::std::borrow::Cow::Borrowed(css);
                ::dioxus_web_component::InjectedStyle::Css(css)
            }
        } else {
            quote!()
        };

        let stylesheet = if let Some(stylesheet) = &self.stylesheet {
            quote! {
                let url = ::std::borrow::Cow::Borrowed(#stylesheet);
                ::dioxus_web_component::InjectedStyle::Stylesheet(url)
            }
        } else {
            quote!()
        };

        match (&self.css, &self.stylesheet) {
            (None, None) => quote! {
                ::dioxus_web_component::InjectedStyle::default()
            },
            (None, Some(_)) => quote! {
                #stylesheet
            },
            (Some(_), None) => quote! {
                #css
            },
            (Some(_), Some(_)) => quote! {
                let css = #css;
                let stylesheet = #stylesheet;
                ::dioxus_web_component::InjectedStyle::Multiple(vec![stylesheet, css])
            },
        }
    }

    pub fn impl_web_component(&self) -> TokenStream {
        let name = &self.item_fn.sig.ident;
        let wc_name = self.web_component_name();
        let attribute_names = self.attributes.iter().map(|attr| &attr.name);
        let attribute_instances = self.attributes.iter().map(Attribute::new_instance);
        let event_instances = self.events.iter().map(Event::new_instance);

        let instance_name = format_ident!("{}", wc_name.to_string().to_snake_case());

        let mut all_idents = vec![];
        all_idents.extend(self.attributes.iter().map(|attr| attr.ident.clone()));
        all_idents.extend(self.events.iter().map(|evt| evt.ident.clone()));

        let mut all_rsx_attributes = vec![];
        all_rsx_attributes.extend(self.attributes.iter().map(Attribute::rsx_attribute));
        all_rsx_attributes.extend(self.events.iter().map(|evt| evt.ident.to_token_stream()));

        let style = self.web_component_style();

        quote! {
            impl ::dioxus_web_component::DioxusWebComponent for #wc_name {
                fn style() -> ::dioxus_web_component::InjectedStyle {
                    #style
                }

                fn attributes() -> &'static [&'static str] {
                    &[
                        #(#attribute_names),*
                    ]
                }

                #[allow(clippy::default_trait_access)]
                #[allow(clippy::clone_on_copy)]
                fn element() -> Element {
                    let ::dioxus_web_component::Context { rx, event_target } = ::dioxus::prelude::use_context();

                    #(#attribute_instances)*
                    #(#event_instances)*

                    ::dioxus::prelude::use_context_provider(|| #wc_name {
                        #(#all_idents),*
                    });
                    let #instance_name = ::dioxus::prelude::use_context::<#wc_name>();

                    let _ = ::dioxus::prelude::use_coroutine::<(), _, _>(|_| #instance_name.watch(rx));

                    rsx! {
                        #name {
                            #(#all_rsx_attributes)*
                        }
                    }
                }

            }
        }
    }

    fn web_component_name(&self) -> Ident {
        let name = &self.item_fn.sig.ident;
        format_ident!("{name}WebComponent")
    }
}

impl Debug for WebComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WebComponent")
            .field("tag", &self.tag)
            .field("css", &self.css)
            .field("stylesheet", &self.stylesheet)
            .field("attributes", &self.attributes)
            .field("events", &self.events)
            .field("item_fn", &self.item_fn.sig.to_token_stream().to_string())
            .finish()
    }
}
