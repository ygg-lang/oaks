#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
//! Svelte support for the Oak language framework.

// pub mod ast;
// // pub mod builder;

/// Language module.
pub mod language;

/// Lexer module.
pub mod lexer;

/// Parser module.
pub mod parser;

/// LSP module.
#[cfg(feature = "lsp")]
pub mod lsp;

pub use crate::{
    language::SvelteLanguage,
    lexer::{SvelteLexer, token_type::SvelteTokenType},
    parser::SvelteParser,
};

#[cfg(feature = "lsp")]
pub use lsp::SvelteLanguageService;

pub use crate::{lexer::token_type::SvelteTokenType as TokenType, parser::element_type::SvelteElementType as ElementType};
