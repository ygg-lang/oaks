#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Windows Batch (BAT) support for the Oak language framework.

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;
/// Language configuration module.
pub mod language;
/// Lexer module.
pub mod lexer;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
/// LSP module.
pub mod lsp;
/// Parser module.
pub mod parser;

pub use builder::BatBuilder as Builder;
pub use language::BatLanguage as Language;
pub use lexer::{BatLexer as Lexer, token_type::BatTokenType as TokenType};
pub use parser::{BatParser as Parser, element_type::BatElementType as ElementType};
