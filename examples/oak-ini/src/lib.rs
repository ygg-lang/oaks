#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Ini support for the Oak language framework.

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;

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

pub use crate::{ast::IniRoot, builder::IniBuilder, language::IniLanguage, lexer::IniLexer, parser::IniParser};

/// Parses an INI string.
pub fn parse(ini: &str) -> Result<crate::ast::IniRoot, String> {
    use oak_core::{Builder, parser::session::ParseSession, source::SourceText};
    let language = IniLanguage::default();
    let builder = IniBuilder::new(&language);
    let source = SourceText::new(ini.to_string());
    let mut cache = ParseSession::default();
    let result = builder.build(&source, &[], &mut cache);
    result.result.map_err(|e| format!("{:?}", e))
}

/// Highlighter implementation.
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::IniHighlighter;

/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::IniLanguageService;

/// MCP service implementation.
#[cfg(feature = "mcp")]
pub use crate::mcp::serve_ini_mcp;
pub use lexer::token_type::IniTokenType;
pub use parser::element_type::IniElementType;
