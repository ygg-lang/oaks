use oak_core::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ProtobufLanguage;

impl Language for ProtobufLanguage {
    type SyntaxKind = crate::kind::ProtobufSyntaxKind;
    type TypedRoot = crate::ast::SourceFile;
}
