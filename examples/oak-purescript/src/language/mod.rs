use oak_core::{Language, LanguageCategory};

/// PureScript 语言实现
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PurescriptLanguage {
    /// 语言配置
    pub config: (),
}

impl Default for PurescriptLanguage {
    fn default() -> Self {
        Self { config: () }
    }
}

impl Language for PurescriptLanguage {
    const NAME: &'static str = "purescript";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::PurescriptSyntaxKind;
    type ElementType = crate::kind::PurescriptSyntaxKind;
    type TypedRoot = ();
}
