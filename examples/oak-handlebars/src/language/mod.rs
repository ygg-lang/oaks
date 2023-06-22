use oak_core::{Language, LanguageCategory};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HandlebarsLanguage {}

unsafe impl Send for HandlebarsLanguage {}
unsafe impl Sync for HandlebarsLanguage {}

impl Language for HandlebarsLanguage {
    const NAME: &'static str = "handlebars";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;

    type TokenType = crate::kind::HandlebarsSyntaxKind;
    type ElementType = crate::kind::HandlebarsSyntaxKind;
    type TypedRoot = crate::ast::HandlebarsRoot;
}
