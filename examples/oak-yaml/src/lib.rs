#![feature(new_range_api)]
// pub mod ast;
mod builder;
pub mod highlighter;
pub mod kind;
pub mod language;
pub mod lexer;
pub mod parser;
// pub mod syntax;

mod formatter;
pub mod lsp;
#[cfg(feature = "mcp")]
pub mod mcp;

pub use crate::{builder::YamlBuilder, formatter::YamlFormatter, highlighter::YamlHighlighter, kind::YamlSyntaxKind, language::YamlLanguage, lexer::YamlLexer, lsp::YamlLanguageService, parser::YamlParser};

#[cfg(feature = "mcp")]
pub use crate::mcp::serve_yaml_mcp;

#[cfg(all(feature = "mcp", feature = "axum"))]
pub use crate::mcp::serve_yaml_mcp_axum;
