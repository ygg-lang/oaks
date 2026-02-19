use oak_core::{Language, LanguageCategory};

/// The reStructuredText language definition for Oaks.
#[derive(Debug, Clone, Default)]
pub struct RstLanguage {
    /// Whether to allow directives.
    pub allow_directives: bool,
    /// Whether to allow substitutions.
    pub allow_substitutions: bool,
    /// Whether to allow roles.
    pub allow_roles: bool,
    /// Whether to allow footnotes.
    pub allow_footnotes: bool,
    /// Whether to allow citations.
    pub allow_citations: bool,
    /// Whether to allow admonitions.
    pub allow_admonitions: bool,
}

impl Language for RstLanguage {
    const NAME: &'static str = "restructuredtext";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;

    type TokenType = crate::lexer::token_type::RstTokenType;
    type ElementType = crate::parser::element_type::RstElementType;
    type TypedRoot = crate::ast::RstRoot;
}
