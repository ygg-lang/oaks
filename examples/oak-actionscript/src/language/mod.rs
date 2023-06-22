use crate::{ast::ActionScriptRoot, lexer::ActionScriptTokenType, parser::ActionScriptElementType};
use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

/// ActionScript 语言配置和元数据。
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

    type TokenType = ActionScriptTokenType;
    type ElementType = ActionScriptElementType;
    type TypedRoot = ActionScriptRoot;
}
