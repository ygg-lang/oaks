#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

use oak_core::Builder;

/// AST module
/// AST module.
pub mod ast;
/// Builder module
pub mod builder;
/// Language definition and configuration for F#.
pub mod language;
/// Lexer implementation for F#.
pub mod lexer;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
/// LSP module
pub mod lsp;
/// Parser module
pub mod parser;

pub use ast::FSharpRoot;
pub use builder::FSharpBuilder;
pub use language::FSharpLanguage;
pub use lexer::{FSharpLexer, token_type::FSharpTokenType};
pub use parser::{FSharpParser, element_type::FSharpElementType};

/// Parses F# source code into a [FSharpRoot] AST.
///
/// This is a convenience function that initializes the language, builder,
/// and parser to process the source text.
///
/// # Errors
///
/// Returns an [oak_core::OakError] if parsing fails.
pub fn parse(source: &str) -> Result<FSharpRoot, oak_core::OakError> {
    let language = FSharpLanguage::new();
    let builder = FSharpBuilder::new(&language);
    let source_text = oak_core::source::SourceText::new(source);
    let mut session = oak_core::parser::ParseSession::<FSharpLanguage>::default();
    let output = builder.build(&source_text, &[], &mut session);
    output.result
}

/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::FsharpLanguageService;
