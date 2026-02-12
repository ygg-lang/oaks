//! Jinja2 template parser library.
//!
//! This crate provides a parser for Jinja2 templates, built on top of the Oak framework.
//! It includes:
//! - Lexer (`JinjaLexer`)
//! - Parser (`JinjaParser`)
//! - Abstract Syntax Tree (`ast`)
//! - Language support (`JinjaLanguage`)

pub mod ast;
pub mod builder;
pub mod language;
pub mod lexer;
pub mod parser;

pub use language::JinjaLanguage;
pub use lexer::{JinjaLexer, token_type::JinjaTokenType};
pub use parser::{JinjaParser, element_type::JinjaElementType};
