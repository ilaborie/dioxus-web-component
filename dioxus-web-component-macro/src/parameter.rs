use darling::error::Accumulator;
use proc_macro2::TokenStream;
use quote::ToTokens as _;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{FnArg, Ident, Pat, PatIdent, PatType, Type};

use crate::{Attribute, Event, Property};

#[derive(Debug)]
pub enum Parameter {
    Attribute(Attribute, Option<Property>),
    Property(Property),
    Event(Event),
}

impl Parameter {
    pub fn parse(errors: &mut Accumulator, inputs: &mut Punctuated<FnArg, Comma>) -> Vec<Self> {
        inputs
            .iter_mut()
            .filter_map(|arg| ParameterInfo::build(errors, arg))
            .map(ParameterInfo::into_parameter)
            .collect()
    }
}

impl Parameter {
    pub fn struct_attribute(&self) -> TokenStream {
        match self {
            Self::Attribute(attr, _) => attr.struct_attribute(),
            Self::Property(prop) => prop.struct_attribute(),
            Self::Event(evt) => evt.struct_attribute(),
        }
    }

    pub fn new_instance(&self, shared: &Ident) -> TokenStream {
        match self {
            Self::Attribute(attr, _) => attr.new_instance(),
            Self::Property(prop) => prop.new_instance(),
            Self::Event(evt) => evt.new_instance(shared),
        }
    }

    pub fn ident(&self) -> Ident {
        match self {
            Self::Attribute(attr, _) => attr.ident.clone(),
            Self::Property(prop) => prop.ident.clone(),
            Self::Event(evt) => evt.ident.clone(),
        }
    }

    pub fn rsx_attribute(&self) -> TokenStream {
        match self {
            Self::Attribute(attr, _) => attr.rsx_attribute(),
            Self::Property(prop) => prop.rsx_attribute(),
            Self::Event(evt) => evt.ident.to_token_stream(),
        }
    }
}

struct ParameterInfo {
    ident: Ident,
    ty: Type,
    attribute: Option<Attribute>,
    property: Option<Property>,
    event: Option<Event>,
}

impl ParameterInfo {
    fn build(errors: &mut Accumulator, arg: &mut FnArg) -> Option<Self> {
        let FnArg::Typed(arg) = arg else {
            return None;
        };

        let PatType { attrs, pat, ty, .. } = arg;
        let Pat::Ident(PatIdent { ident, .. }) = pat.as_ref() else {
            panic!("Expected an ident, got {pat:#?}");
        };

        let ident = ident.clone();
        let ty = Type::clone(ty);
        let mut result = Self {
            ident,
            ty,
            attribute: None,
            property: None,
            event: None,
        };

        attrs.retain(|attr| result.parse_attribute(errors, attr));

        Some(result)
    }

    fn parse_attribute(&mut self, errors: &mut Accumulator, attr: &syn::Attribute) -> bool {
        if attr.path().is_ident("event") {
            let event = Event::parse(attr, self.ident.clone(), self.ty.clone());
            self.event = errors.handle(event);
            false
        } else if attr.path().is_ident("property") {
            let property = Property::parse(attr, self.ident.clone(), self.ty.clone());
            self.property = errors.handle(property);
            false
        } else if attr.path().is_ident("attribute") {
            let attribute = Attribute::parse(attr, self.ident.clone(), self.ty.clone());
            self.attribute = errors.handle(attribute);
            false
        } else {
            true
        }
    }

    fn into_parameter(self) -> Parameter {
        let Self {
            ident,
            ty,
            attribute,
            property,
            event,
        } = self;

        match (attribute, property, event) {
            (Some(attr), prop, _) => Parameter::Attribute(attr, prop),
            (None, Some(prop), _) => Parameter::Property(prop),
            (None, None, Some(event)) => Parameter::Event(event),
            (None, None, None) => {
                let ty_str = ty.to_token_stream().to_string();
                let is_event = ty_str.starts_with("EventHandler <");
                if is_event {
                    Parameter::Event(Event::new(ident, ty))
                } else {
                    Parameter::Attribute(Attribute::new(ident, ty), None)
                }
            }
        }
    }
}
