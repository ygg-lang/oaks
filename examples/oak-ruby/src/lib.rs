#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub mod ast;
pub mod kind;
pub mod language;
pub mod lexer;
pub mod parser;

mod builder;
mod formatter;
pub mod highlighter;
pub mod lsp;
#[cfg(feature = "mcp")]
pub mod mcp;

pub use crate::{builder::RubyBuilder, formatter::RubyFormatter, highlighter::RubyHighlighter, kind::RubySyntaxKind, language::RubyLanguage, lexer::RubyLexer, lsp::RubyLanguageService, parser::RubyParser};

#[cfg(feature = "mcp")]
pub use crate::mcp::serve_ruby_mcp;

#[cfg(all(feature = "mcp", feature = "axum"))]
pub use crate::mcp::serve_ruby_mcp_axum;
