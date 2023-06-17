use crate::kind::ActionScriptSyntaxKind;
use oak_core::Language;

/// ActionScript 语言实现
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ActionScriptLanguage;

impl Language for ActionScriptLanguage {
    type SyntaxKind = ActionScriptSyntaxKind;
}
