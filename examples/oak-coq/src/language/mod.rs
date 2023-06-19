use crate::{ast::CoqRoot, kind::CoqSyntaxKind, lexer::CoqLexer};
use oak_core::Language;

/// Coq 语言实现
#[derive(Debug, Clone)]
pub struct CoqLanguage;

impl Language for CoqLanguage {
    type SyntaxKind = CoqSyntaxKind;
    type TypedRoot = CoqRoot;
}

impl CoqLanguage {
    /// 创建新的 Coq 语言实例
    pub fn new() -> Self {
        Self
    }

    /// 创建 Coq 词法分析器
    pub fn lexer(&self) -> CoqLexer<'_> {
        CoqLexer::new(self)
    }
}

impl Default for CoqLanguage {
    fn default() -> Self {
        Self::new()
    }
}
