#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![feature(adt_const_params)]
#![allow(incomplete_features)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;

/// Language configuration module.
pub mod language;
/// Lexer module.
pub mod lexer;
/// LSP module.
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
// /// MCP module.
// #[cfg(feature = "mcp")]
// pub mod mcp;
/// Parser module.
pub mod parser;

pub use crate::{
    ast::{DsvField, DsvRecord, DsvRoot},
    builder::DsvBuilder,
    language::{Dsv, DsvLanguage},
    lexer::DsvLexer,
    parser::DsvParser,
};

#[cfg(feature = "lsp")]
pub use crate::lsp::DsvLanguageService;

#[cfg(feature = "serde")]
pub use crate::language::from_value;
#[cfg(feature = "serde")]
pub use crate::language::to_value;

/// Serializes the given value to a DSV string with the given configuration.
#[cfg(feature = "serde")]
pub fn to_string_with_config<const LANG: DsvLanguage, T: ::serde::Serialize>(value: &T) -> Result<String, oak_core::OakError> {
    use oak_core::source::ToSource;
    let dsv_root = to_value::<LANG, T>(value)?;
    let mut buffer = oak_core::source::SourceBuffer::default();
    dsv_root.to_source(&mut buffer);
    Ok(buffer.finish())
}

/// Deserializes a DSV string into a value of type `T` with the given configuration.
#[cfg(feature = "serde")]
pub fn from_str_with_config<const LANG: DsvLanguage, T: ::serde::de::DeserializeOwned>(s: &str) -> Result<T, oak_core::OakError> {
    use oak_core::{Builder, Lexer, Parser};
    let mut session = oak_core::parser::session::ParseSession::<Dsv<LANG>>::default();
    let lexer = crate::lexer::DsvLexer::<LANG>::new();
    lexer.lex(s, &[], &mut session);
    let parser = crate::parser::DsvParser::<LANG>::new();
    let parse_result = parser.parse(s, &[], &mut session);
    let _green_tree = parse_result.result?;

    let builder = crate::builder::DsvBuilder::<LANG>::new();
    let mut builder_cache = oak_core::parser::session::ParseSession::<Dsv<LANG>>::default();
    let diagnostics = builder.build(s, &[], &mut builder_cache);
    let ast_root = diagnostics.result?;

    from_value(ast_root)
}

/// Parses a DSV string into a `DsvRoot` AST.
pub fn parse<const LANG: DsvLanguage>(dsv: &str) -> Result<crate::ast::DsvRoot<LANG>, oak_core::OakError> {
    use oak_core::{Builder, parser::session::ParseSession, source::SourceText};
    let builder = DsvBuilder::<LANG>::new();
    let source = SourceText::new(dsv.to_string());
    let mut cache = ParseSession::default();
    let result = builder.build(&source, &[], &mut cache);
    result.result
}
