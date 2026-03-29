use crate::{ast::VxDocument, lexer::token_type::VocTokenType, parser::element_type::VocElementType};
use oak_core::{Language, LanguageCategory};

/// VOC language configuration and metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VocLanguage;

impl VocLanguage {
    /// Creates a new VOC language configuration.
    pub fn new() -> Self {
        Self
    }
}

impl Default for VocLanguage {
    fn default() -> Self {
        Self::new()
    }
}

impl Language for VocLanguage {
    const NAME: &'static str = "voc";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = VocTokenType;
    type ElementType = VocElementType;
    type TypedRoot = VxDocument;
}
