#![doc = include_str!("readme.md")]
#![allow(incomplete_features)]
#![feature(lazy_type_alias)]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

/// Tree building and construction utilities for the Oak parsing framework.
///
/// This module provides the [`Builder`] trait and related utilities for constructing
/// kind trees from parsed tokens. It includes incremental building capabilities
/// and tree manipulation utilities for efficient parsing operations.
pub mod builder;
/// Error handling and diagnostic reporting for the parsing system.
pub mod errors;
/// Utility functions and helper types for parsing operations.
pub mod helpers;
/// Syntax kind definitions for tokens and nodes in the parsing system.
pub mod kinds;
/// Language definition trait for coordinating language-specific components.
pub mod language;
/// Lexical analysis and tokenization functionality.
pub mod lexer;
/// Parsing functionality for converting tokens to kind trees.
pub mod parser;
/// Source text management and location tracking.
pub mod source;
/// Tree structures for representing kind trees (green and red trees).
pub mod tree;
/// Tree traversal utilities for visiting and transforming kind trees.
///
/// This module provides visitor patterns and traversal utilities for walking
/// through kind trees, enabling operations like kind highlighting,
/// code analysis, and tree transformations.
pub mod visitor;

pub use crate::{
    builder::Builder,
    errors::{LexResult, OakDiagnostics, OakError, OakErrorKind, ParseResult},
    kinds::SyntaxKind,
    language::Language,
    lexer::{Lexer, LexerState, Token},
    parser::{Associativity, OperatorInfo, Parser, PrattParser, Precedence},
    source::{SourceLocation, SourceText},
    tree::*,
};
