use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct ElixirLanguage {}

impl ElixirLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for ElixirLanguage {
    const NAME: &'static str = "elixir";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::ElixirSyntaxKind;
    type ElementType = crate::kind::ElixirSyntaxKind;
    type TypedRoot = ();
}
