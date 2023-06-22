#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub mod language;

pub mod ast;
mod builder;
pub mod kind;
pub mod lexer;
pub mod parser;

mod formatter;
pub mod highlighter;
pub mod lsp;
pub mod mcp;

pub use crate::{builder::RBuilder, formatter::RFormatter, highlighter::RHighlighter, kind::RSyntaxKind, language::RLanguage, lexer::RLexer, lsp::RLanguageService, parser::RParser};

pub use crate::mcp::serve_r_mcp;
