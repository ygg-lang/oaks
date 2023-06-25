#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Dhall support for the Oak language framework.

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;

/// Kind definition module.
/// Language configuration module.
pub mod language;
/// Lexer module.
pub mod lexer;
/// LSP module.
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;

/// Parser module.
pub mod parser;

pub use crate::{ast::DHallRoot, builder::DHallBuilder, language::DHallLanguage, lexer::DHallLexer, parser::DHallParser};

/// Parses a Dhall string.
pub fn parse(dhall: &str) -> Result<crate::ast::DHallRoot, String> {
    use oak_core::{Builder, parser::session::ParseSession, source::SourceText};
    let language = DHallLanguage::default();
    let builder = DHallBuilder::new(&language);
    let source = SourceText::new(dhall.to_string());
    let mut cache = ParseSession::default();
    let result = builder.build(&source, &[], &mut cache);
    result.result.map_err(|e| format!("{:?}", e))
}

/// Highlighter implementation.
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::DHallHighlighter;

#[cfg(feature = "lsp")]
pub use crate::lsp::DHallLanguageService;
/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::formatter::DHallFormatter;

pub use lexer::token_type::DHallTokenType;
pub use parser::element_type::DHallElementType;
