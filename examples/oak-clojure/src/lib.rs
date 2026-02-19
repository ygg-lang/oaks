#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;
mod language;
mod lexer;
/// Parser module.
pub mod parser;
// pub mod lsp;

pub use builder::ClojureBuilder;
pub use language::ClojureLanguage;
pub use lexer::{ClojureLexer, token_type::ClojureTokenType};
pub use parser::{ClojureParser, element_type::ClojureElementType};
