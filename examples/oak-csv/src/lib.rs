#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]

/// The AST nodes for CSV.
pub mod ast;
/// The builder for CSV.
pub mod builder;
/// The language configuration and marker.
pub mod language;
/// The lexer for CSV.
pub mod lexer;
/// Language service implementation for CSV.
#[cfg(feature = "lsp")]
pub mod lsp;
/// The parser for CSV.
pub mod parser;

pub use crate::{
    ast::{CsvField, CsvRecord, CsvRoot},
    builder::CsvBuilder,
    language::{CSV_LANG, CsvLanguage},
    lexer::CsvLexer,
    parser::CsvParser,
};

/// A CSV root node.
pub type CsvRootNode = crate::ast::CsvRoot;

/// Serializes the given value to a CSV string.
#[cfg(feature = "serde")]
/// Serializes the given value to a CSV string.
pub fn to_string<T: ::serde::Serialize>(value: &T) -> Result<String, oak_core::OakError> {
    oak_dsv::to_string_with_config::<CSV_LANG, T>(value)
}

/// Deserializes a CSV string into a value of type `T`.
#[cfg(feature = "serde")]
/// Deserializes a CSV string into a value of type `T`.
pub fn from_str<T: ::serde::de::DeserializeOwned>(s: &str) -> Result<T, oak_core::OakError> {
    oak_dsv::from_str_with_config::<CSV_LANG, T>(s)
}

/// Returns the default CSV configuration.
pub fn language() -> oak_dsv::DsvLanguage {
    CSV_LANG
}

/// Parses a CSV string into a `CsvRoot` AST.
pub fn parse(csv: &str) -> Result<crate::ast::CsvRoot, oak_core::OakError> {
    use oak_core::{Builder, parser::session::ParseSession, source::SourceText};
    let builder = CsvBuilder::new();
    let source = SourceText::new(csv.to_string());
    let mut cache = ParseSession::default();
    let result = builder.build(&source, &[], &mut cache);
    result.result
}

/// Language service implementation for CSV.
#[cfg(feature = "lsp")]
pub use crate::lsp::CsvLanguageService;
