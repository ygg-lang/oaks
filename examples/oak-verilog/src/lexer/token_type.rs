use oak_core::{TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VerilogKind {
    // Tokens
    Whitespace,
    Comment,
    String,
    Number,
    Identifier,

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
    InitialKw,
    InoutKw,
    ParameterKw,

    // Operators
    EqualEqual,
    NotEqual,
    LessEqual,
    GreaterEqual,
    LeftShift,
    RightShift,
    AndAnd,
    OrOr,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    Bang,
    Less,
    Greater,
    Ampersand,
    Pipe,
    Caret,
    Tilde,

    // Punctuation
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Semicolon,
    Comma,
    Dot,
    Colon,
    Hash,
    At,
    Question,

    // Elements
    Module,

    // Internal
    Error,
    Eof,
}

pub type VerilogTokenType = VerilogKind;

impl TokenType for VerilogKind {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::String | Self::Number => UniversalTokenRole::Literal,
            Self::Identifier => UniversalTokenRole::Name,
            Self::ModuleKw
            | Self::EndmoduleKw
            | Self::WireKw
            | Self::RegKw
            | Self::InputKw
            | Self::OutputKw
            | Self::AlwaysKw
            | Self::BeginKw
            | Self::EndKw
            | Self::IfKw
            | Self::ElseKw
            | Self::AssignKw
            | Self::PosedgeKw
            | Self::NegedgeKw
            | Self::CaseKw
            | Self::EndcaseKw
            | Self::DefaultKw
            | Self::InitialKw
            | Self::InoutKw
            | Self::ParameterKw => UniversalTokenRole::Keyword,
            Self::EqualEqual
            | Self::NotEqual
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::LeftShift
            | Self::RightShift
            | Self::AndAnd
            | Self::OrOr
            | Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::Equal
            | Self::Bang
            | Self::Less
            | Self::Greater
            | Self::Ampersand
            | Self::Pipe
            | Self::Caret
            | Self::Tilde => UniversalTokenRole::Operator,
            Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace | Self::Semicolon | Self::Comma | Self::Dot | Self::Colon | Self::Hash | Self::At | Self::Question => {
                UniversalTokenRole::Punctuation
            }
            Self::Error => UniversalTokenRole::Error,
            Self::Eof => UniversalTokenRole::None,
            _ => UniversalTokenRole::None,
        }
    }
}
