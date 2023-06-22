#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
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

pub use crate::{builder::SchemeBuilder, formatter::SchemeFormatter, highlighter::SchemeHighlighter, kind::SchemeSyntaxKind, language::SchemeLanguage, lexer::SchemeLexer, lsp::SchemeLanguageService};

#[cfg(feature = "mcp")]
pub use crate::mcp::serve_scheme_mcp;

#[cfg(all(feature = "mcp", feature = "axum"))]
pub use crate::mcp::serve_scheme_mcp_axum;
