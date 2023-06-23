#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Xml support for the Oak language framework.

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;

// pub mod formatter;
// pub mod highlighter;
/// Type definitions module.
/// Language configuration module.
pub mod language;
/// Lexer module.
pub mod lexer;
/// LSP module.
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;

// pub mod mcp;
/// Parser module.
pub mod parser;
/// Serde serialization module.
#[cfg(feature = "serde")]
pub mod serde;

pub use crate::{
    ast::{XmlNode, XmlValue},
    language::XmlLanguage,
    lexer::{XmlLexer, token_type::XmlTokenType},
    parser::XmlParser,
};

// #[cfg(feature = "oak-highlight")]
// pub use crate::lsp::highlighter::XmlHighlighter;

// /// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::XmlLanguageService;
// #[cfg(feature = "lsp")]
// pub use crate::lsp::formatter::XmlFormatter;

#[cfg(feature = "serde")]
pub use crate::serde::{from_value, to_value};

#[cfg(feature = "serde")]
pub fn to_string<T: ::serde::Serialize>(value: &T) -> Result<String, String> {
    let xml_value = to_value(value).map_err(String::from)?;
    Ok(xml_value.to_string())
}

#[cfg(feature = "serde")]
pub fn from_str<T: ::serde::de::DeserializeOwned>(s: &str) -> Result<T, String> {
    let xml_value = parse(s)?;
    from_value(xml_value).map_err(String::from)
}

pub fn parse(xml: &str) -> Result<XmlValue, String> {
    use crate::builder::XmlBuilder;
    use oak_core::{Builder, parser::session::ParseSession, source::SourceText};
    let builder = XmlBuilder::new();
    let source = SourceText::new(xml.to_string());
    let mut cache = ParseSession::default();
    let result = builder.build(&source, &[], &mut cache);
    result.result.map(|root| root.value).map_err(|e| format!("Parse failed: {:?}, diagnostics: {:?}", e, result.diagnostics))
}

pub use parser::element_type::XmlElementType;
