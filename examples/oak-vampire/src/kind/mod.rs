#![doc = include_str!("../../readme.md")]

use oak_core::{ElementType, Token, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

/// Type alias for Token with VampireSyntaxKind
pub type VampireToken = Token<VampireSyntaxKind>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VampireSyntaxKind {
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
    EqEq,
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

impl TokenType for VampireSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::LineComment | Self::BlockComment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for VampireSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
