#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
mod builder;
mod formatter;
pub mod highlighter;
pub mod kind;
pub mod language;
pub mod lexer;
pub mod lsp;
pub mod mcp;
pub mod parser;

pub use crate::{builder::SmalltalkBuilder, formatter::SmalltalkFormatter, highlighter::SmalltalkHighlighter, kind::SmalltalkSyntaxKind, language::SmalltalkLanguage, lexer::SmalltalkLexer, lsp::SmalltalkLanguageService, parser::SmalltalkParser};

pub use crate::mcp::serve_smalltalk_mcp;
