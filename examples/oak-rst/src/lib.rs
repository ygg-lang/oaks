#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![allow(missing_copy_implementations)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

/// The reStructuredText language implementation for Oaks.
pub mod language;

/// Abstract Syntax Tree for reStructuredText.
pub mod ast;

/// Lexer for tokenizing reStructuredText source.
pub mod lexer;

/// Parser for building reStructuredText syntax trees.
pub mod parser;

pub use crate::{ast::RstRoot, language::RstLanguage, lexer::RstLexer, parser::RstParser};
