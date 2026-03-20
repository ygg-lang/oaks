use crate::{lexer::token_type::SvelteTokenType, parser::element_type::SvelteElementType};
use oak_core::{Language, LanguageCategory};

/// Svelte language definition.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SvelteLanguage {
    /// Tag start
    pub tag_start: String,
    /// Tag end
    pub tag_end: String,
}

impl Default for SvelteLanguage {
    fn default() -> Self {
        Self { tag_start: "{".to_string(), tag_end: "}".to_string() }
    }
}

impl Language for SvelteLanguage {
    const NAME: &'static str = "svelte";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;
    type TokenType = SvelteTokenType;
    type ElementType = SvelteElementType;
    type TypedRoot = crate::ast::SvelteRoot;
}
