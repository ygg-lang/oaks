use oak_core::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BashLanguage;

impl Language for BashLanguage {
    type SyntaxKind = crate::kind::BashSyntaxKind;
    type TypedRoot = crate::ast::SourceFile;
}
