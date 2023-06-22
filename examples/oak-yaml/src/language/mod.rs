use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct YamlLanguage {}

impl YamlLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for YamlLanguage {
    const NAME: &'static str = "yaml";
    const CATEGORY: LanguageCategory = LanguageCategory::Config;

    type TokenType = crate::kind::YamlSyntaxKind;
    type ElementType = crate::kind::YamlSyntaxKind;
    type TypedRoot = ();
}
