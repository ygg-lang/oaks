#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! C# support for the Oak language framework.
//!
//! This crate provides lexing, parsing, and AST building for the C# language,
//! integrated into the Oak ecosystem. It supports modern C# features including
//! namespaces, classes, structs, records, and more.

use oak_core::Builder;

/// AST module containing high-level C# syntax tree definitions.
pub mod ast;
/// Builder module for converting green trees into high-level AST nodes.
pub mod builder;
/// Language definition and configuration for C#.
pub mod language;
/// Lexer implementation for C#.
pub mod lexer;
/// LSP-related functionality (hover, completion, highlighting) for C#.
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// Parser implementation for C#.
pub mod parser;

pub use ast::CSharpRoot;
pub use builder::CSharpBuilder;
pub use language::CSharpLanguage;
pub use lexer::{CSharpLexer, token_type::CSharpTokenType};
pub use parser::CSharpParser;

/// Parses C# source code into a [CSharpRoot] AST.
///
/// This is a convenience function that initializes the language, builder,
/// and parser to process the source text.
///
/// # Errors
///
/// Returns an [oak_core::OakError] if parsing fails.
pub fn parse(source: &str) -> Result<CSharpRoot, oak_core::OakError> {
    let language = CSharpLanguage::new();
    let builder = CSharpBuilder::new(&language);
    let source_text = oak_core::source::SourceText::new(source);
    let mut session = oak_core::parser::ParseSession::<CSharpLanguage>::default();
    let output = builder.build(&source_text, &[], &mut session);
    output.result
}

pub use parser::element_type::CSharpElementType;

/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::CSharpLanguageService;
