#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// SQL language implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SqlLanguage {
    /// Whether it is case sensitive.
    pub case_sensitive: bool,
    /// Whether to allow double-quoted identifiers.
    pub quoted_identifiers: bool,
    /// Whether to allow backtick identifiers.
    pub backtick_identifiers: bool,
    /// Whether to allow bracket identifiers.
    pub bracket_identifiers: bool,
}

impl SqlLanguage {
    /// Creates a new SQL language instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a standard SQL language instance.
    pub fn standard() -> Self {
        Self::default()
    }

    /// Creates a MySQL-style SQL language instance.
    pub fn mysql() -> Self {
        Self { case_sensitive: false, quoted_identifiers: true, backtick_identifiers: true, bracket_identifiers: false }
    }

    /// Creates a PostgreSQL-style SQL language instance.
    pub fn postgresql() -> Self {
        Self { case_sensitive: false, quoted_identifiers: true, backtick_identifiers: false, bracket_identifiers: false }
    }

    /// Creates a SQL Server-style SQL language instance.
    pub fn sqlserver() -> Self {
        Self { case_sensitive: false, quoted_identifiers: true, backtick_identifiers: false, bracket_identifiers: true }
    }
}

impl Default for SqlLanguage {
    fn default() -> Self {
        Self { case_sensitive: false, quoted_identifiers: true, backtick_identifiers: false, bracket_identifiers: false }
    }
}

impl Language for SqlLanguage {
    const NAME: &'static str = "sql";
    const CATEGORY: LanguageCategory = LanguageCategory::Dsl;

    type TokenType = crate::lexer::token_type::SqlTokenType;
    type ElementType = crate::parser::element_type::SqlElementType;
    type TypedRoot = crate::ast::SqlRoot;
}
