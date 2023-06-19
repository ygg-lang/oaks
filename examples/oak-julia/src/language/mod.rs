use crate::JuliaSyntaxKind;
use oak_core::Language;

/// Julia 语言实现
#[derive(Debug)]
pub struct JuliaLanguage {
    pub allow_comment: bool,
}

impl Language for JuliaLanguage {
    type SyntaxKind = JuliaSyntaxKind;
    type TypedRoot = ();
}

impl Default for JuliaLanguage {
    fn default() -> Self {
        Self { allow_comment: true }
    }
}
