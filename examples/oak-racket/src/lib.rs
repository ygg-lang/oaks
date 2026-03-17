#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]

/// Abstract Syntax Tree types.
pub mod ast;
/// AST builder.
pub mod builder;
/// Code formatter.
pub mod formatter;
/// Syntax highlighter.
pub mod highlighter;
/// Language definition.
pub mod language;
/// Lexer for tokenization.
pub mod lexer;
/// Language Server Protocol support.
pub mod lsp;
/// Model Context Protocol support.
pub mod mcp;
/// Parser for syntax analysis.
pub mod parser;

pub use language::RacketLanguage;
