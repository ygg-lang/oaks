#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub mod ast;
mod builder;
pub mod highlighter;
pub mod kind;
mod language;
pub mod lexer;
pub mod lsp;
pub mod parser;
pub mod syntax;

#[cfg(feature = "mcp")]
pub mod mcp;

pub use crate::{ast::JasmRoot, builder::JasmBuilder, highlighter::JasmHighlighter, language::JasmLanguage, lexer::JasmLexer, lsp::JasmLanguageService, parser::JasmParser};
