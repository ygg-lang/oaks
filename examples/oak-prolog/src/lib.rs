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
pub mod lsp;
pub mod mcp;
pub mod parser;

pub use crate::{
    ast::PrologRoot,
    builder::PrologBuilder,
    highlighter::{HighlightKind, Highlighter, PrologHighlighter},
    kind::PrologSyntaxKind,
    language::PrologLanguage,
    lsp::PrologLanguageService,
    parser::PrologParser,
};

pub use crate::mcp::serve_prolog_mcp;
