#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
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
    ast::ObjectiveCRoot,
    builder::ObjectiveCBuilder,
    highlighter::{HighlightKind, Highlighter, ObjectiveCHighlighter},
    kind::ObjectiveCSyntaxKind,
    language::ObjectiveCLanguage,
    lexer::ObjectiveCLexer,
    lsp::ObjectiveCLanguageService,
    parser::ObjectiveCParser,
};

pub use crate::mcp::serve_objective_c_mcp;
