#![doc = include_str!("readme.md")]
use crate::ast::ValkyrieRoot;
use oak_core::{Language, LanguageCategory};
use oak_dejavu::language::{SyntaxMode, TemplateConfig};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// The Valkyrie programming language definition.
pub struct ValkyrieLanguage {
    /// Current syntax mode
    pub syntax_mode: SyntaxMode,
    /// Template configuration
    pub template: TemplateConfig,
}

impl Default for ValkyrieLanguage {
    fn default() -> Self {
        Self { syntax_mode: SyntaxMode::Programming, template: TemplateConfig::default() }
    }
}

impl Language for ValkyrieLanguage {
    const NAME: &'static str = "valkyrie";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::ValkyrieSyntaxKind;
    type ElementType = crate::lexer::token_type::ValkyrieSyntaxKind;
    type TypedRoot = ValkyrieRoot;
}
