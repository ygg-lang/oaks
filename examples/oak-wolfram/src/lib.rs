#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]

pub mod ast;
mod builder;
pub mod language;
pub mod lexer;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// MCP module.
#[cfg(feature = "mcp")]
pub mod mcp;
pub mod parser;

pub use crate::{ast::WolframRoot, builder::WolframBuilder, language::WolframLanguage, lexer::WolframLexer, parser::WolframParser};

/// Highlighter implementation.
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::WolframHighlighter;

/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::WolframLanguageService;

/// MCP service implementation.
#[cfg(feature = "mcp")]
pub use crate::mcp::serve_wolfram_mcp;
pub use lexer::token_type::WolframTokenType;
pub use oak_core::{ElementType, TokenType};
pub use parser::element_type::WolframElementType;
