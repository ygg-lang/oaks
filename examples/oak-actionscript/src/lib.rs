#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]

//! Actionscript support for the Oak language framework.

/// Abstract Syntax Tree (AST) definitions for ActionScript.
pub mod ast;
mod builder;

mod language;
/// Lexer for ActionScript.
pub mod lexer;
/// Language Server Protocol (LSP) and editor integration for ActionScript.
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;

/// Parser for ActionScript.
pub mod parser;

#[cfg(feature = "lsp")]
#[cfg(feature = "mcp")]
/// Model Context Protocol (MCP) support for ActionScript.
pub mod mcp;

// Re-export main types
pub use crate::{
    ast::ActionScriptRoot,
    builder::ActionScriptBuilder,
    language::ActionScriptLanguage,
    lexer::{ActionScriptLexer, ActionScriptTokenType},
    parser::{ActionScriptElementType, ActionScriptParser},
};

#[cfg(feature = "oak-pretty-print")]
pub use crate::lsp::formatter::ActionScriptFormatter;

#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::ActionScriptHighlighter;

#[cfg(feature = "lsp")]
pub use crate::lsp::ActionScriptLanguageService;

#[cfg(feature = "lsp")]
pub use crate::mcp::serve_actionscript_mcp;
