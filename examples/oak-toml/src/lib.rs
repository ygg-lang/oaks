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
#[cfg(feature = "lsp")]
pub mod lsp;
#[cfg(feature = "mcp-stdio")]
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
    parser::TomlParser,
};

pub fn parse(toml: &str) -> Result<crate::ast::TomlRoot, String> {
    use oak_core::{Builder, SourceText, parser::session::ParseSession};
    let language = TomlLanguage::default();
    let builder = TomlBuilder::new(&language);
    let source = SourceText::new(toml.to_string());
    let mut cache = ParseSession::default();
    let result = builder.build(&source, &[], &mut cache);
    result.result.map_err(|e| format!("{:?}", e))
}

#[cfg(feature = "lsp")]
pub use crate::lsp::TomlLanguageService;

#[cfg(feature = "mcp-stdio")]
pub use crate::mcp::serve_toml_mcp;
