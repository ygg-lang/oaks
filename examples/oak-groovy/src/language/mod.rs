use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

/// Groovy 语言配置
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GroovyLanguage {}

impl Default for GroovyLanguage {
    fn default() -> Self {
        Self {}
    }
}

impl Language for GroovyLanguage {
    const NAME: &'static str = "groovy";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::GroovySyntaxKind;
    type ElementType = crate::kind::GroovySyntaxKind;
    type TypedRoot = ();
}

impl GroovyLanguage {
    pub fn new() -> Self {
        Self::default()
    }
}
