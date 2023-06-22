// use crate::kind::WitSyntaxKind;
use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WitLanguage {}

impl Language for WitLanguage {
    const NAME: &'static str = "wit-component";
    const CATEGORY: LanguageCategory = LanguageCategory::Dsl;

    type TokenType = crate::kind::WitSyntaxKind;
    type ElementType = crate::kind::WitSyntaxKind;
    type TypedRoot = ();
}
