#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub mod ast;
pub mod builder;
pub mod highlighter;
pub mod kind;
pub mod language;
pub mod lexer;
pub mod parser;

pub use crate::{
    ast::PurescriptRoot,
    builder::PurescriptBuilder,
    highlighter::{HighlightKind, Highlighter, PurescriptHighlighter},
    kind::PurescriptSyntaxKind,
    language::PurescriptLanguage,
    lexer::PurescriptLexer,
    parser::PurescriptParser,
};

// #[cfg(feature = "mcp")]
// pub mod mcp;
// #[cfg(feature = "mcp")]
// pub use crate::mcp::serve_purescript_mcp;
