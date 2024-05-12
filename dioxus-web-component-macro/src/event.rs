#![allow(clippy::min_ident_chars)]

use std::fmt::Debug;

use darling::{Error, FromMeta};
use heck::ToKebabCase;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::ext::IdentExt;
use syn::{Attribute, Meta, Type};

#[derive(Debug, FromMeta, Default)]
pub struct EventReceiver {
    name: Option<String>,
    no_bubble: Option<bool>,
    no_cancel: Option<bool>,
}

pub struct Event {
    pub ident: Ident,
    ty: Type,
    web_event_name: Option<String>,
    can_bubble: bool,
    cancelable: bool,
}

impl Debug for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Event")
            .field("ident", &self.ident.to_string())
            .field("ty", &self.ty.to_token_stream().to_string())
            .field("web_event_name", &self.web_event_name)
            .field("can_bubble", &self.can_bubble)
            .field("cancelable", &self.cancelable)
            .finish()
    }
}

impl Event {
    pub(super) fn new(ident: Ident, ty: Type) -> Self {
        Self {
            ident,
            ty,
            web_event_name: None,
            can_bubble: true,
            cancelable: true,
        }
    }

    pub(super) fn parse(attr: &Attribute, ident: Ident, ty: Type) -> Result<Self, Error> {
        let receiver = if let Meta::List(_) = &attr.meta {
            EventReceiver::from_meta(&attr.meta)?
        } else {
            EventReceiver::default()
        };

        let web_event_name = receiver.name;
        let can_bubble = !(receiver.no_bubble.unwrap_or_default());
        let cancelable = !(receiver.no_cancel.unwrap_or_default());

        let result = Self {
            ident,
            ty,
            web_event_name,
            can_bubble,
            cancelable,
        };
        Ok(result)
    }
}

impl Event {
    pub(super) fn struct_attribute(&self) -> TokenStream {
        let Self { ident, ty, .. } = &self;
        quote! {
            #ident : #ty
        }
    }

    fn web_event_name(&self) -> String {
        self.web_event_name.clone().unwrap_or_else(|| {
            self.ident
                .unraw()
                .to_string()
                .trim_start_matches("on_")
                .trim_start_matches("on")
                .to_kebab_case()
        })
    }

    pub(super) fn new_instance(&self) -> TokenStream {
        let Self {
            ident,
            can_bubble,
            cancelable,
            ..
        } = &self;

        let web_event_name = self.web_event_name();

        quote! {
            let #ident = ::dioxus_web_component::custom_event_handler(
                event_target.clone(),
                #web_event_name,
                ::dioxus_web_component::CustomEventOptions {
                    can_bubble: #can_bubble,
                    cancelable: #cancelable,
                });
        }
    }
}
