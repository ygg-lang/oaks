#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![feature(portable_simd)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub mod ast;
pub mod builder;
pub mod highlighter;
mod kind;
mod language;
mod lexer;
#[cfg(feature = "lsp")]
pub mod lsp;
mod parser;

#[cfg(feature = "lsp")]
pub mod mcp;

pub use crate::{ast::HtmlDocument, builder::HtmlBuilder, highlighter::HtmlHighlighter, language::HtmlLanguage, lexer::HtmlLexer, parser::HtmlParser};

#[cfg(feature = "lsp")]
pub use crate::lsp::HtmlLanguageService;
