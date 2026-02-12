#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![allow(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Sql support for the Oak language framework.

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;

/// Type definitions module.
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
    ast::SqlRoot,
    builder::SqlBuilder,
    language::SqlLanguage,
    lexer::{
        SqlLexer,
        token_type::{SqlTokenType, SqlTokenType as SqlSyntaxKind},
    },
    parser::{SqlParser, element_type::SqlElementType},
};

/// Re-export SqlSyntaxKind in a kind module for backward compatibility
pub mod kind {
    pub use crate::lexer::token_type::SqlTokenType as SqlSyntaxKind;
}

/// Highlighter implementation.
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::SqlHighlighter;

#[cfg(feature = "lsp")]
pub use crate::lsp::{SqlLanguageService, formatter::SqlFormatter};
