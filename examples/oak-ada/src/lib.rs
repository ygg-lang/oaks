#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![feature(new_range_api)]
#![allow(missing_docs)]

extern crate oak_core;
extern crate serde;

/// AST 模块
pub mod ast;
mod builder;
mod formatter;
/// 高亮模块
pub mod highlighter;
mod language;
/// 词法分析器模块
pub mod lexer;
/// LSP 模块
pub mod lsp;
#[cfg(feature = "mcp")]
pub mod mcp;
/// 语法分析器模块
pub mod parser;

// 重新导出主要类型
pub use crate::{ast::AdaRoot, builder::AdaBuilder, formatter::AdaFormatter, highlighter::AdaHighlighter, language::AdaLanguage, lexer::AdaLexer, lsp::AdaLanguageService, parser::AdaParser};

#[cfg(feature = "mcp")]
pub use crate::mcp::serve_ada_mcp;
