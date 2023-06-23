#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]
//! Vampire support for the Oak language framework.

pub mod ast;

mod builder;
pub mod language;
pub mod lexer;
pub mod parser;

// #[cfg(feature = "formatter")]
// mod formatter;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// MCP module.
#[cfg(feature = "mcp")]
pub mod mcp;

pub use crate::{ast::VampireRoot, builder::VampireBuilder, language::VampireLanguage, lexer::VampireLexer, parser::VampireParser};

#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::VampireHighlighter;

#[cfg(feature = "lsp")]
pub use crate::lsp::VampireLanguageService;

#[cfg(feature = "mcp")]
pub use crate::mcp::serve_vampire_mcp;
pub use lexer::token_type::VampireTokenType;
pub use parser::element_type::VampireElementType;
