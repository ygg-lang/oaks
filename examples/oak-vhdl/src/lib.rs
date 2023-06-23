#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Vhdl support for the Oak language framework.

/// AST module.
pub mod ast;

// pub mod builder;

/// Type definitions module.
/// Language configuration module.
pub mod language;
/// Lexer module.
pub mod lexer;
// pub mod lsp;

/// Parser module.
pub mod parser;

pub use crate::{language::VhdlLanguage, lexer::VhdlLexer, parser::VhdlParser};
pub use oak_core::{ElementType, TokenType};

// #[cfg(feature = "oak-highlight")]
// // pub use crate::lsp::highlighter::VhdlHighlighter;
// #[cfg(feature = "lsp")]
// pub use crate::lsp::formatter::VhdlFormatter;
// #[cfg(feature = "lsp")]
// pub use crate::{ lsp::VhdlLanguageService};
// #[cfg(feature = "mcp")]
// pub use crate::mcp::serve_vhdl_mcp;
pub use lexer::token_type::VhdlTokenType;
pub use parser::element_type::VhdlElementType;
