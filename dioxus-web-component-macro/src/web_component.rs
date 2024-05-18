#![allow(clippy::min_ident_chars)]

use core::panic;
use std::fmt::Debug;

use darling::ast::NestedMeta;
use darling::{Error, FromMeta};
use heck::{ToKebabCase, ToSnakeCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::ext::IdentExt;
use syn::{Expr, FnArg, Ident, ItemFn, Pat, PatIdent, PatType, Type};

use crate::tag::Tag;
use crate::{Attribute, Event};

#[derive(Debug, FromMeta)]
struct WebComponentReceiver {
    tag: Option<Tag>,
    style: Option<Expr>,
}

pub(crate) struct WebComponent {
    tag: Tag,
    style: Option<Expr>,
    attributes: Vec<Attribute>,
    events: Vec<Event>,
    item_fn: ItemFn,
}

impl WebComponent {
    pub(crate) fn parse(args: TokenStream, mut item_fn: ItemFn) -> Result<Self, Error> {
        let attr_args = NestedMeta::parse_meta_list(args)?;
        let mut errors = Error::accumulator();

        let (tag, style) = errors
            .handle(WebComponentReceiver::from_list(&attr_args))
            .map(|wc| (wc.tag, wc.style))
            .unwrap_or_default();

        let tag = if let Some(tag) = tag {
            tag
        } else {
            let tag = item_fn.sig.ident.unraw().to_string().to_kebab_case();
            errors
                .handle_in(|| {
                    tag.parse()
                        .map_err(|err| Error::custom(err).with_span(&item_fn.sig.ident))
                })
                .unwrap_or(Tag(tag))
        };

        let mut events = vec![];
        let mut attributes = vec![];
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

            let mut has_attribute = false;
            // Parse argument attributes (keep attribute that is not handled)
            attrs.retain(|attr| {
                if attr.path().is_ident("event") {
                    has_attribute = true;
                    let event = Event::parse(attr, ident.clone(), ty.clone());
                    if let Some(event) = errors.handle(event) {
                        events.push(event);
                    }
                    false
                } else if attr.path().is_ident("attribute") {
                    has_attribute = true;
                    let attr = Attribute::parse(attr, ident.clone(), ty.clone());
                    if let Some(attr) = errors.handle(attr) {
                        attributes.push(attr);
                    }
                    false
                } else {
                    true
                }
            });

            if !has_attribute {
                let ty_str = ty.to_token_stream().to_string();
                let is_event = ty_str.starts_with("EventHandler <");
                if is_event {
                    events.push(Event::new(ident, ty));
                } else {
                    attributes.push(Attribute::new(ident, ty));
                }
            }
        }

        errors.finish()?;

        let result = Self {
            tag,
            style,
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
        let tag = &self.tag.0;
        // TODO Default tag should be kebab-case
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

    pub fn impl_web_component(&self) -> TokenStream {
        let name = &self.item_fn.sig.ident;
        let wc_name = self.web_component_name();
        let attribute_names = self.attributes.iter().map(|attr| attr.name());
        let attribute_instances = self.attributes.iter().map(Attribute::new_instance);
        let event_instances = self.events.iter().map(Event::new_instance);

        let instance_name = format_ident!("{}", wc_name.to_string().to_snake_case());

        let mut all_idents = vec![];
        all_idents.extend(self.attributes.iter().map(|attr| attr.ident.clone()));
        all_idents.extend(self.events.iter().map(|evt| evt.ident.clone()));

        let mut all_rsx_attributes = vec![];
        all_rsx_attributes.extend(self.attributes.iter().map(Attribute::rsx_attribute));
        all_rsx_attributes.extend(self.events.iter().map(|evt| evt.ident.to_token_stream()));

        let style = self
            .style
            .as_ref()
            .map(|style| {
                quote! {
                        fn style() -> ::dioxus_web_component::InjectedStyle {
                            #style
                        }
                }
            })
            .unwrap_or_default();

        quote! {
            impl ::dioxus_web_component::DioxusWebComponent for #wc_name {
                #style

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
            .field("style", &self.style.to_token_stream().to_string())
            .field("attributes", &self.attributes)
            .field("events", &self.events)
            .field("item_fn", &self.item_fn.sig.to_token_stream().to_string())
            .finish()
    }
}
