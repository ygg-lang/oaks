#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![allow(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Yaml support for the Oak language framework.

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;
/// Kind definition module.
/// Language configuration module.
pub mod language;
/// Lexer module.
pub mod lexer;
/// LSP module.
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "mcp"))]
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// MCP module.
#[cfg(feature = "mcp")]
pub mod mcp;

/// Parser module.
pub mod parser;

pub use crate::{builder::YamlBuilder, language::YamlLanguage, lexer::YamlLexer, parser::YamlParser};

/// Highlighter implementation.
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::YamlHighlighter;

/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::YamlLanguageService;
#[cfg(feature = "oak-pretty-print")]
pub use crate::lsp::formatter::YamlFormatter;

#[cfg(feature = "lsp")]
pub use crate::mcp::serve_yaml_mcp;
pub use lexer::token_type::YamlTokenType;
pub use parser::element_type::YamlElementType;
