#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]
//! Wolfram support for the Oak language framework.

pub mod ast;
mod builder;
pub mod language;
pub mod lexer;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
pub mod parser;

pub use crate::{builder::WolframBuilder, language::WolframLanguage, lexer::WolframLexer, parser::WolframParser};
pub use lexer::token_type::WolframTokenType;
pub use oak_core::{ElementType, TokenType};
pub use parser::element_type::WolframElementType;
