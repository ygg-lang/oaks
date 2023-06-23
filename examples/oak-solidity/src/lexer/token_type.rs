use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type SolidityToken = Token<SolidityTokenType>;

impl TokenType for SolidityTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::LineComment | Self::BlockComment => UniversalTokenRole::Comment,
            Self::Contract
            | Self::Interface
            | Self::Library
            | Self::Function
            | Self::Modifier
            | Self::Event
            | Self::Struct
            | Self::Enum
            | Self::Mapping
            | Self::Public
            | Self::Private
            | Self::Internal
            | Self::External
            | Self::Pure
            | Self::View
            | Self::Payable
            | Self::Constant
            | Self::Bool
            | Self::String
            | Self::Bytes
            | Self::Address
            | Self::Uint
            | Self::Int
            | Self::Fixed
            | Self::Ufixed
            | Self::If
            | Self::Else
            | Self::For
            | Self::While
            | Self::Do
            | Self::Break
            | Self::Continue
            | Self::Return
            | Self::Try
            | Self::Catch
            | Self::Import
            | Self::Pragma
            | Self::Using
            | Self::Is
            | Self::Override
            | Self::Virtual
            | Self::Abstract => UniversalTokenRole::Keyword,
            Self::NumberLiteral | Self::StringLiteral | Self::BooleanLiteral | Self::AddressLiteral | Self::HexLiteral => UniversalTokenRole::Literal,
            Self::Identifier => UniversalTokenRole::Name,
            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::Power
            | Self::Equal
            | Self::NotEqual
            | Self::Less
            | Self::Greater
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::And
            | Self::Or
            | Self::Not
            | Self::BitAnd
            | Self::BitOr
            | Self::BitXor
            | Self::BitNot
            | Self::LeftShift
            | Self::RightShift
            | Self::Assign
            | Self::PlusAssign
            | Self::MinusAssign
            | Self::StarAssign
            | Self::SlashAssign
            | Self::PercentAssign => UniversalTokenRole::Operator,
            Self::LeftParen | Self::RightParen | Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket | Self::Semicolon | Self::Comma | Self::Dot | Self::Arrow => UniversalTokenRole::Punctuation,
            Self::Error => UniversalTokenRole::Error,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::LineComment | Self::BlockComment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::LineComment | Self::BlockComment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }
}

impl SolidityTokenType {
    pub fn is_token_type(&self) -> bool {
        !matches!(self, Self::SourceFile)
    }

    pub fn is_element_type(&self) -> bool {
        matches!(self, Self::SourceFile)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SolidityTokenType {
    // 空白和换
    Whitespace,
    Newline,

    // 注释
    LineComment,
    BlockComment,

    // 关键
    Contract,
    Interface,
    Library,
    Function,
    Modifier,
    Event,
    Struct,
    Enum,
    Mapping,
    Array,

    // 可见性修饰符
    Public,
    Private,
    Internal,
    External,

    // 状态修饰符
    Pure,
    View,
    Payable,
    Constant,

    // 类型关键
    Bool,
    String,
    Bytes,
    Address,
    Uint,
    Int,
    Fixed,
    Ufixed,

    // 控制
    If,
    Else,
    For,
    While,
    Do,
    Break,
    Continue,
    Return,
    Try,
    Catch,

    // 其他关键
    Import,
    Pragma,
    Using,
    Is,
    Override,
    Virtual,
    Abstract,

    // 字面
    NumberLiteral,
    StringLiteral,
    BooleanLiteral,
    AddressLiteral,
    HexLiteral,

    // 标识
    Identifier,

    // 操作
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Power,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Not,
    BitAnd,
    BitOr,
    BitXor,
    BitNot,
    LeftShift,
    RightShift,
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,

    // 分隔
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    Dot,
    Arrow,

    // 结构
    SourceFile,

    // 特殊
    Error,
    Eof,
}
