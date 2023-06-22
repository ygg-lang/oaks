use oak_core::{ElementType, Token, TokenType, UniversalElementRole, UniversalTokenRole};
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

    // Nodes
    Module,

    // Special
    Error,
    Eof,
}

impl TokenType for VerilogKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            VerilogKind::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for VerilogKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
