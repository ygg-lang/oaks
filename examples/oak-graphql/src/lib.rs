#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]
//! GraphQL support for the Oak language framework.

/// AST module for GraphQL.
pub mod ast;
/// Builder module for GraphQL.
pub mod builder;
pub use builder::GraphQLBuilder;
/// Language configuration module for GraphQL.
pub mod language;
pub use crate::language::GraphQLLanguage;
/// Lexer module for GraphQL.
pub mod lexer;
/// LSP module for GraphQL.
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
/// Parser module for GraphQL.
pub mod parser;
pub use lexer::token_type::GraphQLTokenType;
pub use parser::element_type::GraphQLElementType;

/// Parses a GraphQL string.
pub fn parse(graphql: &str) -> Result<crate::ast::GraphQLRoot, String> {
    use oak_core::{Builder, parser::session::ParseSession, source::SourceText};
    let language = GraphQLLanguage {};
    let builder = GraphQLBuilder::new(&language);
    let source = SourceText::new(graphql.to_string());
    let mut cache = ParseSession::default();
    let result = builder.build(&source, &[], &mut cache);
    result.result.map_err(|e| format!("{:?}", e))
}
