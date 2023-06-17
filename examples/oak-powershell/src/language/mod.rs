use crate::kind::PowerShellSyntaxKind;
use oak_core::Language;

pub struct PowerShellLanguage {}

impl Language for PowerShellLanguage {
    type SyntaxKind = PowerShellSyntaxKind;
}
