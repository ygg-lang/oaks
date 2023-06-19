use crate::kind::GroovySyntaxKind;
use oak_core::Language;

/// Groovy 语言配置
#[derive(Debug, Clone)]
pub struct GroovyLanguage;

impl Default for GroovyLanguage {
    fn default() -> Self {
        Self
    }
}

impl Language for GroovyLanguage {
    type SyntaxKind = GroovySyntaxKind;
    type TypedRoot = ();
}

impl GroovyLanguage {
    pub fn new() -> Self {
        Self::default()
    }
}
