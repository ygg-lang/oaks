use oak_core::{Language, LanguageCategory};

/// Groovy 语言配置
#[derive(Debug, Clone)]
pub struct GroovyLanguage;

impl Default for GroovyLanguage {
    fn default() -> Self {
        Self
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
