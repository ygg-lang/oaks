#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc = include_str!("readme.md")]
#![allow(missing_docs)]

pub mod ast;
pub mod builder;
pub mod highlighter;
pub mod kind;
mod language;
pub mod lexer;
pub mod lsp;
pub mod mcp;
pub mod parser;

pub use crate::{ast::CoqRoot, builder::CoqBuilder, highlighter::CoqHighlighter, language::CoqLanguage, lexer::CoqLexer, lsp::CoqLanguageService, parser::CoqParser};
