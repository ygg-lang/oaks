use oak_core::syntax::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LlvmKind {
    // Placeholder for LLVM IR tokens
    Identifier,
    Number,
    String,
    Comment,
    Whitespace,
    Newline,
    Error,
    Eof,
}

impl SyntaxKind for LlvmKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token(&self) -> bool {
        !matches!(self, Self::Error | Self::Eof)
    }
}