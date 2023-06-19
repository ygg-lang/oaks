use oak_core::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PrologLanguage;

impl Language for PrologLanguage {
    type SyntaxKind = crate::kind::PrologSyntaxKind;
    type TypedRoot = crate::ast::SourceFile;
}
