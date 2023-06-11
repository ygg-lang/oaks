#![feature(new_range_api)]
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

extern crate alloc;

pub mod errors;
pub mod helpers;
pub mod kinds;
/// Language definition and coordination traits for different programming languages.
///
/// This module provides the [`Language`] trait which serves as the foundation for
/// defining programming languages within the incremental parsing system. It coordinates
/// all language-related types and behaviors.
pub mod language;

/// Lexical analysis and tokenization for converting source text into tokens.
///
/// This module provides traits and utilities for converting source text into
/// sequences of tokens that can be consumed by parsers. It includes support
/// for common lexical patterns and incremental tokenization.
pub mod lexer;

/// Parsing algorithms and combinators for constructing kind trees.
///
/// This module provides the core parsing infrastructure including the [`Parser`]
/// trait for building kind trees, operator precedence parsing utilities,
/// and incremental parsing capabilities through the [`IncrementalParser`] trait.
pub mod parser;
pub mod source;
pub mod tree;
/// Tree traversal utilities for visiting and transforming kind trees.
///
/// This module provides visitor patterns and traversal utilities for walking
/// through kind trees, enabling operations like kind highlighting,
/// code analysis, and tree transformations.
pub mod visitor;

pub use crate::{
    errors::{OakError, OakErrorKind, LexResult, ParseResult, OakDiagnostics},
    kinds::SyntaxKind,
    language::Language,
    lexer::{Lexer, LexerState, Token},
    parser::{Associativity, IncrementalParser, OperatorInfo, Parser, PrattParser, Precedence},
    source::{SourceLocation, SourceText},
    tree::*,
};
