#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
//! Wit-component support for the Oak language framework.

pub mod ast;
pub mod builder;
pub mod language;
pub mod lexer;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
pub mod parser;

pub use crate::{ast::WitRoot, builder::WitBuilder, language::WitLanguage, lexer::WitLexer, parser::WitParser};
pub use lexer::token_type::WitTokenType;
pub use parser::element_type::WitElementType;

/// Parses a WIT string.
pub fn parse(wit: &str) -> Result<crate::ast::WitRoot, String> {
    use oak_core::{Builder, parser::session::ParseSession, source::SourceText};
    let language = WitLanguage::default();
    let builder = WitBuilder::new(&language);
    let source = SourceText::new(wit.to_string());
    let mut cache = ParseSession::default();
    let result = builder.build(&source, &[], &mut cache);
    result.result.map_err(|e| format!("{:?}", e))
}
