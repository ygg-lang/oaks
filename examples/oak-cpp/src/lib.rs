#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Cpp support for the Oak language framework.

/// AST 模块。
pub mod ast;
/// 类型定义模块。
/// 语言配置模块。
pub mod language;
/// 词法分析器模块。
pub mod lexer;
/// 语法分析器模块。
pub mod parser;

pub use crate::{ast::CppRoot, language::CppLanguage, lexer::CppLexer, parser::CppParser};
pub use lexer::token_type::CppTokenType;
pub use parser::element_type::CppElementType;
