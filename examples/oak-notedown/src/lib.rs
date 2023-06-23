#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]
//! Notedown support for the Oak language framework.

pub mod ast;
pub mod builder;

pub mod language;
pub mod lexer;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// MCP module.
#[cfg(feature = "mcp")]
pub mod mcp;

pub mod parser;

pub use crate::{
    ast::NoteDocument as NoteRoot,
    builder::NoteBuilder,
    language::NotedownLanguage as NoteLanguage,
    lexer::{NotedownLexer as NoteLexer, token_type::NoteTokenType},
    parser::NoteParser,
};

#[cfg(feature = "lsp")]
pub use crate::lsp::highlighter::{HighlightKind, Highlighter, NoteHighlighter};

#[cfg(feature = "lsp")]
pub use crate::lsp::NoteLanguageService;

#[cfg(feature = "mcp")]
pub use crate::mcp::serve_note_mcp;
pub use parser::element_type::NoteElementType;
