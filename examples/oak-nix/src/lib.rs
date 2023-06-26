#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;

// pub mod formatter;

/// Syntax kind module.
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

pub use crate::{ast::NixRoot, builder::NixBuilder, language::NixLanguage, lexer::NixLexer, parser::NixParser};

/// Parses a Nix string.
pub fn parse(nix: &str) -> Result<NixRoot, String> {
    use oak_core::{Builder, parser::session::ParseSession, source::SourceText};
    let language = NixLanguage::default();
    let builder = NixBuilder::new(&language);
    let source = SourceText::new(nix.to_string());
    let mut cache = ParseSession::default();
    let result = builder.build(&source, &[], &mut cache);
    result.result.map_err(|e| format!("{:?}", e))
}

/// Highlighter implementation.
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::NixHighlighter;

/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::NixLanguageService;

/// MCP service implementation.
#[cfg(feature = "mcp")]
pub use crate::mcp::serve_nix_mcp;
pub use lexer::token_type::NixTokenType;
pub use parser::element_type::NixElementType;
