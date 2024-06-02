#![allow(clippy::min_ident_chars)]

use std::fmt::Debug;

use darling::ast::NestedMeta;
use darling::error::Accumulator;
use darling::{Error, FromMeta};
use heck::{ToKebabCase, ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::ext::IdentExt;
use syn::{Expr, Ident, ItemFn};

use crate::tag::Tag;
use crate::{Attribute, Parameter, Property};

#[derive(Debug, FromMeta)]
struct WebComponentReceiver {
    tag: Option<Tag>,
    style: Option<Expr>,
    no_typescript: Option<bool>,
}

pub(crate) struct WebComponent {
    tag: Tag,
    style: Option<Expr>,
    parameters: Vec<Parameter>,
    item_fn: ItemFn,
    no_typescript: Option<bool>,
}

impl WebComponent {
    pub(crate) fn parse(
        attr_args: &[NestedMeta],
        mut item_fn: ItemFn,
        errors: &mut Accumulator,
    ) -> Self {
        let (tag, style, no_typescript) = errors
            .handle(WebComponentReceiver::from_list(attr_args))
            .map(|wc| (wc.tag, wc.style, wc.no_typescript))
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
                .unwrap_or(Tag::new(tag))
        };

        let parameters = Parameter::parse(errors, &mut item_fn.sig.inputs);

        Self {
            tag,
            style,
            parameters,
            item_fn,
            no_typescript,
        }
    }

    fn attributes(&self) -> impl Iterator<Item = &Attribute> {
        self.parameters.iter().filter_map(|it| match it {
            Parameter::Attribute(attr, _) => Some(attr),
            Parameter::Property(_) | Parameter::Event(_) => None,
        })
    }

    fn properties(&self) -> impl Iterator<Item = &Property> {
        self.parameters.iter().filter_map(|it| match it {
            Parameter::Property(prop) | Parameter::Attribute(_, Some(prop)) => Some(prop),
            Parameter::Attribute(_, None) | Parameter::Event(_) => None,
        })
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
        let attribute_names = self.attributes().map(|attr| attr.name());
        let props = self.properties().map(Property::new_property);
        let style = self.style.as_ref().map_or_else(
            || {
                quote! {
                        ::dioxus_web_component::InjectedStyle::default()
                }
            },
            quote::ToTokens::to_token_stream,
        );
        let tag = &self.tag.to_string();
        let builder_name = self.builder_name();

        quote! {
            #visibility fn #fn_name() {
                let attributes = ::std::vec![
                    #(#attribute_names.to_string()),*
                ];
                let properties = ::std::vec![
                    #(#props),*
                ];
                let style = #style;
                ::dioxus_web_component::register_dioxus_web_component(#tag, attributes, properties, style, #builder_name);
            }
        }
    }

    pub fn web_component(&self) -> TokenStream {
        let visibility = &self.item_fn.vis;
        let name = self.web_component_name();

        let attributes = self.parameters.iter().map(Parameter::struct_attribute);

        quote! {
            #[derive(Clone, Copy)]
            #[allow(dead_code)]
            #visibility struct #name {
                #(#attributes),*
            }
        }
    }

    pub fn impl_dioxus_web_component(&self) -> TokenStream {
        let wc_name = self.web_component_name();
        let attribute_patterns = self.attributes().map(Attribute::pattern_attribute_changed);

        let property_set = self
            .properties()
            .filter(|prop| !prop.readonly())
            .map(Property::pattern_set_property);
        let property_get = self.properties().map(Property::pattern_get_property);

        quote! {
            impl ::dioxus_web_component::DioxusWebComponent for #wc_name {
                #[allow(clippy::single_match)]
                #[allow(clippy::redundant_closure)]
                fn set_attribute(&mut self, attribute: &str, new_value: Option<String>) {
                    match attribute {
                        #(#attribute_patterns)*
                        _ => {}
                    }
                }

                #[allow(clippy::single_match)]
                #[allow(clippy::redundant_closure)]
                fn set_property(&mut self, property: &str, value: ::wasm_bindgen::JsValue) {
                    match property{
                        #(#property_set)*
                        _ => {}
                    }
                }

                #[allow(clippy::single_match)]
                #[allow(clippy::redundant_closure)]
                fn get_property(&mut self, property: &str) -> ::wasm_bindgen::JsValue {
                    match property{
                        #(#property_get)*
                        _ => ::wasm_bindgen::JsValue::undefined(),
                    }
                }
            }
        }
    }

    pub fn builder_fn(&self) -> TokenStream {
        let name = &self.item_fn.sig.ident;
        let builder_name = self.builder_name();
        let wc_name = self.web_component_name();
        let instance_name = format_ident!("__{}", wc_name.to_string().to_snake_case());
        let shared_name = format_ident!("__wc");
        let coroutine_name = format_ident!("__coroutine");

        let instances = self
            .parameters
            .iter()
            .map(|param| param.new_instance(&shared_name));

        let all_idents = self.parameters.iter().map(Parameter::ident);

        let all_rsx_attributes = self.parameters.iter().map(Parameter::rsx_attribute);

        quote! {
            #[allow(clippy::default_trait_access)]
            #[allow(clippy::clone_on_copy)]
            #[allow(clippy::redundant_closure)]
            fn #builder_name() -> ::dioxus::prelude::Element {
                let mut #shared_name = ::dioxus::prelude::use_context::<::dioxus_web_component::Shared>();

                #(#instances)*

                let mut #instance_name = #wc_name {
                    #(#all_idents),*
                };

                let #coroutine_name = ::dioxus::prelude::use_coroutine(move |mut rx| async move {
                    use ::dioxus_web_component::{StreamExt, DioxusWebComponent};
                    while let Some(message) = rx.next().await {
                        #instance_name.handle_message(message);
                    }
                });

                ::dioxus::prelude::use_effect(move || {
                    #shared_name.set_tx(#coroutine_name.tx());
                });

                rsx! {
                    #name {
                        #(#all_rsx_attributes)*
                    }
                }
            }
        }
    }

    fn web_component_name(&self) -> Ident {
        let name = &self.item_fn.sig.ident;
        format_ident!("{name}WebComponent")
    }

    fn builder_name(&self) -> Ident {
        let name = &self.item_fn.sig.ident;
        format_ident!("{}_builder", name.to_string().to_snake_case())
    }

    pub fn typescript(&self, errors: &mut Accumulator) -> TokenStream {
        if self.no_typescript.unwrap_or_default() {
            return quote! {};
        }
        let name = &self.item_fn.sig.ident;
        let const_name = format_ident!("{}_TYPESCRIPT", name.to_string().to_shouty_snake_case());
        let type_name = format!("{}Element", name.to_string().to_upper_camel_case());
        let tag_name = self.tag.to_string();
        let properties = self
            .properties()
            .map(|prop| {
                let name = prop.js_name();
                let ty = prop.js_type(errors);
                if prop.readonly() {
                    format!("readonly {name}: {ty};")
                } else {
                    format!("{name}: {ty};")
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        let definition = format!(
            "
export type {type_name} = HTMLElement & {{
    {properties}
}};

declare global {{
    interface HTMLElementTagNameMap {{
        '{tag_name}': {type_name};
    }}
}}"
        );

        quote! {
            #[::wasm_bindgen::prelude::wasm_bindgen(typescript_custom_section)]
            const #const_name: &str = #definition;
        }
    }
}

impl Debug for WebComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WebComponent")
            .field("tag", &self.tag)
            .field("style", &self.style.to_token_stream().to_string())
            .field("parameters", &self.parameters)
            .field("item_fn", &self.item_fn.sig.to_token_stream().to_string())
            .field("no_typescript", &self.no_typescript)
            .finish()
    }
}
