use oak_core::{Language, LanguageCategory};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ElixirLanguage;

impl Language for ElixirLanguage {
    const NAME: &'static str = "elixir";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::ElixirSyntaxKind;
    type ElementType = crate::kind::ElixirSyntaxKind;
    type TypedRoot = ();
}
