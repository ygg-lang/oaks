#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![allow(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Perl support for the Oak language framework.

/// AST module for Perl.
pub mod ast;
/// Builder module for Perl.
pub mod builder;
/// Kind module for Perl syntax types.
/// Language configuration module for Perl.
pub mod language;
/// Lexer module for Perl tokenization.
pub mod lexer;
/// LSP module for Perl language service features.
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// MCP module.
#[cfg(feature = "mcp")]
pub mod mcp;

/// Parser module for Perl syntax analysis.
pub mod parser;

pub use crate::{ast::PerlRoot, builder::PerlBuilder, language::PerlLanguage, lexer::PerlLexer, parser::PerlParser};

/// Highlighter implementation for Perl.
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::PerlHighlighter;

/// LSP implementation for Perl.
#[cfg(feature = "lsp")]
pub use crate::lsp::PerlLanguageService;

/// MCP service implementation for Perl.
#[cfg(feature = "mcp")]
pub use crate::mcp::serve_perl_mcp;
pub use lexer::token_type::PerlTokenType;
pub use parser::element_type::PerlElementType;
