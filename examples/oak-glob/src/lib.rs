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

pub use crate::{ast::GlobRoot, builder::GlobBuilder, language::GlobLanguage, lexer::GlobLexer, parser::GlobParser};

/// Parses a glob pattern string.
pub fn parse(glob: &str) -> Result<crate::ast::GlobRoot, String> {
    use oak_core::{Builder, parser::session::ParseSession, source::SourceText};
    let builder = GlobBuilder::default();
    let source = SourceText::new(glob.to_string());
    let mut cache = ParseSession::default();
    let result = builder.build(&source, &[], &mut cache);
    result.result.map_err(|e| format!("{:?}", e))
}

/// Highlighter implementation.
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::GlobHighlighter;

/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::GlobLanguageService;

/// MCP service implementation.
#[cfg(feature = "mcp")]
pub use crate::mcp::serve_glob_mcp;
pub use lexer::token_type::GlobTokenType;
pub use parser::element_type::GlobElementType;
