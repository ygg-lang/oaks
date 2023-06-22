#![feature(new_range_api)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]

/// AST definitions for Valkyrie.
pub mod ast;
/// Builder implementation for Valkyrie.
pub mod builder;
/// Formatter implementation for Valkyrie.
pub mod formatter;
/// Highlighter implementation for Valkyrie.
pub mod highlighter;
/// Syntax kinds for Valkyrie.
pub mod kind;
/// Language definition for Valkyrie.
pub mod language;
/// Lexer implementation for Valkyrie.
pub mod lexer;
/// Language Server Protocol support for Valkyrie.
pub mod lsp;
/// MCP support for Valkyrie.
pub mod mcp;
/// Parser implementation for Valkyrie.
pub mod parser;

// Re-export main types for convenience
pub use crate::{
    ast::ValkyrieRoot, builder::ValkyrieBuilder, formatter::ValkyrieFormatter, highlighter::ValkyrieHighlighter, kind::ValkyrieSyntaxKind, language::ValkyrieLanguage, lexer::ValkyrieLexer, lsp::ValkyrieLanguageService, parser::ValkyrieParser,
};

pub use crate::mcp::serve_valkyrie_mcp;
