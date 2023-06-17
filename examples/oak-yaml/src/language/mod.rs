use crate::kind::YamlSyntaxKind;
use oak_core::Language;

pub struct YamlLanguage {}

impl Language for YamlLanguage {
    type SyntaxKind = YamlSyntaxKind;
}
