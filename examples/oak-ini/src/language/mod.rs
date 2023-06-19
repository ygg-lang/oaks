use crate::syntax::IniSyntaxKind;
use oak_core::Language;

/// Ini 语言定义
#[derive(Debug, Default)]
pub struct IniLanguage;

impl Language for IniLanguage {
    type SyntaxKind = IniSyntaxKind;
    type TypedRoot = (); // TODO: 添加 AST 根类型
}
