#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub mod ast;

mod builder;
pub mod kind;
pub mod language;
pub mod lexer;
pub mod parser;

pub mod engine;
mod formatter;
pub mod highlighter;
#[cfg(feature = "lsp")]
pub mod lsp;
#[cfg(feature = "lsp")]
pub mod mcp;

pub use crate::{ast::TailwindRoot, builder::TailwindBuilder, engine::TailwindEngine, formatter::TailwindFormatter, highlighter::TailwindHighlighter, language::TailwindLanguage, lexer::TailwindLexer, parser::TailwindParser};

#[cfg(feature = "lsp")]
pub use crate::lsp::TailwindLanguageService;

#[cfg(feature = "mcp-stdio")]
pub use crate::mcp::serve_tailwind_mcp;
