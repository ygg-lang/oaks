#![doc = include_str!("readme.md")]
use crate::{lexer::token_type::CsvTokenType, parser::element_type::CsvElementType};
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Oaks 框架的 CSV 语言实现。
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CsvLanguage {}

impl CsvLanguage {
    /// 创建一个新的 `CsvLanguage` 实例。
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for CsvLanguage {
    const NAME: &'static str = "csv";
    const CATEGORY: LanguageCategory = LanguageCategory::Config;

    type TokenType = CsvTokenType;
    type ElementType = CsvElementType;
    type TypedRoot = ();
}
