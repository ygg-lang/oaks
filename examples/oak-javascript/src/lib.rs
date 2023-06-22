#![feature(new_range_api)]
#![feature(portable_simd)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

extern crate oak_core;
extern crate serde;

pub mod ast;
mod builder;
mod formatter;
pub mod highlighter;
pub mod kind;
pub mod language;
pub mod lexer;
pub mod lsp;
#[cfg(feature = "mcp")]
pub mod mcp;
pub mod parser;

// 重新导出主要类型
pub use crate::{
    ast::JavaScriptRoot, builder::JavaScriptBuilder, formatter::JavaScriptFormatter, highlighter::JavaScriptHighlighter, kind::JavaScriptSyntaxKind, language::JavaScriptLanguage, lexer::JavaScriptLexer, lsp::JavaScriptLanguageService,
    parser::JavaScriptParser,
};

#[cfg(feature = "mcp")]
pub use crate::mcp::serve_javascript_mcp;

#[cfg(all(feature = "mcp", feature = "axum"))]
pub use crate::mcp::serve_javascript_mcp_axum;
