use oak_core::{Language, LanguageCategory, UniversalTokenRole};

/// The AsciiDoc language definition for Oaks.
#[derive(Debug, Clone, Default)]
pub struct AsciidocLanguage {
    /// Whether to allow macros.
    pub allow_macros: bool,
    /// Whether to allow attributes.
    pub allow_attributes: bool,
    /// Whether to allow blocks.
    pub allow_blocks: bool,
    /// Whether to allow footnotes.
    pub allow_footnotes: bool,
    /// Whether to allow cross-references.
    pub allow_cross_references: bool,
    /// Whether to allow include directives.
    pub allow_includes: bool,
}

impl Language for AsciidocLanguage {
    const NAME: &'static str = "asciidoc";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;

    type TokenType = crate::lexer::token_type::AsciidocTokenType;
    type ElementType = crate::parser::element_type::AsciidocElementType;
    type TypedRoot = crate::ast::AsciidocRoot;
}
