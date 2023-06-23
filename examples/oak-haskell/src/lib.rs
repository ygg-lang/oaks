#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]
//! Haskell support for the Oak language framework.

pub mod ast;
pub mod builder;

mod language;
mod lexer;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// MCP module.
#[cfg(feature = "mcp")]
pub mod mcp;

mod parser;

pub use crate::{ast::HaskellRoot, builder::HaskellBuilder, language::HaskellLanguage, lexer::HaskellLexer, parser::HaskellParser};

#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::HaskellHighlighter;

#[cfg(feature = "lsp")]
pub use crate::lsp::HaskellLanguageService;

#[cfg(feature = "mcp")]
pub use crate::mcp::serve_haskell_mcp;
pub use lexer::token_type::HaskellTokenType;
pub use parser::element_type::HaskellElementType;
