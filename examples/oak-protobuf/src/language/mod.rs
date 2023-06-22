use oak_core::{Language, LanguageCategory};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ProtobufLanguage {}

impl ProtobufLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for ProtobufLanguage {
    const NAME: &'static str = "protobuf";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::ProtobufSyntaxKind;
    type ElementType = crate::kind::ProtobufSyntaxKind;
    type TypedRoot = crate::ast::ProtobufRoot;
}
