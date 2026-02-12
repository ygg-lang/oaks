use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};

pub type TclToken = Token<TclTokenType>;

impl TokenType for TclTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        false
    }

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalTokenRole::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TclTokenType {
    // Node kinds (These are usually used in ElementType, but sometimes also as tokens)
    Root,
    Command,
    Word,
    SimpleWord,
    VariableWord,
    ScriptWord,
    BracedWord,

    // Literals
    Number,
    StringLiteral,
    Identifier,

    // Keywords
    If,
    Else,
    ElseIf,
    For,
    While,
    ForEach,
    Proc,
    Return,
    Break,
    Continue,
    Set,
    Unset,
    Global,
    Upvar,
    Variable,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Ampersand,
    AmpersandAmpersand,
    Pipe,
    PipePipe,
    Exclamation,

    // Punctuation
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Semicolon,
    Comma,
    Dollar,

    // Special
    Whitespace,
    Newline,
    Comment,
    Error,
    Eof,
}

impl From<TclTokenType> for UniversalElementRole {
    fn from(kind: TclTokenType) -> Self {
        match kind {
            TclTokenType::Root => UniversalElementRole::Root,
            TclTokenType::Command => UniversalElementRole::Expression,
            TclTokenType::Word | TclTokenType::SimpleWord | TclTokenType::VariableWord | TclTokenType::ScriptWord | TclTokenType::BracedWord => UniversalElementRole::Expression,
            TclTokenType::Identifier => UniversalElementRole::Name,
            TclTokenType::Number | TclTokenType::StringLiteral => UniversalElementRole::Value,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::parser::element_type::TclElementType> for TclTokenType {
    fn from(element: crate::parser::element_type::TclElementType) -> Self {
        unsafe { std::mem::transmute(element) }
    }
}
