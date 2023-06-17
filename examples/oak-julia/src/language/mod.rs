use crate::JuliaSyntaxKind;
use oak_core::Language;

/// Julia 语言实现
pub struct JuliaLanguage;

impl Language for JuliaLanguage {
    type SyntaxKind = JuliaSyntaxKind;
}
