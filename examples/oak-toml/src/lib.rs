#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;
/// Syntax kind module.
/// Language configuration module.
pub mod language;
/// Lexer module.
pub mod lexer;
/// LSP module.
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// MCP module.
#[cfg(feature = "mcp")]
pub mod mcp;

/// Parser module.
pub mod parser;

/// Highlighter implementation.
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::TomlHighlighter;
pub use crate::{
    ast::TomlRoot,
    builder::TomlBuilder,
    language::TomlLanguage,
    lexer::{TomlLexer, token_type::TomlTokenKind as TomlSyntaxKind},
    parser::{TomlParser, parse, parse_with_config},
};
pub use oak_core::{Builder, TokenType};

/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::TomlLanguageService;
#[cfg(feature = "oak-pretty-print")]
pub use crate::lsp::formatter::TomlFormatter;

#[cfg(feature = "mcp")]
// pub use crate::mcp::serve_toml_mcp;

/// Serializes a Rust type into a TOML string.
#[cfg(feature = "serde")]
pub use crate::language::to_string;

/// Deserializes a TOML string into a Rust type.
#[cfg(feature = "serde")]
pub use crate::language::from_str;
pub use crate::parser::element_type::TomlElementType as ElementType;
