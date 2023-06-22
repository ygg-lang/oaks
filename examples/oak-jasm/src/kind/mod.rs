use crate::syntax::JasmSyntaxKind;
use core::range::Range;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JasmToken {
    pub kind: JasmSyntaxKind,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub text: String,
}

impl JasmToken {
    pub fn new(kind: JasmSyntaxKind, span: Range<usize>, text: String) -> Self {
        Self { kind, span, text }
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    pub fn is_identifier(&self) -> bool {
        matches!(self.kind, JasmSyntaxKind::IdentifierToken)
    }
    pub fn is_keyword(&self) -> bool {
        matches!(
            self.kind,
            JasmSyntaxKind::ClassKw
                | JasmSyntaxKind::VersionKw
                | JasmSyntaxKind::MethodKw
                | JasmSyntaxKind::FieldKw
                | JasmSyntaxKind::StringKw
                | JasmSyntaxKind::SourceFileKw
                | JasmSyntaxKind::StackKw
                | JasmSyntaxKind::LocalsKw
                | JasmSyntaxKind::EndKw
                | JasmSyntaxKind::CompiledKw
                | JasmSyntaxKind::FromKw
                | JasmSyntaxKind::InnerClassKw
                | JasmSyntaxKind::NestMembersKw
                | JasmSyntaxKind::BootstrapMethodKw
        )
    }
}
