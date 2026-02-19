#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]

/// The AST nodes for TSV.
pub mod ast;
/// The builder for TSV.
pub mod builder;
/// The language configuration and marker.
pub mod language;
/// The lexer for TSV.
pub mod lexer;
/// The parser for TSV.
pub mod parser;

/// Language service implementation for TSV.
#[cfg(feature = "lsp")]
pub mod lsp;

pub use crate::{
    ast::{TsvField, TsvRecord, TsvRoot},
    builder::TsvBuilder,
    language::{TSV_LANG, TsvLanguage},
    lexer::TsvLexer,
    parser::TsvParser,
};

/// A TSV root node.
pub type TsvRootNode = crate::ast::TsvRoot;

/// Serializes the given value to a TSV string.
#[cfg(feature = "serde")]
/// Serializes the given value to a TSV string.
pub fn to_string<T: ::serde::Serialize>(value: &T) -> Result<String, oak_core::OakError> {
    oak_dsv::to_string_with_config::<TSV_LANG, T>(value)
}

/// Deserializes a TSV string into a value of type `T`.
#[cfg(feature = "serde")]
/// Deserializes a TSV string into a value of type `T`.
pub fn from_str<T: ::serde::de::DeserializeOwned>(s: &str) -> Result<T, oak_core::OakError> {
    oak_dsv::from_str_with_config::<TSV_LANG, T>(s)
}

/// Returns the default TSV configuration.
pub fn language() -> oak_dsv::DsvLanguage {
    TSV_LANG
}

/// Parses a TSV string into a `TsvRoot` AST.
pub fn parse(tsv: &str) -> Result<crate::ast::TsvRoot, oak_core::OakError> {
    use oak_core::{Builder, parser::session::ParseSession, source::SourceText};
    let builder = TsvBuilder::new();
    let source = SourceText::new(tsv.to_string());
    let mut cache = ParseSession::default();
    let result = builder.build(&source, &[], &mut cache);
    result.result
}

/// Language service implementation for TSV.
#[cfg(feature = "lsp")]
pub use crate::lsp::TsvLanguageService;
