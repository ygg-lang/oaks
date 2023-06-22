#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub mod ast;
pub mod builder;
pub mod errors;
pub mod formatter;
pub mod highlighter;
pub mod kind;
pub mod language;
pub mod lexer;
pub mod lsp;
#[cfg(feature = "mcp")]
pub mod mcp;
pub mod parser;

// Re-export main types for convenience
pub use crate::{
    builder::TomlBuilder,
    formatter::TomlFormatter,
    highlighter::TomlHighlighter,
    kind::{TomlSyntaxKind, TomlTokenKind},
    language::TomlLanguage,
    lexer::TomlLexer,
    lsp::TomlLanguageService,
    parser::TomlParser,
};

#[cfg(feature = "mcp")]
pub use crate::mcp::serve_toml_mcp;
