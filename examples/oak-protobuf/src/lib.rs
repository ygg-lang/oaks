#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]
//! Protobuf support for the Oak language framework.

pub mod ast;
pub mod builder;

pub mod language;
pub mod lexer;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// MCP module.
#[cfg(feature = "mcp")]
pub mod mcp;

pub mod parser;

pub use crate::{
    ast::ProtobufRoot,
    builder::ProtobufBuilder,
    language::ProtobufLanguage,
    // crate::lsp::highlighter::{HighlightKindHighlighterProtobufHighlighter},
    lexer::token_type::ProtobufTokenType,
    parser::ProtobufParser,
};

#[cfg(feature = "lsp")]
pub use crate::lsp::ProtobufLanguageService;

// #[cfg(feature = "mcp")]
// pub use crate::mcp::serve_protobuf_mcp;
pub use parser::element_type::ProtobufElementType;
