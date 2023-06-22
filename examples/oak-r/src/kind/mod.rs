use crate::language::RLanguage;
use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RSyntaxKind {
    // 空白符和换行
    Whitespace,
    Newline,

    // 注释
    Comment,

    // 字面量
    StringLiteral,
    IntegerLiteral,
    FloatLiteral,
    BooleanLiteral,
    NullLiteral,
    Inf,
    NaN,
    NA,
    NaInteger,
    NaReal,
    NaComplex,
    NaCharacter,

    // 标识符
    Identifier,

    // 关键字
    If,
    Else,
    For,
    In,
    While,
    Repeat,
    Next,
    Break,
    Function,
    Return,
    True,
    False,
    Null,

    // 运算符
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Not,
    AndAnd,
    OrOr,
    Tilde,
    LeftArrow,
    RightArrow,
    DoubleLeftArrow,
    DoubleRightArrow,
    Pipe,
    Operator,

    // 分隔符
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    Colon,
    DoubleColon,
    TripleColon,
    Dot,
    Dollar,
    At,
    Question,

    // 根节点
    Root,

    // 表达式
    BinaryExpression,
    UnaryExpression,
    LiteralExpression,
    IdentifierExpression,
    CallExpression,
    GroupingExpression,
    BlockExpression,
    IfExpression,
    WhileExpression,
    ForExpression,
    RepeatExpression,
    FunctionDefinition,
    IndexExpression,
    MemberExpression,
    ArgumentList,
    ParameterList,

    // 错误和结束
    Error,
    Eof,
}

pub type RNode<'a> = oak_core::tree::RedNode<'a, RLanguage>;

impl ElementType for RSyntaxKind {
    type Role = UniversalElementRole;
    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::BinaryExpression | Self::UnaryExpression | Self::LiteralExpression | Self::IdentifierExpression | Self::CallExpression | Self::GroupingExpression | Self::IndexExpression | Self::MemberExpression => UniversalElementRole::Expression,
            Self::BlockExpression | Self::IfExpression | Self::WhileExpression | Self::ForExpression | Self::RepeatExpression => UniversalElementRole::Container,
            Self::FunctionDefinition => UniversalElementRole::Definition,
            Self::ArgumentList | Self::ParameterList => UniversalElementRole::Detail,
            _ => UniversalElementRole::None,
        }
    }
}

impl TokenType for RSyntaxKind {
    const END_OF_STREAM: Self = RSyntaxKind::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::StringLiteral | Self::IntegerLiteral | Self::FloatLiteral | Self::BooleanLiteral | Self::NullLiteral => UniversalTokenRole::Literal,
            Self::Identifier => UniversalTokenRole::Name,
            Self::If | Self::Else | Self::For | Self::In | Self::While | Self::Repeat | Self::Next | Self::Break | Self::Function | Self::Return | Self::True | Self::False | Self::Null | Self::Inf | Self::NaN | Self::NA => UniversalTokenRole::Keyword,
            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::Caret
            | Self::Equal
            | Self::EqualEqual
            | Self::NotEqual
            | Self::Less
            | Self::Greater
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::And
            | Self::Or
            | Self::Not
            | Self::AndAnd
            | Self::OrOr
            | Self::Tilde
            | Self::LeftArrow
            | Self::RightArrow
            | Self::DoubleLeftArrow
            | Self::DoubleRightArrow
            | Self::Pipe => UniversalTokenRole::Operator,
            Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace | Self::Comma | Self::Semicolon | Self::Colon | Self::DoubleColon | Self::TripleColon | Self::Dot | Self::Dollar | Self::At => {
                UniversalTokenRole::Punctuation
            }
            _ => UniversalTokenRole::None,
        }
    }
}
