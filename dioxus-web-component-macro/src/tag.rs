use std::fmt::{self, Display};
use std::str::FromStr;

use darling::{Error, FromMeta};

#[derive(Debug)]
pub enum InvalidTagError {
    Empty,
    InvalidStartingLetter(String),
    NoHyphen(String),
    HasUpperCase(char, String),
    InvalidChar(char, String),
    ForbiddenName(String),
}

impl Display for InvalidTagError {
    #[allow(clippy::min_ident_chars)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "need a non-empty custom element tag"),
            Self::InvalidStartingLetter(tag) => {
                write!(
                    f,
                    "a custom element tag should start with an ASCII lower case letter (a..z), having \"{tag}\""
                )
            }
            Self::NoHyphen(tag) => write!(
                f,
                "a custom element tag should contains an hyphen '-', having \"{tag}\""
            ),
            Self::HasUpperCase(ch, tag) => {
                write!(
                    f,
                    "a custom element cannot contains an ASCII upper case letter, having \"{tag}\" containing '{ch}'"
                )
            }
            Self::InvalidChar(ch, tag) => write!(
                f,
                "invalid char for a custom element tag \"{tag}\" containing '{ch}'"
            ),
            Self::ForbiddenName(s) => write!(f, "this custom element tag is reserved \"{s}\""),
        }
    }
}

const FORBIDDEN_NAMES: &[&str] = &[
    "annotation-xml",
    "color-profile",
    "font-face",
    "font-face-src",
    "font-face-uri",
    "font-face-format",
    "font-face-name",
    "missing-glyph",
];

#[derive(Debug)]
pub struct Tag(pub String);

impl FromStr for Tag {
    type Err = InvalidTagError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        check_tag(value)?;
        Ok(Self(value.to_owned()))
    }
}

impl FromMeta for Tag {
    fn from_string(value: &str) -> darling::Result<Self> {
        Tag::from_str(value).map_err(Error::custom)
    }
}

/// Check the tag validity
///
/// See  [MDN - Valid custom element names]()
///
/// # Errors
///
/// Fail if the tag is invalid.
fn check_tag(tag: &str) -> Result<(), InvalidTagError> {
    // not empty
    let Some(start) = tag.chars().next() else {
        return Err(InvalidTagError::Empty);
    };
    // start with an ASCII lower letter (a..=z)
    if !start.is_ascii_lowercase() {
        return Err(InvalidTagError::InvalidStartingLetter(tag.to_owned()));
    }
    // contains a hyphen
    if !tag.contains('-') {
        return Err(InvalidTagError::NoHyphen(tag.to_owned()));
    }
    // no ASCII uppercase
    let search = tag.chars().find(char::is_ascii_uppercase);
    if let Some(invalid_char) = search {
        return Err(InvalidTagError::HasUpperCase(invalid_char, tag.to_owned()));
    }
    // avoid some chars
    let search = tag.chars().skip(1).find(|ch| !valid_chars(*ch));
    if let Some(invalid_char) = search {
        return Err(InvalidTagError::InvalidChar(invalid_char, tag.to_owned()));
    }
    // Forbidden
    let search = FORBIDDEN_NAMES.iter().find(|name| **name == tag);
    if let Some(name) = search {
        return Err(InvalidTagError::ForbiddenName((*name).to_string()));
    }

    Ok(())
}

// See <https://html.spec.whatwg.org/multipage/custom-elements.html#valid-custom-element-name>
fn valid_chars(ch: char) -> bool {
    matches!(ch, '-'
        | '.'
        | '0'..='9'
        | 'a'..='z'
        | '\u{00B7}'
        | '\u{00C0}'..='\u{00D6}'
        | '\u{00D8}'..='\u{00F6}'
        | '\u{00F8}'..='\u{037D}'
        | '\u{037F}'..='\u{1FFF}'
        | '\u{200C}'..='\u{200D}'
        | '\u{203F}'..='\u{2040}'
        | '\u{2070}'..='\u{218F}'
        | '\u{2C00}'..='\u{2FEF}'
        | '\u{3001}'..='\u{D7FF}'
        | '\u{F900}'..='\u{FDCF}'
        | '\u{FDF0}'..='\u{FFFD}'
        | '\u{10000}'..='\u{EFFFF}'
    )
}

#[cfg(test)]
mod tests {
    use assert2::let_assert;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("a-a")]
    #[case("my-custom-tag")]
    #[case("i-love-🦀")]
    fn should_accept_valid_tag(#[case] tag: &str) {
        let result = check_tag(tag);
        let_assert!(Ok(()) = result);
    }

    #[rstest]
    #[case::empty("")]
    #[case::start_not_letter("-")]
    #[case::start_not_letter("1")]
    #[case::start_not_letter("_")]
    #[case::uppercase("my-CustomTag")]
    #[case::char("my-custom tag")]
    #[case::forbidden("annotation-xml")]
    #[case::forbidden("color-profile")]
    #[case::forbidden("font-face")]
    #[case::forbidden("font-face-src")]
    #[case::forbidden("font-face-uri")]
    #[case::forbidden("font-face-format")]
    #[case::forbidden("font-face-name")]
    #[case::forbidden("missing-glyph")]
    fn should_reject_invalid_tag(#[case] tag: &str) {
        let result = check_tag(tag);
        let_assert!(Err(_) = result);
    }
}
