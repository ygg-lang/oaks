#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![allow(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

//! Toml support for the Oak language framework.

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;
/// Error handling module.
pub mod errors;

/// Syntax kind module.
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

// Re-export main types for convenience
pub use crate::{
    ast::TomlRoot,
    builder::TomlBuilder,
    language::TomlLanguage,
    lexer::{TomlLexer, token_type::TomlTokenKind as TomlSyntaxKind},
    parser::TomlParser,
};
pub use oak_core::{Builder, TokenType};

/// Highlighter implementation.
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::TomlHighlighter;

/// Parses a TOML string.
pub fn parse(toml: &str) -> Result<crate::ast::TomlRoot, String> {
    use oak_core::{Builder, parser::session::ParseSession, source::SourceText};
    let language = TomlLanguage::default();
    let builder = TomlBuilder::new(&language);
    let source = SourceText::new(toml.to_string());
    let mut cache = ParseSession::default();
    let result = builder.build(&source, &[], &mut cache);
    result.result.map_err(|e| format!("{:?}", e))
}

/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::TomlLanguageService;
#[cfg(feature = "oak-pretty-print")]
pub use crate::lsp::formatter::TomlFormatter;

#[cfg(feature = "mcp")]
// pub use crate::mcp::serve_toml_mcp;

/// Deserializes from a string.
#[cfg(feature = "serde")]
pub fn from_str<T: serde::de::DeserializeOwned>(s: &str) -> Result<T, toml::de::Error> {
    toml::from_str(s)
}
pub use crate::parser::element_type::TomlElementType as ElementType;
