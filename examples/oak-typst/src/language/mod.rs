use crate::{ast::TypstRoot, kind::TypstSyntaxKind};
use oak_core::Language;

/// Typst 语言定义
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypstLanguage;

impl Language for TypstLanguage {
    const NAME: &'static str = "typst";
    type TokenType = TypstSyntaxKind;
    type ElementType = TypstSyntaxKind;
    type TypedRoot = TypstRoot;
}
