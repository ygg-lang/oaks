use crate::lexer::MojoTokenType;
use oak_core::UniversalElementRole;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MojoElementType {
    // Tokens (mapped from MojoTokenType)
    Fn,
    Struct,
    Var,
    Let,
    If,
    Else,
    While,
    For,
    In,
    Return,
    Break,
    Continue,
    Import,
    From,
    True,
    False,
    None,
    Identifier,
    Integer,
    Float,
    String,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    Not,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Colon,
    Semicolon,
    Arrow,
    Whitespace,
    Newline,
    Comment,
    Indent,
    Dedent,
    EndOfStream,

    // Statements
    FunctionDef,
    StructDef,
    VariableDecl,
    Assignment,
    IfStatement,
    WhileStatement,
    ForStatement,
    ReturnStatement,
    ExpressionStatement,

    // Expressions
    BinaryExpr,
    UnaryExpr,
    CallExpr,
    LiteralExpr,
    IdentifierExpr,
    MemberExpr,
    ListExpr,

    // Components
    ParamList,
    ArgList,
    Block,

    // Special
    Root,
    Grouping,
    Error,
}

impl MojoElementType {
    /// 是否为琐碎节点（空白、注释等）
    pub fn is_trivia(&self) -> bool {
        matches!(self, MojoElementType::Whitespace | MojoElementType::Newline | MojoElementType::Comment)
    }
}

impl From<MojoTokenType> for MojoElementType {
    fn from(token: MojoTokenType) -> Self {
        match token {
            MojoTokenType::Fn => MojoElementType::Fn,
            MojoTokenType::Struct => MojoElementType::Struct,
            MojoTokenType::Var => MojoElementType::Var,
            MojoTokenType::Let => MojoElementType::Let,
            MojoTokenType::If => MojoElementType::If,
            MojoTokenType::Else => MojoElementType::Else,
            MojoTokenType::While => MojoElementType::While,
            MojoTokenType::For => MojoElementType::For,
            MojoTokenType::In => MojoElementType::In,
            MojoTokenType::Return => MojoElementType::Return,
            MojoTokenType::Break => MojoElementType::Break,
            MojoTokenType::Continue => MojoElementType::Continue,
            MojoTokenType::Import => MojoElementType::Import,
            MojoTokenType::From => MojoElementType::From,
            MojoTokenType::True => MojoElementType::True,
            MojoTokenType::False => MojoElementType::False,
            MojoTokenType::None => MojoElementType::None,
            MojoTokenType::Identifier => MojoElementType::Identifier,
            MojoTokenType::Integer => MojoElementType::Integer,
            MojoTokenType::Float => MojoElementType::Float,
            MojoTokenType::String => MojoElementType::String,
            MojoTokenType::Plus => MojoElementType::Plus,
            MojoTokenType::Minus => MojoElementType::Minus,
            MojoTokenType::Star => MojoElementType::Star,
            MojoTokenType::Slash => MojoElementType::Slash,
            MojoTokenType::Percent => MojoElementType::Percent,
            MojoTokenType::Equal => MojoElementType::Equal,
            MojoTokenType::EqualEqual => MojoElementType::EqualEqual,
            MojoTokenType::NotEqual => MojoElementType::NotEqual,
            MojoTokenType::Less => MojoElementType::Less,
            MojoTokenType::LessEqual => MojoElementType::LessEqual,
            MojoTokenType::Greater => MojoElementType::Greater,
            MojoTokenType::GreaterEqual => MojoElementType::GreaterEqual,
            MojoTokenType::And => MojoElementType::And,
            MojoTokenType::Or => MojoElementType::Or,
            MojoTokenType::Not => MojoElementType::Not,
            MojoTokenType::LeftParen => MojoElementType::LeftParen,
            MojoTokenType::RightParen => MojoElementType::RightParen,
            MojoTokenType::LeftBracket => MojoElementType::LeftBracket,
            MojoTokenType::RightBracket => MojoElementType::RightBracket,
            MojoTokenType::LeftBrace => MojoElementType::LeftBrace,
            MojoTokenType::RightBrace => MojoElementType::RightBrace,
            MojoTokenType::Comma => MojoElementType::Comma,
            MojoTokenType::Dot => MojoElementType::Dot,
            MojoTokenType::Colon => MojoElementType::Colon,
            MojoTokenType::Semicolon => MojoElementType::Semicolon,
            MojoTokenType::Arrow => MojoElementType::Arrow,
            MojoTokenType::Whitespace => MojoElementType::Whitespace,
            MojoTokenType::Newline => MojoElementType::Newline,
            MojoTokenType::Comment => MojoElementType::Comment,
            MojoTokenType::Indent => MojoElementType::Indent,
            MojoTokenType::Dedent => MojoElementType::Dedent,
            MojoTokenType::EndOfStream => MojoElementType::EndOfStream,
            MojoTokenType::Error => MojoElementType::Error,
        }
    }
}

impl oak_core::ElementType for MojoElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        UniversalElementRole::None
    }
}
