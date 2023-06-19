#![doc = include_str!("../../readme.md")]

use oak_core::{SyntaxKind, Token};
use serde::{Deserialize, Serialize};

/// Type alias for Token with VampireSyntaxKind
pub type VampireToken = Token<VampireSyntaxKind>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VampireSyntaxKind {
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

impl SyntaxKind for VampireSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::LineComment | Self::BlockComment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::LineComment | Self::BlockComment)
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
