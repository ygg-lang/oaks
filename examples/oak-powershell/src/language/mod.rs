use crate::kind::PowerShellSyntaxKind;
use oak_core::Language;

pub type TypedRoot = crate::ast::PowerShellRoot;

#[derive(Debug, Default)]
pub struct PowerShellLanguage;

impl Language for PowerShellLanguage {
    type SyntaxKind = PowerShellSyntaxKind;
    type TypedRoot = TypedRoot;
}
