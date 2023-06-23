#![feature(new_range_api)]
#![warn(missing_docs)]
//! Mojo support for the Oak language framework.

/// AST module.
pub mod ast;
/// Builder module.
pub mod builder;
/// Lexer module.
pub mod lexer;
/// Parser module.
pub mod parser;

pub use lexer::token_type::MojoTokenType as TokenType;
pub use parser::element_type::MojoElementType as ElementType;

use oak_core::Language;

/// Mojo language definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MojoLanguage;

impl Language for MojoLanguage {
    const NAME: &'static str = "mojo";
    type TokenType = lexer::token_type::MojoTokenType;
    type ElementType = parser::element_type::MojoElementType;
    type TypedRoot = Vec<ast::MojoStatement>;
}
