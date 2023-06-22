#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![allow(missing_docs)]

pub mod ast;
mod builder;
mod kind;
mod language;
mod lexer;
mod parser;

#[cfg(feature = "oak-pretty-print")]
mod formatter;
#[cfg(feature = "oak-highlight")]
mod highlighter;

pub mod lsp;
pub mod mcp;

#[cfg(feature = "oak-pretty-print")]
pub use crate::formatter::RegexFormatter;
#[cfg(feature = "oak-highlight")]
pub use crate::highlighter::RegexHighlighter;
pub use crate::{
    ast::RegexRoot,
    kind::{RegexSyntaxKind, RegexToken},
    language::RegexLanguage,
    lexer::RegexLexer,
    lsp::RegexLanguageService,
    parser::RegexParser,
};

pub use crate::mcp::serve_regex_mcp;
