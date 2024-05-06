#![doc = include_str!("../README.md")]
#![allow(clippy::multiple_crate_versions)]

use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

mod web_component;
pub(crate) use self::web_component::WebComponent;

mod attribute;
pub(crate) use self::attribute::Attribute;

mod event;
pub(crate) use self::event::Event;

/// Proc macro to create the web component glue
#[proc_macro_attribute]
pub fn web_component(args: TokenStream, input: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(input as ItemFn);

    let wc = match WebComponent::parse(args.into(), item) {
        Ok(wc) => wc,
        Err(err) => {
            return TokenStream::from(err.write_errors());
        }
    };
    // println!("🌀 {wc:#?}");

    // TODO checks
    // * invalid tag name
    //   tag: https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/define#valid_custom_element_names
    //
    // * untagged arguments
    // * (invalid types)

    let dioxus_component = wc.dioxus_component();
    let register_fn = wc.register_fn();
    let web_component = wc.web_component();
    let impl_web_component_watch = wc.impl_web_component_watch();
    let impl_web_component = wc.impl_web_component();

    proc_macro::TokenStream::from(quote! {
        #dioxus_component
        #register_fn
        #web_component
        #impl_web_component_watch
        #impl_web_component
    })
}
