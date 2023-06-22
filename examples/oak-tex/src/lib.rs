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

mod formatter;
pub mod highlighter;
pub mod lsp;
#[cfg(feature = "mcp")]
pub mod mcp;

pub use crate::{ast::TexRoot, builder::TexBuilder, formatter::TexFormatter, highlighter::TexHighlighter, language::TexLanguage, lexer::TexLexer, lsp::TexLanguageService, parser::TexParser};

#[cfg(feature = "mcp")]
pub use crate::mcp::serve_tex_mcp;

#[cfg(all(feature = "mcp", feature = "axum"))]
pub use crate::mcp::serve_tex_mcp_axum;
