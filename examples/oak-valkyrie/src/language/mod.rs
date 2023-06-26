use crate::{ast::ValkyrieRoot, lexer::ValkyrieTokenType, parser::ValkyrieElementType};
use oak_core::{Language, LanguageCategory};
use oak_dejavu::language::{DejavuLanguage, SyntaxMode};

/// Valkyrie language configuration and metadata.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValkyrieLanguage {
    /// The base Dejavu language configuration.
    pub base: DejavuLanguage,
}

impl ValkyrieLanguage {
    /// Creates a new Valkyrie language configuration.
    pub fn new() -> Self {
        Self { base: DejavuLanguage::default() }
    }
}

impl Default for ValkyrieLanguage {
    fn default() -> Self {
        Self::new()
    }
}

impl Language for ValkyrieLanguage {
    const NAME: &'static str = "valkyrie";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = ValkyrieTokenType;
    type ElementType = ValkyrieElementType;
    type TypedRoot = ValkyrieRoot;
}

impl ValkyrieLanguage {
    /// Gets the syntax mode.
    pub fn syntax_mode(&self) -> SyntaxMode {
        self.base.syntax_mode
    }

    /// Gets the template configuration.
    pub fn template(&self) -> &oak_dejavu::language::TemplateConfig {
        &self.base.template
    }
}
