#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
pub mod ast;
mod builder;
pub mod kind;
pub mod language;
pub mod lexer;
pub mod parser;

mod formatter;
pub mod highlighter;
pub mod lsp;
pub mod mcp;

pub use crate::{ast::TypstRoot, builder::TypstBuilder, formatter::TypstFormatter, highlighter::TypstHighlighter, language::TypstLanguage, lexer::TypstLexer, lsp::TypstLanguageService, parser::TypstParser};

#[cfg(feature = "io-std")]
pub use crate::mcp::serve_typst_mcp;
