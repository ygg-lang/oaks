use crate::{ast::CrystalRoot, lexer::CrystalTokenType, parser::CrystalElementType};
use oak_core::{Language, LanguageCategory};

/// Crystal 语言实现
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CrystalLanguage;

impl Language for CrystalLanguage {
    const NAME: &'static str = "Crystal";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = CrystalTokenType;
    type ElementType = CrystalElementType;
    type TypedRoot = CrystalRoot;
}
