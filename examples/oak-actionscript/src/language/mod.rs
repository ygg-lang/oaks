#![doc = include_str!("readme.md")]
use crate::ast::ActionScriptRoot;
use oak_core::{Language, LanguageCategory};

/// ActionScript language configuration and metadata.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ActionScriptLanguage {
    /// Enable strict mode
    pub strict_mode: bool,
    /// Enable AS3 specific features
    pub as3_features: bool,
}

impl Default for ActionScriptLanguage {
    fn default() -> Self {
        Self { strict_mode: true, as3_features: true }
    }
}

impl Language for ActionScriptLanguage {
    const NAME: &'static str = "actionscript";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::ActionScriptTokenType;
    type ElementType = crate::parser::element_type::ActionScriptElementType;
    type TypedRoot = ActionScriptRoot;
}
