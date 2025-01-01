#![doc = include_str!("../README.md")]
#![allow(clippy::multiple_crate_versions)]

use proc_macro::TokenStream;
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

#[doc = include_str!("./doc.md")]
#[proc_macro_attribute]
pub fn web_component(args: TokenStream, input: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(input as ItemFn);

    let mut errors = darling::Error::accumulator();
    let wc = WebComponent::parse(args.into(), item, &mut errors);
    let result = wc.generate(&mut errors);

    if let Err(err) = errors.finish() {
        return TokenStream::from(err.write_errors());
    }

    proc_macro::TokenStream::from(result)
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use assert2::let_assert;
    use syn::ItemFn;

    #[test]
    fn should_parse_multiple_events() {
        let_assert!(Ok(args) = "".parse());
        let input = "fn MyWebComponent(
     #[event] on_event: EventHandler<i64>,
     #[event] on_snake_evt: EventHandler<bool>,
) -> Element {
    rsx!()
}";
        let item = syn::parse_str::<ItemFn>(input).expect("valid rust code");

        let mut errors = darling::Error::accumulator();
        let wc = WebComponent::parse(args, item, &mut errors);

        let tokens = wc.generate(&mut errors);
        let syntax_tree = syn::parse_file(&tokens.to_string()).expect("a file");
        let formatted = prettyplease::unparse(&syntax_tree);
        insta::assert_snapshot!(formatted);

        // insta::assert_debug_snapshot!(tokens);
        // insta::assert_snapshot!(tokens);

        let errors = errors.finish();
        errors.expect("no errors");
    }
}
