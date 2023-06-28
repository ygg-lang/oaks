#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![feature(portable_simd)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;

// pub mod formatter;

/// Type definitions module.
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

pub use crate::{
    ast::{JsonRoot, JsonValueNode},
    builder::JsonBuilder,
    language::JsonLanguage,
    lexer::JsonLexer,
    parser::JsonParser,
};

/// Highlighter implementation.
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::JsonHighlighter;

#[cfg(feature = "serde")]
pub use crate::language::to_string;

#[cfg(feature = "serde")]
pub use crate::language::from_str;

/// Parses a JSON string into a `JsonValueNode` AST.
pub fn parse(json: &str) -> Result<crate::ast::JsonValueNode, String> {
    use oak_core::{Builder, parser::session::ParseSession, source::SourceText};
    let language = JsonLanguage::default();
    let builder = JsonBuilder::new(&language);
    let source = SourceText::new(json.to_string());
    let mut cache = ParseSession::default();
    let result = builder.build(&source, &[], &mut cache);
    result.result.map(|root| root.value).map_err(|e| format!("{:?}", e))
}

pub use oak_macros::json;

/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::JsonLanguageService;
// #[cfg(feature = "oak-pretty-print")]
// pub use crate::lsp::formatter::JsonFormatter;

/// MCP service implementation.
#[cfg(feature = "mcp")]
pub use crate::mcp::serve_json_mcp;
pub use lexer::token_type::JsonTokenType;
pub use parser::element_type::JsonElementType;
