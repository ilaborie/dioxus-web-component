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
struct PropertyReceiver {
    name: Option<String>,
    readonly: Option<bool>,
    initial: Option<Expr>,
    try_from_js: Option<Expr>,
    try_into_js: Option<Expr>,
}

pub(super) struct Property {
    pub ident: Ident,
    ty: Type,
    name: Option<String>,
    readonly: Option<bool>,
    initial: Option<Expr>,
    try_from_js: Option<Expr>,
    try_into_js: Option<Expr>,
}

impl Debug for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Property")
            .field("ident", &self.ident.to_string())
            .field("ty", &self.ty.to_token_stream().to_string())
            .field("name", &self.name)
            .field("readonly", &self.readonly)
            .field("initial", &self.initial.to_token_stream().to_string())
            .field(
                "try_from_js",
                &self.try_from_js.to_token_stream().to_string(),
            )
            .field(
                "try_into_js",
                &self.try_into_js.to_token_stream().to_string(),
            )
            .finish()
    }
}

impl Property {
    // pub(super) fn new(ident: Ident, ty: Type) -> Self {
    //     Self {
    //         ident,
    //         ty,
    //         name: None,
    //         readonly: None,
    //         initial: None,
    //         try_from_js: None,
    //         try_into_js: None,
    //     }
    // }

    pub(super) fn parse(
        attr: &syn::Attribute,
        ident: Ident,
        ty: Type,
    ) -> Result<Self, darling::Error> {
        let receiver = if let Meta::List(_) = &attr.meta {
            PropertyReceiver::from_meta(&attr.meta)?
        } else {
            PropertyReceiver::default()
        };

        let result = Self {
            ident,
            ty,
            name: receiver.name,
            readonly: receiver.readonly,
            initial: receiver.initial,
            try_from_js: receiver.try_from_js,
            try_into_js: receiver.try_into_js,
        };
        Ok(result)
    }
}

impl Property {
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

    pub(super) fn readonly(&self) -> bool {
        self.readonly.unwrap_or_default()
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

    fn try_from_js_value(&self) -> TokenStream {
        self.try_from_js.as_ref().map_or_else(
            || {
                quote! {
                    |value| value.try_into()
                }
            },
            ToTokens::to_token_stream,
        )
    }

    fn try_into_js_value(&self) -> TokenStream {
        self.try_into_js.as_ref().map_or_else(
            || {
                quote! {
                    |value| value.try_into()
                }
            },
            ToTokens::to_token_stream,
        )
    }

    pub(super) fn new_property(&self) -> TokenStream {
        let name = self.name().to_string();
        let readonly = self.readonly.unwrap_or_default();

        quote! {
            ::dioxus_web_component::Property::new(#name, #readonly)
        }
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

    pub(super) fn pattern_set_property(&self) -> TokenStream {
        let ident = &self.ident;
        let name = self.name();
        let try_from_js = self.try_from_js_value();

        quote! {
            #name => {
                if let Ok(new_value) = Ok(value).and_then(#try_from_js) {
                    self.#ident.set(new_value);
                }
            }
        }
    }

    pub(super) fn pattern_get_property(&self) -> TokenStream {
        let ident = &self.ident;
        let name = self.name();
        let try_into_js = self.try_into_js_value();

        quote! {
            #name => {
                Ok(self.#ident.read().clone())
                    .and_then(#try_into_js)
                    .unwrap_or(::wasm_bindgen::JsValue::undefined())
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
