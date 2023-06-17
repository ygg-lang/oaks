use crate::{lexer::IniLexer, syntax::IniSyntaxKind};
use oak_core::Language;

/// Ini 语言定义
#[derive(Debug, Clone)]
pub struct IniLanguage;

impl Language for IniLanguage {
    type SyntaxKind = IniSyntaxKind;
}

impl IniLanguage {
    pub fn new() -> Self {
        Self
    }

    pub fn lexer(&self) -> IniLexer<'_> {
        IniLexer::new(self)
    }
}
