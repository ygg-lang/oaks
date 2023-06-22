use oak_core::{Language, LanguageCategory};

/// Erlang 语言配置
#[derive(Debug, Clone, Default)]
pub struct ErlangLanguage;

impl Language for ErlangLanguage {
    const NAME: &'static str = "erlang";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::ErlangSyntaxKind;
    type ElementType = crate::kind::ErlangSyntaxKind;
    type TypedRoot = ();
}
