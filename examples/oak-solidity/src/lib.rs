#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]
//! Solidity support for the Oak language framework.

// pub mod ast;
mod builder;

pub mod language;
pub mod lexer;
pub mod parser;
//

#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// MCP module.
#[cfg(feature = "mcp")]
pub mod mcp;

#[cfg(feature = "lsp")]
pub use crate::lsp::SolidityLanguageService;
pub use crate::{builder::SolidityBuilder, language::SolidityLanguage, lexer::SolidityLexer, parser::SolidityParser};

#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::SolidityHighlighter;
#[cfg(feature = "mcp")]
pub use crate::mcp::serve_solidity_mcp;
pub use lexer::token_type::SolidityTokenType;
pub use parser::element_type::SolidityElementType;
