#![feature(new_range_api)]
#![doc = include_str!("../readme.md")]

pub mod ast;
pub mod builder;
pub mod formatter;
pub mod highlighter;
pub mod kind;
pub mod language;
pub mod lexer;
pub mod lsp;
#[cfg(feature = "mcp")]
pub mod mcp;

pub mod parser;

// Re-exports
pub use crate::{
    ast::TypeScriptRoot,
    builder::TypeScriptBuilder,
    formatter::TypeScriptFormatter,
    highlighter::{HighlightKind, Highlighter, TypeScriptHighlighter},
    kind::TypeScriptSyntaxKind,
    language::TypeScriptLanguage,
    lexer::TypeScriptLexer,
    lsp::TypeScriptLanguageService,
    parser::TypeScriptParser,
};

#[cfg(feature = "mcp")]
pub use crate::mcp::serve_typescript_mcp;
