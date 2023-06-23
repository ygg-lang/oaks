#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
use oak_core::Builder;

// Csharp support for the Oak language framework.

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;
/// Syntax kind definitions.
/// Language definition.
pub mod language;
/// Lexer.
pub mod lexer;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// Parser.
pub mod parser;

pub use ast::CSharpRoot;
pub use builder::CSharpBuilder;
pub use language::CSharpLanguage;
pub use lexer::{CSharpLexer, token_type::CSharpTokenType};
pub use parser::CSharpParser;

/// Parse C# source code into CSharpRoot AST
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
