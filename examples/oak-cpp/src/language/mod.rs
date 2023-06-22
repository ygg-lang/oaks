use crate::{
    lexer::{CppLexer, CppTokenType},
    parser::CppElementType,
};
use oak_core::{Language, LanguageCategory};

/// C++ 语言实现
#[derive(Debug, Clone)]
pub struct CppLanguage;

impl Language for CppLanguage {
    const NAME: &'static str = "cpp";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = CppTokenType;
    type ElementType = CppElementType;
    type TypedRoot = ();
}

impl CppLanguage {
    /// 创建新的 C++ 语言实例
    pub fn new() -> Self {
        Self
    }

    /// 创建 C++ 词法分析器
    pub fn lexer(&self) -> CppLexer<'_> {
        CppLexer::new(self)
    }
}

impl Default for CppLanguage {
    fn default() -> Self {
        Self::new()
    }
}
