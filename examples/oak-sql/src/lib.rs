#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! SQL support for the Oak language framework.

/// AST module for SQL.
pub mod ast;
/// Builder module for SQL.
pub mod builder;

/// Language configuration module for SQL.
pub mod language;
/// Lexer module for SQL.
pub mod lexer;
/// LSP module for SQL.
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// MCP module for SQL.
#[cfg(feature = "mcp")]
pub mod mcp;

/// Parser module for SQL.
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

/// Parses a SQL string.
pub fn parse(sql: &str) -> Result<crate::ast::SqlRoot, String> {
    use oak_core::{Builder, parser::session::ParseSession, source::SourceText};
    let language = SqlLanguage::default();
    let builder = SqlBuilder::new(&language);
    let source = SourceText::new(sql.to_string());
    let mut cache = ParseSession::default();
    let result = builder.build(&source, &[], &mut cache);
    result.result.map_err(|e| format!("{:?}", e))
}

/// Re-export SqlSyntaxKind in a kind module for backward compatibility
pub mod kind {
    /// Re-export SqlTokenType as SqlSyntaxKind.
    pub use crate::lexer::token_type::SqlTokenType as SqlSyntaxKind;
}

/// Highlighter implementation.
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::SqlHighlighter;

#[cfg(feature = "lsp")]
pub use crate::lsp::{SqlLanguageService, formatter::SqlFormatter};
