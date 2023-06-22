#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![feature(new_range_api)]
#![feature(portable_simd)]
#![warn(missing_docs)]

extern crate oak_core;
extern crate serde;

pub mod ast;

mod builder;
mod language;
/// 词法分析器模块
pub mod lexer;
/// 语法分析器模块
pub mod parser;

mod formatter;
/// 高亮模块
pub mod highlighter;
/// LSP 模块
pub mod lsp;
#[cfg(feature = "mcp")]
/// MCP integration for Rust.
pub mod mcp;

// 重新导出主要类型
pub use crate::{ast::RustRoot, builder::RustBuilder, formatter::RustFormatter, highlighter::RustHighlighter, language::RustLanguage, lexer::RustLexer, lsp::RustLanguageService, parser::RustParser};

#[cfg(feature = "mcp")]
pub use crate::mcp::serve_rust_mcp;

#[cfg(all(feature = "mcp", feature = "axum"))]
pub use crate::mcp::serve_rust_mcp_axum;
