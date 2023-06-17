use oak_core::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerilogKind {
    // Literals
    Number,
    String,

    // Keywords
    Module,
    Endmodule,
    Wire,
    Reg,
    Input,
    Output,
    Always,
    Begin,
    End,
    If,
    Else,

    // Identifiers
    Identifier,

    // Operators
    Assign,
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    NotEqual,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    Dot,

    // Comments and whitespace
    Comment,
    Whitespace,
    Newline,

    // Special
    Error,
    Eof,
}

impl SyntaxKind for VerilogKind {
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
        !matches!(self, Self::Error | Self::Eof)
    }

    fn is_element_type(&self) -> bool {
        false
    }
}