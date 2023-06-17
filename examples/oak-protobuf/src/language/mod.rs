use crate::kind::ProtobufSyntaxKind;
use oak_core::Language;

pub struct ProtobufLanguage {}

impl Language for ProtobufLanguage {
    type SyntaxKind = ProtobufSyntaxKind;
}
