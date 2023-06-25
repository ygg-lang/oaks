#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// The type of the root node for PHP AST.
pub type TypedRoot = crate::ast::PhpRoot;

/// PHP language implementation.
///
/// This struct implements the [`Language`] trait for the PHP language.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PhpLanguage {
    /// Start tag, e.g., "<?php"
    pub tag_start: String,
    /// End tag, e.g., "?>"
    pub tag_end: String,
    /// Short start tag, e.g., "<?"
    pub short_tag_start: String,
    /// Echo tag start, e.g., "<?="
    pub echo_tag_start: String,
}

impl Default for PhpLanguage {
    fn default() -> Self {
        Self { tag_start: "<?php".to_string(), tag_end: "?>".to_string(), short_tag_start: "<?".to_string(), echo_tag_start: "<?=".to_string() }
    }
}

impl PhpLanguage {
    /// Creates a new `PhpLanguage` instance.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Language for PhpLanguage {
    const NAME: &'static str = "php";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::PhpTokenType;
    type ElementType = crate::parser::element_type::PhpElementType;
    type TypedRoot = crate::ast::PhpRoot;
}
