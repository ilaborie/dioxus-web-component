#![allow(clippy::min_ident_chars)]

use std::fmt::Debug;

use darling::{Error, FromMeta};
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
    pub ty: Type,
    pub web_event_name: String,
    pub can_bubble: bool,
    pub cancelable: bool,
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
    pub(super) fn parse(attr: &Attribute, ident: Ident, ty: Type) -> Result<Self, Error> {
        let receiver = if let Meta::List(_) = &attr.meta {
            EventReceiver::from_meta(&attr.meta)?
        } else {
            EventReceiver::default()
        };

        let web_event_name = if let Some(name) = receiver.name {
            name
        } else {
            ident
                .unraw()
                .to_string()
                .trim_start_matches("on_")
                .to_string()
        };

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

    pub(super) fn new_instance(&self) -> TokenStream {
        let Self {
            ident,
            web_event_name,
            can_bubble,
            cancelable,
            ..
        } = &self;

        // Events
        // let on_click = custom_event_handler(event_target, "click", CustomEventOptions::default());
        quote! {
            let #ident = ::dioxus_web_component::custom_event_handler(
                event_target,
                #web_event_name,
                ::dioxus_web_component::CustomEventOptions {
                    can_bubble: #can_bubble,
                    cancelable: #cancelable,
                });
        }
    }
}
