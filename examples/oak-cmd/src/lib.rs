#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Windows Command (CMD) support for the Oak language framework.

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

pub use builder::CmdBuilder as Builder;
pub use language::CmdLanguage as Language;
pub use lexer::{CmdLexer as Lexer, token_type::CmdTokenType as TokenType};
pub use parser::{CmdParser as Parser, element_type::CmdElementType as ElementType};
