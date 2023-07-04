#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
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
/// MCP module.
#[cfg(feature = "mcp")]
pub mod mcp;

/// Parser module.
pub mod parser;

pub use crate::{
    ast::{Attribute, ScriptAst, StyleAst, StyleRule, TemplateNode, VxDocument, VxParseError},
    builder::VocBuilder,
    language::VocLanguage,
    lexer::VocLexer,
    parser::VocParser,
};

pub use oak_core::{Builder, ElementType, TokenType};

/// Parses a VX document from source text.
pub fn parse_vx(source: &str) -> Result<VxDocument, VxParseError> {
    use oak_core::Builder;
    let language = VocLanguage::new();
    let builder = VocBuilder::new(&language);
    let source_text = oak_core::SourceText::new(source.to_string());
    let mut cache = oak_core::parser::session::ParseSession::<VocLanguage>::default();
    let result = builder.build(&source_text, &[], &mut cache);
    result.result.map_err(|e| VxParseError { message: format!("{:?}", e), line: 1, column: 1 })
}

/// Highlighter implementation.
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::VocLanguageService;

/// MCP service implementation.
#[cfg(feature = "mcp")]
pub use crate::mcp::serve_voc_mcp;
pub use lexer::token_type::VocTokenType;
pub use parser::element_type::VocElementType;
