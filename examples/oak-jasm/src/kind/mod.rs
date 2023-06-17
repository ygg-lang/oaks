use crate::syntax::JasmSyntaxKind;
use core::ops::Range;

#[derive(Clone, Debug)]
pub struct JasmToken {
    pub kind: JasmSyntaxKind,
    pub span: Range<usize>,
    pub text: alloc::string::String,
}

impl JasmToken {
    pub fn new(kind: JasmSyntaxKind, span: Range<usize>, text: alloc::string::String) -> Self {
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
