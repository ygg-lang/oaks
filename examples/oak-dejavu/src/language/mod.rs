#![doc = include_str!("readme.md")]
use crate::{ast::DejavuRoot, lexer::token_type::DejavuSyntaxKind};
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// Syntax modes for Dejavu parser.
pub enum SyntaxMode {
    /// Programming mode: Standard .vk file
    Programming,
    /// Template mode: The entire file is a template environment
    Template,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// The Dejavu programming language definition.
pub struct DejavuLanguage {
    /// Current syntax mode
    pub syntax_mode: SyntaxMode,
}

impl Default for DejavuLanguage {
    fn default() -> Self {
        Self { syntax_mode: SyntaxMode::Template }
    }
}

impl Language for DejavuLanguage {
    const NAME: &'static str = "dejavu";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = DejavuSyntaxKind;
    type ElementType = DejavuSyntaxKind;
    type TypedRoot = DejavuRoot;
}
