use crate::{ast::GoRoot, kind::GoLangSyntaxKind};
use oak_core::Language;

/// Go 语言实现
#[derive(Debug, Clone, Default)]
pub struct GoLangLanguage {}

impl Language for GoLangLanguage {
    type SyntaxKind = GoLangSyntaxKind;
    type TypedRoot = GoRoot;
}
