#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Dart support for the Oak language framework.

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
/// 语法分析器模块。
pub mod parser;

pub use crate::{language::DartLanguage, lexer::DartLexer, parser::DartParser};

//
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::DartHighlighter;

// /// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::DartLanguageService;
#[cfg(feature = "lsp")]
pub use crate::lsp::formatter::DartFormatter;
pub use lexer::token_type::DartTokenType;
pub use parser::element_type::DartElementType;
