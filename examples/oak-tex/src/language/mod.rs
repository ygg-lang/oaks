use crate::{ast::TexRoot, kind::TexSyntaxKind};
use oak_core::Language;

/// TeX 语言定义
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TexLanguage;

impl Language for TexLanguage {
    const NAME: &'static str = "tex";
    type TokenType = TexSyntaxKind;
    type ElementType = TexSyntaxKind;
    type TypedRoot = TexRoot;
}
