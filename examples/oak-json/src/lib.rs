#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![feature(portable_simd)]
#![allow(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Json support for the Oak language framework.

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;

// pub mod formatter;

/// Type definitions module.
/// Language configuration module.
pub mod language;
/// Lexer module.
pub mod lexer;
/// LSP module.
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// MCP module.
#[cfg(feature = "mcp")]
pub mod mcp;

/// Parser module.
pub mod parser;
/// Serde support module.
#[cfg(feature = "serde")]
pub mod serde;

pub use crate::{
    ast::{JsonRoot, JsonValue},
    builder::JsonBuilder,
    language::JsonLanguage,
    lexer::JsonLexer,
    parser::JsonParser,
};

/// Highlighter implementation.
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::JsonHighlighter;

#[cfg(feature = "serde")]
pub use crate::serde::{from_value, to_value};

#[cfg(feature = "serde")]
pub fn to_string<T: ::serde::Serialize>(value: &T) -> Result<String, String> {
    let json_value = to_value(value).map_err(|e| e.to_string())?;
    Ok(json_value.to_string())
}

#[cfg(feature = "serde")]
pub fn from_str<T: ::serde::de::DeserializeOwned>(s: &str) -> Result<T, String> {
    let json_value = parse(s)?;
    from_value(json_value).map_err(|e| e.to_string())
}

pub fn parse(json: &str) -> Result<crate::ast::JsonValue, String> {
    use oak_core::{Builder, parser::session::ParseSession, source::SourceText};
    let language = JsonLanguage::default();
    let builder = JsonBuilder::new(&language);
    let source = SourceText::new(json.to_string());
    let mut cache = ParseSession::default();
    let result = builder.build(&source, &[], &mut cache);
    result.result.map(|root| root.value).map_err(|e| format!("{:?}", e))
}

pub use oak_macros::json;

/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::JsonLanguageService;
// #[cfg(feature = "oak-pretty-print")]
// pub use crate::lsp::formatter::JsonFormatter;

/// MCP service implementation.
#[cfg(feature = "mcp")]
pub use crate::mcp::serve_json_mcp;
pub use lexer::token_type::JsonTokenType;
pub use parser::element_type::JsonElementType;
