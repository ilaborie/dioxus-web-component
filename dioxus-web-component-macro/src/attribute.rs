#![allow(clippy::min_ident_chars)]

use std::fmt::Debug;

use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::ext::IdentExt;
use syn::{Meta, Type};

#[derive(Debug, FromMeta, Default)]
struct AttributeReceiver {
    name: Option<String>,
    option: Option<bool>,
    // initial: Option<Path>,
    // parse: Option<Path>,
}

pub(super) struct Attribute {
    pub ident: Ident,
    pub ty: Type,
    pub name: String,
    pub is_option: bool,
    // TODO Default value
    // TODO parse Option<String> to ty
}

impl Debug for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Attribute")
            .field("ident", &self.ident.to_string())
            .field("ty", &self.ty.to_token_stream().to_string())
            .field("name", &self.name)
            .field("is_option", &self.is_option)
            .finish()
    }
}

impl Attribute {
    pub(super) fn parse(
        attr: &syn::Attribute,
        ident: Ident,
        ty: Type,
    ) -> Result<Self, darling::Error> {
        let receiver = if let Meta::List(_) = &attr.meta {
            AttributeReceiver::from_meta(&attr.meta)?
        } else {
            AttributeReceiver::default()
        };

        let name = if let Some(name) = receiver.name {
            name
        } else {
            ident.unraw().to_string()
        };

        let is_option = if let Some(option) = receiver.option {
            option
        } else {
            let ty_str = ty.to_token_stream().to_string();
            ty_str.contains("Option<")
        };

        let result = Self {
            ident,
            ty,
            name,
            is_option,
        };
        Ok(result)
    }
}

impl Attribute {
    pub(super) fn struct_attribute(&self) -> TokenStream {
        let Self { ident, ty, .. } = &self;
        quote! {
            #ident : ::dioxus::prelude::Signal<#ty>
        }
    }

    pub(super) fn new_instance(&self) -> TokenStream {
        let Self { ident, .. } = &self;
        quote! {
            let #ident = ::dioxus::prelude::use_signal(Default::default);
        }
    }

    pub(super) fn pattern_attribute_changed(&self) -> TokenStream {
        let Self {
            ident,
            name,
            is_option,
            ..
        } = &self;

        if *is_option {
            quote! {
                #name => {
                    let value = new_value.and_then(|it| it.parse().ok());
                    self.#ident.set(value);
                }
            }
        } else {
            quote! {
                #name => {
                    let value = new_value.and_then(|it| it.parse().ok()).unwrap_or_default();
                    self.#ident.set(value);
                }
            }
        }
    }

    pub(super) fn rsx_attribute(&self) -> TokenStream {
        let Self {
            ident, is_option, ..
        } = &self;

        if *is_option {
            quote! {
                #ident: #ident().clone(),
            }
        } else {
            quote! {
                #ident,
            }
        }
    }
}
