use oak_core::{Language, LanguageCategory};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MarkdownLanguage {
    pub allow_math: bool,
}

impl Language for MarkdownLanguage {
    const NAME: &'static str = "markdown";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;

    type TokenType = crate::kind::MarkdownSyntaxKind;
    type ElementType = crate::kind::MarkdownSyntaxKind;
    type TypedRoot = crate::ast::MarkdownRoot;
}

impl Default for MarkdownLanguage {
    fn default() -> Self {
        Self { allow_math: false }
    }
}
