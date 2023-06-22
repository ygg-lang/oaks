use crate::{lexer::CssTokenType, parser::CssElementType};
use oak_core::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CssLanguage;

impl Language for CssLanguage {
    const NAME: &'static str = "css";
    type TokenType = CssTokenType;
    type ElementType = CssElementType;
    type TypedRoot = ();
}
