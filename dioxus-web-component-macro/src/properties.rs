#![allow(clippy::min_ident_chars)]

use std::borrow::Cow;
use std::fmt::Debug;

use darling::error::Accumulator;
use darling::FromMeta;
use heck::{ToKebabCase, ToLowerCamelCase};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::ext::IdentExt;
use syn::{Expr, GenericArgument, Meta, PathArguments, PathSegment, Type};

#[derive(Debug, FromMeta, Default)]
struct PropertyReceiver {
    name: Option<String>,
    readonly: Option<bool>,
    initial: Option<Expr>,
    try_from_js: Option<Expr>,
    try_into_js: Option<Expr>,
    js_type: Option<String>,
}

#[derive(Clone)]
pub(super) struct Property {
    pub ident: Ident,
    ty: Type,
    name: Option<String>,
    readonly: Option<bool>,
    initial: Option<Expr>,
    try_from_js: Option<Expr>,
    try_into_js: Option<Expr>,
    js_type: Option<String>,
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
            .field("js_type", &self.js_type)
            .finish()
    }
}

impl Property {
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
            js_type: receiver.js_type,
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
        let name = self.js_name();
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
                let value = self.#ident.read().clone();
                Ok(value)
                    .and_then(#try_into_js)
                    .unwrap_or_else(|err| {
                        ::dioxus::logger::tracing::warn!("get {} conversion error {:?}, return undefined", #name, err);
                        wasm_bindgen::JsValue::undefined()
                    })
            }
        }
    }

    pub(super) fn rsx_attribute(&self) -> TokenStream {
        let ident = &self.ident;

        quote! {
            #ident: #ident().clone(),
        }
    }

    pub(super) fn js_name(&self) -> String {
        self.name().to_lower_camel_case()
    }

    pub(super) fn js_type(&self, errors: &mut Accumulator) -> String {
        if let Some(ty) = &self.js_type {
            return ty.clone();
        }
        extract_js_type(&self.ty, errors)
    }
}

#[allow(clippy::print_stderr)]
// TODO add a warning
// see https://github.com/rust-lang/rust/issues/54140
fn extract_js_type(ty: &Type, errors: &mut Accumulator) -> String {
    let result = match ty {
        Type::Array(arr) => {
            let inner = extract_js_type(&arr.elem, errors);
            Some(format!("Array<{inner}>"))
        }
        Type::Group(grp) => Some(extract_js_type(&grp.elem, errors)),
        Type::Never(_) => Some("never".to_string()),
        Type::Paren(paren) => Some(extract_js_type(&paren.elem, errors)),
        Type::Tuple(tpl) => {
            let inner = tpl
                .elems
                .iter()
                .map(|ty| extract_js_type(ty, errors))
                .collect::<Vec<_>>();
            Some(format!("[{}]", inner.join(", ")))
        }
        Type::Path(path) if path.path.segments.len() == 1 =>
        {
            #[allow(clippy::indexing_slicing)]
            extract_path_segment_js_type(&path.path.segments[0], errors)
        }
        // TODO maybe detect some predefine path like std collections
        // Other cases are not handled
        _ => None,
    };

    result.unwrap_or_else(|| {
        let msg = format!(
            "Oops, we cannot define the Javascript type for {ty:?}.
Use the explicit `js_type` attribute on the property to define the expected type.
Or, disable the typescript generation with `#[web_component(no_typescript = true,  ...)]`"
        );
        errors.push(darling::Error::custom(msg).with_span(ty));
        "any".to_string()
    })
}

#[allow(clippy::match_same_arms)]
fn extract_path_segment_js_type(segment: &PathSegment, errors: &mut Accumulator) -> Option<String> {
    let ident = segment.ident.to_string();
    match ident.to_string().as_str() {
        "bool" => Some("boolean".to_string()),
        "u8" | "u16" | "u32" | "i16" | "i32" | "i64" | "f32" | "f64" => Some("number".to_string()),
        // it's probably better to have a number for usize/isize
        "isize" | "usize" => Some("number".to_string()),
        "u64" => Some("bigint".to_string()),
        "char" | "String" => Some("string".to_string()),
        "Option" => {
            let PathArguments::AngleBracketed(generics) = &segment.arguments else {
                return None;
            };
            let Some(GenericArgument::Type(inner_ty)) = generics.args.first() else {
                return None;
            };

            let inner = extract_js_type(inner_ty, errors);
            Some(format!("{inner} | null"))
        }
        "Vec" => {
            let PathArguments::AngleBracketed(generics) = &segment.arguments else {
                return None;
            };
            let Some(GenericArgument::Type(inner_ty)) = generics.args.first() else {
                return None;
            };

            let inner = extract_js_type(inner_ty, errors);
            Some(format!("Array<{inner}>"))
        }
        _ => None,
    }
}
