use crate::{kind::CppSyntaxKind, lexer::CppLexer};
use oak_core::Language;

/// C++ 语言实现
#[derive(Debug, Clone)]
pub struct CppLanguage;

impl Language for CppLanguage {
    type SyntaxKind = CppSyntaxKind;
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
