#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Coq support for the Oak language framework.

/// AST 模块。
pub mod ast;
/// 构建器模块。
pub mod builder;
/// 类型定义模块。
/// 语言配置模块。
pub mod language;
/// 词法分析器模块。
pub mod lexer;
/// LSP 模块。
#[cfg(feature = "lsp")]
pub mod lsp;
/// MCP 模块。
#[cfg(feature = "mcp")]
pub mod mcp;
/// 语法分析器模块。
pub mod parser;

pub use crate::{
    ast::CoqRoot,
    builder::CoqBuilder,
    language::CoqLanguage,
    lexer::CoqLexer,
    parser::{CoqParser, element_type::CoqElementType},
};
pub use lexer::token_type::CoqTokenType;

/// Highlighter implementation.
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::CoqHighlighter;

/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::CoqLanguageService;
#[cfg(feature = "oak-pretty-print")]
pub use crate::lsp::formatter::CoqFormatter;

/// MCP server implementation.
#[cfg(feature = "mcp")]
pub use crate::mcp::serve_coq_mcp;
