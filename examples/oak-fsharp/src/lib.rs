#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Fsharp support for the Oak language framework.

pub mod ast;
pub mod builder;
mod language;
mod lexer;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
pub mod parser;

pub use ast::FSharpRoot;
pub use builder::FSharpBuilder;
pub use language::FSharpLanguage;
pub use lexer::{FSharpLexer, token_type::FSharpTokenType};
pub use parser::element_type::FSharpElementType;
