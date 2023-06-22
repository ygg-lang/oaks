#![feature(new_range_api)]
#![doc = include_str!("../readme.md")]

pub mod ast;
pub mod builder;
pub mod errors;
pub mod frontend;
pub mod highlighter;
pub mod kind;
pub mod language;
pub mod lexer;
pub mod lsp;
pub mod mcp;
pub mod parser;

// Re-exports
pub use crate::{ast::PythonRoot, builder::PythonBuilder, frontend::PythonFrontend, highlighter::PythonHighlighter, kind::PythonSyntaxKind, language::PythonLanguage, lexer::PythonLexer, lsp::PythonLanguageService, parser::PythonParser};

pub use crate::mcp::serve_python_mcp;
