#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]

/// AST module for Pascal.
pub mod ast;
/// Builder module for Pascal.
pub mod builder;

/// Language configuration module for Pascal.
pub mod language;
/// Lexer module for Pascal tokenization.
pub mod lexer;
/// LSP and IDE support module for Pascal.
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// MCP module.
#[cfg(feature = "mcp")]
pub mod mcp;

/// Parser module for Pascal syntax analysis.
pub mod parser;

pub use crate::{ast::PascalRoot, builder::PascalBuilder, language::PascalLanguage, lexer::PascalLexer, parser::PascalParser};

#[cfg(feature = "lsp")]
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::PascalHighlighter;

#[cfg(feature = "lsp")]
pub use crate::lsp::PascalLanguageService;

#[cfg(feature = "mcp")]
pub use crate::mcp::serve_pascal_mcp;
pub use lexer::token_type::PascalTokenType;
pub use parser::element_type::PascalElementType;
