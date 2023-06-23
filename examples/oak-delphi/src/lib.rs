#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]
//! Delphi programming language parser implementation
//!
//! This module provides a complete parser for the Delphi programming language,
//! including lexersyntax definitionsand language configuration.

pub mod ast;
mod builder;

mod language;
mod lexer;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
pub mod parser;

// Re-export main types
pub use crate::{
    ast::DelphiRoot,
    builder::DelphiBuilder,
    language::DelphiLanguage,
    lexer::{DelphiLexer, token_type::DelphiTokenType},
    parser::element_type::DelphiElementType,
};

#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub use crate::lsp::{formatter::DelphiFormatter, highlighter::DelphiHighlighter};
