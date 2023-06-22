#![feature(new_range_api)]
#![feature(portable_simd)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub mod ast;
pub mod builder;
#[cfg(feature = "oak-highlight")]
pub mod highlighter;
pub mod kind;
pub mod language;
pub mod lexer;
pub mod lsp;
#[cfg(feature = "mcp")]
pub mod mcp;
pub mod parser;

// 重新导出主要类型
pub use crate::{ast::JsonValue, builder::JsonBuilder, kind::JsonSyntaxKind, language::JsonLanguage, lexer::JsonLexer, lsp::JsonLanguageService, parser::JsonParser};
