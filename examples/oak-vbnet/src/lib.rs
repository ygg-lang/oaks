#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

use oak_core::builder::Builder;

/// AST module containing high-level VB.NET syntax tree definitions.
pub mod ast;
/// Builder module for converting green trees into high-level AST nodes.
pub mod builder;
/// Language definition and configuration for VB.NET.
pub mod language;
/// Lexer implementation for VB.NET.
pub mod lexer;
/// Parser implementation for VB.NET.
pub mod parser;

pub use ast::VbNetRoot;
pub use builder::VbNetBuilder;
pub use language::VbNetLanguage;
pub use lexer::{VbNetLexer, token_type::VbNetTokenType};
pub use parser::VbNetParser;

/// Parses VB.NET source code into a [VbNetRoot] AST.
///
/// This is a convenience function that initializes the language, builder,
/// and parser to process the source text.
///
/// # Errors
///
/// Returns an [oak_core::OakError] if parsing fails.
pub fn parse(source: &str) -> Result<VbNetRoot, oak_core::OakError> {
    let language = VbNetLanguage::new();
    let builder = VbNetBuilder::new(&language);
    let source_text = oak_core::source::SourceText::new(source);
    let mut session = oak_core::parser::ParseSession::<VbNetLanguage>::default();
    let output = builder.build(&source_text, &[], &mut session);
    output.result
}

/// Parses VB.NET source code into a [VbNetRoot] AST using a caching parse session.
///
/// This function uses a [CachingParseSession] to cache parsed results for improved performance
/// when processing the same content multiple times.
///
/// # Errors
///
/// Returns an [oak_core::OakError] if parsing fails.
pub fn parse_with_cache(source: &str, cache: &mut oak_core::parser::CachingParseSession<VbNetLanguage, oak_core::parser::ParseSession<VbNetLanguage>>) -> Result<VbNetRoot, oak_core::OakError> {
    use oak_core::ParseCache;

    let language = VbNetLanguage::new();
    let builder = VbNetBuilder::new(&language);
    let source_text = oak_core::source::SourceText::new(source);

    // Parse the code
    let output = builder.build(&source_text, &[], cache);

    // Return the result
    output.result
}
