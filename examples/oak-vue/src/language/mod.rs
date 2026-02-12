use crate::{ast::VueRoot, lexer::token_type::VueTokenType, parser::element_type::VueElementType};
use oak_core::{Language, LanguageCategory};

/// Vue language definition.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VueLanguage {
    /// Interpolation start
    pub interpolation_start: String,
    /// Interpolation end
    pub interpolation_end: String,
}

impl Default for VueLanguage {
    fn default() -> Self {
        Self { interpolation_start: "{{".to_string(), interpolation_end: "}}".to_string() }
    }
}

impl VueLanguage {
    /// Returns the interpolation start delimiter.
    pub fn interpolation_start(&self) -> &str {
        &self.interpolation_start
    }

    /// Returns the interpolation end delimiter.
    pub fn interpolation_end(&self) -> &str {
        &self.interpolation_end
    }
}

impl Language for VueLanguage {
    const NAME: &'static str = "vue";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;

    type TokenType = VueTokenType;
    type ElementType = VueElementType;
    type TypedRoot = VueRoot;
}
