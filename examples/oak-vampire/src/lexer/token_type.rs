use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type VampireToken = Token<VampireTokenType>;

impl TokenType for VampireTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::LineComment | Self::BlockComment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalTokenRole::None,
            Self::Text => UniversalTokenRole::None,
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Newline => UniversalTokenRole::Whitespace,
            Self::Error => UniversalTokenRole::None,
            Self::Eof => UniversalTokenRole::None,

            // 标点符号
            Self::LeftParen
            | Self::RightParen
            | Self::LeftBracket
            | Self::RightBracket
            | Self::LeftBrace
            | Self::RightBrace
            | Self::Colon
            | Self::Semicolon
            | Self::Dot
            | Self::Comma
            | Self::Question
            | Self::Bang
            | Self::At
            | Self::Hash
            | Self::Dollar
            | Self::Percent
            | Self::Caret
            | Self::Ampersand
            | Self::Star
            | Self::Plus
            | Self::Minus
            | Self::Eq
            | Self::LessThan
            | Self::GreaterThan
            | Self::Slash
            | Self::Backslash
            | Self::Pipe
            | Self::Tilde => UniversalTokenRole::Punctuation,

            // 复合操作符
            Self::DoubleEq
            | Self::NotEq
            | Self::LessEq
            | Self::GreaterEq
            | Self::AndAnd
            | Self::OrOr
            | Self::PlusPlus
            | Self::MinusMinus
            | Self::PlusEq
            | Self::MinusEq
            | Self::StarEq
            | Self::SlashEq
            | Self::PercentEq
            | Self::LeftShift
            | Self::RightShift
            | Self::Arrow => UniversalTokenRole::Operator,

            // 关键字
            Self::FofKw
            | Self::CnfKw
            | Self::TffKw
            | Self::ThfKw
            | Self::TpiKw
            | Self::IncludeKw
            | Self::AxiomKw
            | Self::HypothesisKw
            | Self::DefinitionKw
            | Self::AssumptionKw
            | Self::LemmaKw
            | Self::TheoremKw
            | Self::ConjectureKw
            | Self::NegatedConjectureKw
            | Self::PlainKw
            | Self::TypeKw
            | Self::FiDomainKw
            | Self::FiFunctorsKw
            | Self::FiPredicatesKw
            | Self::UnknownKw
            | Self::ForallKw
            | Self::ExistsKw
            | Self::AndKw
            | Self::OrKw
            | Self::NotKw
            | Self::ImpliesKw
            | Self::IffKw
            | Self::XorKw
            | Self::NorKw
            | Self::NandKw
            | Self::BoolKw
            | Self::IntKw
            | Self::RealKw
            | Self::RatKw
            | Self::IndividualKw
            | Self::OTypeKw
            | Self::ITypeKw
            | Self::TTypeKw => UniversalTokenRole::Keyword,

            // 字面量
            Self::IntegerLiteral | Self::RealLiteral | Self::StringLiteral | Self::BoolLiteral => {
                        UniversalTokenRole::Literal
                    }
                    Self::Identifier => UniversalTokenRole::Name,

                    // 注释
            Self::LineComment | Self::BlockComment => UniversalTokenRole::Comment,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum VampireTokenType {
    // 根节点
    Root,

    // 基础文本
    Text,
    Whitespace,
    Newline,

    // 错误处理
    Error,

    // EOF
    Eof,

    // 标点符号
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Colon,
    Semicolon,
    Dot,
    Comma,
    Question,
    Bang,
    At,
    Hash,
    Dollar,
    Percent,
    Caret,
    Ampersand,
    Star,
    Plus,
    Minus,
    Eq,
    LessThan,
    GreaterThan,
    Slash,
    Backslash,
    Pipe,
    Tilde,

    // 复合操作符
    DoubleEq,
    NotEq,
    LessEq,
    GreaterEq,
    AndAnd,
    OrOr,
    PlusPlus,
    MinusMinus,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    LeftShift,
    RightShift,
    Arrow,

    // Vampire 关键字
    FofKw,
    CnfKw,
    TffKw,
    ThfKw,
    TpiKw,
    IncludeKw,
    AxiomKw,
    HypothesisKw,
    DefinitionKw,
    AssumptionKw,
    LemmaKw,
    TheoremKw,
    ConjectureKw,
    NegatedConjectureKw,
    PlainKw,
    TypeKw,
    FiDomainKw,
    FiFunctorsKw,
    FiPredicatesKw,
    UnknownKw,

    // 逻辑操作符
    ForallKw,
    ExistsKw,
    AndKw,
    OrKw,
    NotKw,
    ImpliesKw,
    IffKw,
    XorKw,
    NorKw,
    NandKw,

    // 基本类型
    BoolKw,
    IntKw,
    RealKw,
    RatKw,
    IndividualKw,
    OTypeKw,
    ITypeKw,
    TTypeKw,

    // 字面量
    IntegerLiteral,
    RealLiteral,
    StringLiteral,
    BoolLiteral,
    Identifier,

    // 注释
    LineComment,
    BlockComment,
}
