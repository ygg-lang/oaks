use crate::{lexer::CSharpTokenType, parser::CSharpElementType};
use oak_core::{Language, LanguageCategory};

/// C# 语言实现
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CSharpLanguage;

impl Language for CSharpLanguage {
    const NAME: &'static str = "C#";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = CSharpTokenType;
    type ElementType = CSharpElementType;
    type TypedRoot = ();
}
