#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub mod ast;
pub mod builder;
#[cfg(feature = "oak-highlight")]
pub mod highlighter;
pub mod kind;
pub mod language;
pub mod lexer;
pub mod lsp;
#[cfg(feature = "mcp")]
pub mod mcp;
pub mod parser;

pub use crate::{ast::PythonRoot, builder::PythonBuilder, kind::PythonSyntaxKind, language::PythonLanguage, lexer::PythonLexer, lsp::PythonLanguageService, parser::PythonParser};

#[cfg(feature = "oak-highlight")]
pub use crate::highlighter::{HighlightKind, Highlighter, PythonHighlighter};

#[cfg(feature = "mcp")]
pub use crate::mcp::serve_python_mcp;
