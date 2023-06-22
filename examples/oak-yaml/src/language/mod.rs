use oak_core::{Language, LanguageCategory};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct YamlLanguage;

impl Language for YamlLanguage {
    const NAME: &'static str = "yaml";
    const CATEGORY: LanguageCategory = LanguageCategory::Config;

    type TokenType = crate::kind::YamlSyntaxKind;
    type ElementType = crate::kind::YamlSyntaxKind;
    type TypedRoot = ();
}
