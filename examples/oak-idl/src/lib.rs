#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

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

pub use crate::{ast::IdlRoot, builder::IdlBuilder, language::IdlLanguage, lexer::IdlLexer, parser::IdlParser};

/// Highlighter implementation.
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::IdlHighlighter;

/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::IdlLanguageService;
pub use lexer::token_type::IdlTokenType;
pub use parser::element_type::IdlElementType;

/// Parses an IDL string.
pub fn parse(idl: &str) -> Result<crate::ast::IdlRoot, String> {
    use oak_core::{Builder, parser::session::ParseSession, source::SourceText};
    let language = IdlLanguage::default();
    let builder = IdlBuilder::new(&language);
    let source = SourceText::new(idl.to_string());
    let mut cache = ParseSession::default();
    let result = builder.build(&source, &[], &mut cache);
    result.result.map_err(|e| format!("{:?}", e))
}
