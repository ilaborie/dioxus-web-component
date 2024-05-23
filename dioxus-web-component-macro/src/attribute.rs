#![allow(clippy::min_ident_chars)]

use std::borrow::Cow;
use std::fmt::Debug;

use darling::FromMeta;
use heck::ToKebabCase as _;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::ext::IdentExt;
use syn::{Expr, Meta, Type};

#[derive(Debug, FromMeta, Default)]
struct AttributeReceiver {
    name: Option<String>,
    option: Option<bool>,
    initial: Option<Expr>,
    parse: Option<Expr>,
}

pub(super) struct Attribute {
    pub ident: Ident,
    ty: Type,
    name: Option<String>,
    is_option: Option<bool>,
    initial: Option<Expr>,
    parse: Option<Expr>,
}

impl Debug for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Attribute")
            .field("ident", &self.ident.to_string())
            .field("ty", &self.ty.to_token_stream().to_string())
            .field("name", &self.name)
            .field("is_option", &self.is_option)
            .field("initial", &self.initial.to_token_stream().to_string())
            .field("parse", &self.parse.to_token_stream().to_string())
            .finish()
    }
}

impl Attribute {
    pub(super) fn new(ident: Ident, ty: Type) -> Self {
        Self {
            ident,
            ty,
            name: None,
            is_option: None,
            initial: None,
            parse: None,
        }
    }

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

        let result = Self {
            ident,
            ty,
            name: receiver.name,
            is_option: receiver.option,
            initial: receiver.initial,
            parse: receiver.parse,
        };
        Ok(result)
    }
}

impl Attribute {
    pub(super) fn name(&self) -> Cow<str> {
        self.name.as_deref().map_or_else(
            || {
                let name = self.ident.unraw().to_string();
                let name = name.as_str().to_kebab_case();
                Cow::Owned(name)
            },
            Cow::Borrowed,
        )
    }

    fn option(&self) -> bool {
        self.is_option.as_ref().copied().unwrap_or_else(|| {
            let ty_str = self.ty.to_token_stream().to_string();
            ty_str.starts_with("Option <")
        })
    }

    fn initial(&self) -> TokenStream {
        self.initial.as_ref().map_or_else(
            || {
                quote! {
                    ::std::default::Default::default()
                }
            },
            ToTokens::to_token_stream,
        )
    }

    fn parse_value(&self) -> TokenStream {
        self.parse.as_ref().map_or_else(
            || {
                quote! {
                    |value| value.parse().ok()
                }
            },
            ToTokens::to_token_stream,
        )
    }

    pub(super) fn struct_attribute(&self) -> TokenStream {
        let Self { ident, ty, .. } = &self;
        quote! {
            #ident : ::dioxus::prelude::Signal<#ty>
        }
    }

    pub(super) fn new_instance(&self) -> TokenStream {
        let ident = &self.ident;
        let initial = self.initial();
        quote! {
            let #ident = ::dioxus::prelude::use_signal(|| #initial);
        }
    }

    pub(super) fn pattern_attribute_changed(&self) -> TokenStream {
        let ident = &self.ident;
        let name = self.name();
        let parse = self.parse_value();
        let initial = self.initial();

        if self.option() {
            quote! {
                #name => {
                    let value = new_value.and_then(#parse);
                    self.#ident.set(value);
                }
            }
        } else {
            quote! {
                #name => {
                    let value = new_value.and_then(#parse).unwrap_or_else(|| #initial);
                    self.#ident.set(value);
                }
            }
        }
    }

    pub(super) fn rsx_attribute(&self) -> TokenStream {
        let ident = &self.ident;

        quote! {
            #ident: #ident().clone(),
        }
    }
}
