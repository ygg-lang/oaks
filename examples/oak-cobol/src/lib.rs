#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Cobol support for the Oak language framework.

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;
pub use builder::CobolBuilder;
/// Kind definition module.
/// Language configuration module.
pub mod language;
pub use language::CobolLanguage;
/// Token type module.
pub mod lexer;
pub use lexer::token_type::CobolTokenType;
/// LSP module.
#[cfg(feature = "lsp")]
pub mod lsp;
/// Parser module.
pub mod parser;
pub use parser::{CobolParser, element_type::CobolElementType};

pub use oak_core::{ElementType, TokenType};
