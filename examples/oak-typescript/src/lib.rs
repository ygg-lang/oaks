#![feature(new_range_api)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub mod ast;
mod builder;
mod formatter;
pub mod highlighter;
pub mod kind;
pub mod language;
pub mod lexer;
pub mod lsp;
pub mod parser;

#[cfg(feature = "mcp")]
pub mod mcp;

pub use crate::{ast::TypeScriptRoot, builder::TypeScriptBuilder, formatter::TypeScriptFormatter, highlighter::TypeScriptHighlighter, language::TypeScriptLanguage, lexer::TypeScriptLexer, lsp::TypeScriptLanguageService, parser::TypeScriptParser};

#[cfg(feature = "mcp")]
pub use crate::mcp::serve_typescript_mcp;

#[cfg(all(feature = "mcp", feature = "axum"))]
pub use crate::mcp::serve_typescript_mcp_axum;
