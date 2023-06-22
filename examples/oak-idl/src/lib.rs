#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
pub mod ast;
pub mod builder;
pub mod highlighter;
mod kind;
mod language;
mod lexer;
pub mod lsp;
mod parser;

#[cfg(feature = "mcp")]
pub mod mcp;

pub use crate::{
    ast::IdlRoot,
    builder::IdlBuilder,
    highlighter::IdlHighlighter,
    kind::{IdlSyntaxKind, IdlToken},
    language::IdlLanguage,
    lexer::IdlLexer,
    lsp::IdlLanguageService,
    parser::IdlParser,
};
