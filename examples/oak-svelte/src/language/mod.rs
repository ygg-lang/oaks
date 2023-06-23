use crate::{lexer::token_type::SvelteTokenType, parser::element_type::SvelteElementType};
use oak_core::{Language, LanguageCategory};

/// Svelte language definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SvelteLanguage;

impl Language for SvelteLanguage {
    const NAME: &'static str = "svelte";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;
    type TokenType = SvelteTokenType;
    type ElementType = SvelteElementType;
    type TypedRoot = ();
}
