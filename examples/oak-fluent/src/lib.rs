#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;
/// Language configuration module.
pub mod language;
/// Lexer module.
pub mod lexer;
/// Parser module.
pub mod parser;

/// Fluent translator.
pub mod translator;

pub use crate::{
    ast::FluentRoot,
    builder::FluentBuilder,
    language::FluentLanguage,
    lexer::{FluentLexer, token_type::FluentTokenKind as FluentSyntaxKind},
    parser::{FluentParser, parse, parse_with_config},
    translator::Translator,
};

pub use oak_core::{Builder, TokenType};

/// Deserializes a Fluent string into a Fluent AST.
pub use crate::language::from_str;

/// Serializes a Fluent AST into a Fluent string.
pub use crate::language::to_string;
