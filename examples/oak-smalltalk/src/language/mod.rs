use oak_core::Language;

/// Smalltalk 语言定义
#[derive(Debug, Clone)]
pub struct SmalltalkLanguage;

impl SmalltalkLanguage {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SmalltalkLanguage {
    fn default() -> Self {
        Self::new()
    }
}

impl Language for SmalltalkLanguage {
    type SyntaxKind = crate::kind::SmalltalkKind;
}