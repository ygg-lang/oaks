#![doc = include_str!("readme.md")]
use crate::{ast::ValkyrieRoot, lexer::token_type::ValkyrieSyntaxKind};
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// Syntax modes for Valkyrie parser.
pub enum SyntaxMode {
    /// Programming mode: Standard .vk file
    Programming,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// The Valkyrie programming language definition.
pub struct ValkyrieLanguage {
    /// Current syntax mode
    pub syntax_mode: SyntaxMode,
}

impl Default for ValkyrieLanguage {
    fn default() -> Self {
        Self { syntax_mode: SyntaxMode::Programming }
    }
}

impl Language for ValkyrieLanguage {
    const NAME: &'static str = "valkyrie";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = ValkyrieSyntaxKind;
    type ElementType = ValkyrieSyntaxKind;
    type TypedRoot = ValkyrieRoot;
}
