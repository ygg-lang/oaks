#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

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
pub mod mcp;

pub use crate::{builder::SolidityBuilder, formatter::SolidityFormatter, highlighter::SolidityHighlighter, language::SolidityLanguage, lexer::SolidityLexer, lsp::SolidityLanguageService, parser::SolidityParser};

pub use crate::mcp::serve_solidity_mcp;
