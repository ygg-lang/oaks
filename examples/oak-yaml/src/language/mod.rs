use crate::kind::YamlSyntaxKind;
use oak_core::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct YamlLanguage;

impl Language for YamlLanguage {
    type SyntaxKind = YamlSyntaxKind;
    type TypedRoot = ();
}
