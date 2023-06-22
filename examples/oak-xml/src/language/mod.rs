use oak_core::{Language, LanguageCategory};

#[derive(Debug, Clone)]
pub struct XmlLanguage {}

impl Default for XmlLanguage {
    fn default() -> Self {
        Self {}
    }
}

impl Language for XmlLanguage {
    const NAME: &'static str = "xml";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;

    type TokenType = crate::kind::XmlSyntaxKind;
    type ElementType = crate::kind::XmlSyntaxKind;
    type TypedRoot = ();
}
