use crate::kind::IdlSyntaxKind;
use oak_core::Language;

#[derive(Debug, Default)]
pub struct IdlLanguage {}

impl Language for IdlLanguage {
    type SyntaxKind = IdlSyntaxKind;
    type TypedRoot = (); // TODO: 添加 AST 根类型
}
