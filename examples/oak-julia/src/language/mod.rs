use oak_core::{Language, LanguageCategory};

/// Julia 语言实现
#[derive(Debug)]
pub struct JuliaLanguage {
    pub allow_comment: bool,
}

impl Language for JuliaLanguage {
    const NAME: &'static str = "julia";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::JuliaSyntaxKind;
    type ElementType = crate::kind::JuliaSyntaxKind;
    type TypedRoot = crate::ast::JuliaRoot;
}

impl Default for JuliaLanguage {
    fn default() -> Self {
        Self { allow_comment: true }
    }
}
