use oak_core::SyntaxKind;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum BashSyntaxKind {
    // Tokens
    Whitespace = 0,
    Newline,
    Comment,
    StringLiteral,
    Variable,
    NumberLiteral,
    Identifier,
    Keyword,
    Operator,
    Delimiter,
    Command,
    Path,
    Heredoc,
    GlobPattern,
    SpecialChar,
    Text,

    // Composite nodes
    SourceFile,
    Error,
    Eof,
}

impl SyntaxKind for BashSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        true
    }

    fn is_element_type(&self) -> bool {
        false
    }
}
