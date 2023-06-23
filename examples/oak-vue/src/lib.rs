#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Vue support for the Oak language framework.

// pub mod ast;
// // pub mod builder;
// pub mod language;

/// Kind definition module.
/// Lexer module.
pub mod lexer;

// #[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
// pub mod lsp;

/// Parser module.
pub mod parser;

pub use lexer::{VueLexer, token_type::VueTokenType};
pub use parser::{VueParser, element_type::VueElementType};

use oak_core::{parser::ParseOutput, source::SourceText};

// pub fn parse<'a>(source: &'a SourceText, cache: &'a mut oak_core::parser::ParseSession<crate::lexer::token_type::VueLanguage>) -> ParseOutput<'a, crate::lexer::token_type::VueLanguage> {
// let language = crate::lexer::token_type::VueLanguage::default();
// let lexer = VueLexer::new(&language);
// let parser = VueParser::new(&language);
// oak_core::parser::parse(&parser, &lexer, source, &[], cache)
// }
