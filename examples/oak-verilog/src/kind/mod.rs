use oak_core::{SyntaxKind, Token};
use serde::{Deserialize, Serialize};

pub type VerilogToken = Token<VerilogKind>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VerilogKind {
    // Literals
    Number,
    String,

    // Keywords
    ModuleKw,
    EndmoduleKw,
    WireKw,
    RegKw,
    InputKw,
    OutputKw,
    AlwaysKw,
    BeginKw,
    EndKw,
    IfKw,
    ElseKw,
    AssignKw,
    PosedgeKw,
    NegedgeKw,
    CaseKw,
    EndcaseKw,
    DefaultKw,

    // Identifiers
    Identifier,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    NotEqual,
    Bang,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    LeftShift,
    RightShift,
    Ampersand,
    AndAnd,
    Pipe,
    OrOr,
    Caret,
    Tilde,

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
    Colon,
    Hash,
    At,

    // Comments and whitespace
    LineComment,
    BlockComment,
    Whitespace,
    Newline,
    Comment,

    // Special
    Error,
    Eof,
}

impl SyntaxKind for VerilogKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::LineComment | Self::BlockComment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, VerilogKind::Comment)
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
