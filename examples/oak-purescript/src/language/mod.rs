#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// PureScript 语言实现
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PurescriptLanguage {
    /// 语言配置
    pub config: (),
}

impl Default for PurescriptLanguage {
    fn default() -> Self {
        Self { config: () }
    }
}

impl Language for PurescriptLanguage {
    const NAME: &'static str = "purescript";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::PurescriptTokenType;
    type ElementType = crate::parser::element_type::PurescriptElementType;
    type TypedRoot = ();
}
