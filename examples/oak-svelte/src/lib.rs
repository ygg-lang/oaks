#![doc = include_str!("../readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;
/// Language module.
pub mod language;
/// Lexer module.
pub mod lexer;
/// Parser module.
pub mod parser;

pub use ast::*;
pub use builder::SvelteBuilder;
pub use language::SvelteLanguage;
pub use lexer::{SvelteLexer, token_type::SvelteTokenType};
pub use parser::{SvelteParser, element_type::SvelteElementType};
