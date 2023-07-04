#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

/// AST module
pub mod ast;
/// Builder module
pub mod builder;
mod language;
mod lexer;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
/// LSP module
pub mod lsp;
/// Parser module
pub mod parser;

pub use ast::{JinjaElement, JinjaRoot};
pub use builder::JinjaBuilder;
pub use language::JinjaLanguage;
pub use lexer::{JinjaLexer, token_type::JinjaTokenType};
pub use parser::{JinjaParser, element_type::JinjaElementType};

/// Highlighter implementation
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::JinjaHighlighter;

#[cfg(feature = "lsp")]
pub use crate::lsp::JinjaLanguageService;
/// LSP implementation
#[cfg(feature = "lsp")]
pub use crate::lsp::formatter::JinjaFormatter;
