//! Delphi programming language parser implementation
//!
//! This module provides a complete parser for the Delphi programming language,
//! including lexer, syntax definitions, and language configuration.

#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub mod ast;
mod builder;
mod formatter;
pub mod highlighter;
mod kind;
mod language;
mod lexer;
pub mod parser;

// Re-export main types
pub use crate::{
    ast::DelphiRoot,
    builder::DelphiBuilder,
    formatter::DelphiFormatter,
    highlighter::DelphiHighlighter,
    kind::{DelphiSyntaxKind, DelphiToken},
    language::DelphiLanguage,
    lexer::DelphiLexer,
};
