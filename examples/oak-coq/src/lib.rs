#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;
/// Language configuration module.
pub mod language;
/// Lexer module.
pub mod lexer;
/// LSP module.
#[cfg(feature = "lsp")]
pub mod lsp;
/// MCP module.
#[cfg(feature = "mcp")]
pub mod mcp;
/// Parser module.
pub mod parser;

pub use crate::{
    ast::CoqRoot,
    builder::CoqBuilder,
    language::CoqLanguage,
    lexer::CoqLexer,
    parser::{CoqParser, element_type::CoqElementType},
};
pub use lexer::token_type::CoqTokenType;

/// Highlighter implementation.
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::CoqHighlighter;

/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::CoqLanguageService;
#[cfg(feature = "oak-pretty-print")]
pub use crate::lsp::formatter::CoqFormatter;

/// MCP server implementation.
#[cfg(feature = "mcp")]
pub use crate::mcp::serve_coq_mcp;
