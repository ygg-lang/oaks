use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type VocToken = Token<VocTokenType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum VocTokenType {
    Root,
    // 基础
    Identifier,
    Keyword(oak_valkyrie::ValkyrieKeywords),
    StringLiteral,
    CharLiteral,
    IntegerLiteral,
    FloatLiteral,
    BoolLiteral,
    Number,

    // 关键字
    BoolKw,
    I8Kw,
    I16Kw,
    I32Kw,
    I64Kw,
    U8Kw,
    U16Kw,
    U32Kw,
    U64Kw,
    IntKw,
    UintKw,
    F32Kw,
    F64Kw,
    StringKw,
    CharKw,

    // 符号
    TagOpen,      // <
    TagClose,     // >
    TagSelfClose, // />
    TagSlash,     // /
    AttrEq,       // =

    Plus,         // +
    Minus,        // -
    Star,         // *
    Slash,        // /
    Percent,      // %
    Ampersand,    // &
    Pipe,         // |
    Caret,        // ^
    Bang,         // !
    Eq,           // =
    LessThan,     // <
    GreaterThan,  // >
    Dot,          // .
    Comma,        // ,
    Colon,        // :
    Semicolon,    // ;
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }
    Question,     // ?
    Tilde,        // ~

    // 复合符号
    PlusEq,       // +=
    MinusEq,      // -=
    StarEq,       // *=
    SlashEq,      // /=
    PercentEq,    // %=
    AmpersandEq,  // &=
    PipeEq,       // |=
    CaretEq,      // ^=
    EqEq,         // ==
    Ne,           // !=
    Le,           // <=
    Ge,           // >=
    PlusPlus,     // ++
    MinusMinus,   // --
    AndAnd,       // &&
    OrOr,         // ||
    LeftShift,    // <<
    LeftShiftEq,  // <<=
    RightShift,   // >>
    RightShiftEq, // >>=
    DotDot,       // ..
    DotDotDot,    // ...
    Arrow,        // ->
    FatArrow,     // =>

    // 模板
    InterpolationStart,   // {{
    InterpolationEnd,     // }}
    TemplateControlStart, // {%
    TemplateControlEnd,   // %}
    TextPart,

    // 杂项
    Whitespace,
    Newline,
    Comment,
    Error,
    Eof,
}

impl TokenType for VocTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
