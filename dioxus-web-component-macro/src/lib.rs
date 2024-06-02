#![doc = include_str!("../README.md")]
#![allow(clippy::multiple_crate_versions)]

use darling::ast::NestedMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

mod web_component;
pub(crate) use self::web_component::WebComponent;

mod parameter;
pub(crate) use self::parameter::Parameter;

mod attribute;
pub(crate) use self::attribute::Attribute;

mod properties;
pub(crate) use self::properties::Property;

mod event;
pub(crate) use self::event::Event;

pub(crate) mod tag;

/// Proc macro to create the web component glue
#[proc_macro_attribute]
pub fn web_component(args: TokenStream, input: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(input as ItemFn);

    let mut errors = darling::Error::accumulator();
    let attr_args = errors
        .handle(NestedMeta::parse_meta_list(args.into()).map_err(Into::into))
        .unwrap_or_default();

    let wc = WebComponent::parse(&attr_args, item, &mut errors);
    let dioxus_component = wc.dioxus_component();
    let register_fn = wc.register_fn();
    let web_component = wc.web_component();
    let impl_web_component = wc.impl_dioxus_web_component();
    let builder_fn = wc.builder_fn();
    let typescript = wc.typescript(&mut errors);

    if let Err(err) = errors.finish() {
        return TokenStream::from(err.write_errors());
    }

    proc_macro::TokenStream::from(quote! {
        #dioxus_component
        #register_fn
        #web_component
        #impl_web_component
        #builder_fn
        #typescript
    })
}
