#[doc = include_str!("../readme.md")]
use crate::{ast::AdaRoot, lexer::AdaTokenType, parser::AdaElementType};
use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

/// Ada 语言配置和元数据。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AdaLanguage {
    /// 是否启用 Ada 2022 特性
    pub allow_ada_2022: bool,
    /// 是否启用严格模式
    pub strict_mode: bool,
}

impl AdaLanguage {
    /// 创建新的 Ada 语言配置
    pub fn new() -> Self {
        Self { allow_ada_2022: true, strict_mode: false }
    }
}

impl Default for AdaLanguage {
    fn default() -> Self {
        Self::new()
    }
}

impl Language for AdaLanguage {
    const NAME: &'static str = "ada";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = AdaTokenType;
    type ElementType = AdaElementType;
    type TypedRoot = AdaRoot;
}
