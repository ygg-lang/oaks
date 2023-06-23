#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! C support for the Oak language framework.

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;
/// Type definition module.
/// Language configuration module.
pub mod language;
/// Lexer module.
pub mod lexer;
#[cfg(feature = "lsp")]
pub mod lsp;
/// Parser module.
pub mod parser;

pub use lexer::token_type::CTokenType;
pub use oak_core::{ElementType, Language, LanguageCategory, TokenType};
pub use parser::{CParser, element_type::CElementType};
