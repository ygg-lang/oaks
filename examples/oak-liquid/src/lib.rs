#![doc = include_str!("readme.md")]
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

pub use ast::LiquidRoot;
pub use builder::LiquidBuilder;
pub use language::LiquidLanguage;
pub use lexer::{LiquidLexer, token_type::LiquidTokenType};
pub use parser::{LiquidParser, element_type::LiquidElementType};

/// Highlighter implementation
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::LiquidHighlighter;

#[cfg(feature = "lsp")]
pub use crate::lsp::LiquidLanguageService;
/// LSP implementation
#[cfg(feature = "lsp")]
pub use crate::lsp::formatter::LiquidFormatter;
