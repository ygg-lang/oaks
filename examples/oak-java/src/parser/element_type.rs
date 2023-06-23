use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum JavaElementType {
    Token(crate::lexer::token_type::JavaTokenType),
    Root,
    Identifier,
    LiteralExpression,
    ParenthesizedExpression,
    ArrayCreation,
    MethodCall,
    UnaryExpression,
    BinaryExpression,
    AssignmentExpression,
    CastExpression,
    PostfixExpression,
    TernaryExpression,
    MemberSelect,
    ArrayAccess,
    NewExpression,
    VariableDeclaration,
    ExpressionStatement,
    IfStatement,
    WhileStatement,
    DoWhileStatement,
    ForStatement,
    SwitchStatement,
    ReturnStatement,
    Break,
    Continue,
    Parameter,
    CatchClause,
    TryStatement,
    ThrowStatement,
    Package,
    Import,
    ClassDeclaration,
    InterfaceDeclaration,
    EnumDeclaration,
    StructDeclaration,
    RecordDeclaration,
    MethodDeclaration,
    FieldDeclaration,
    SwitchCase,
    DefaultCase,
    BlockStatement,
    CompilationUnit,
    Error,
}

impl ElementType for JavaElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        use UniversalElementRole::*;
        match self {
            Self::Token(token) => match token.role() {
                UniversalTokenRole::Keyword => None,
                UniversalTokenRole::Name => Name,
                UniversalTokenRole::Literal => Value,
                UniversalTokenRole::Operator => Expression,
                UniversalTokenRole::Punctuation => None,
                UniversalTokenRole::Comment => Documentation,
                UniversalTokenRole::Whitespace => None,
                UniversalTokenRole::Error => Error,
                UniversalTokenRole::None => None,
                UniversalTokenRole::Eof => None,
                UniversalTokenRole::Escape => Value,
            },
            Self::Root | Self::CompilationUnit => Root,
            Self::Identifier => Name,
            Self::LiteralExpression => Expression,
            Self::ParenthesizedExpression
            | Self::ArrayCreation
            | Self::MethodCall
            | Self::UnaryExpression
            | Self::BinaryExpression
            | Self::AssignmentExpression
            | Self::CastExpression
            | Self::PostfixExpression
            | Self::TernaryExpression
            | Self::MemberSelect
            | Self::ArrayAccess
            | Self::NewExpression => Expression,
            Self::VariableDeclaration
            | Self::ExpressionStatement
            | Self::IfStatement
            | Self::WhileStatement
            | Self::DoWhileStatement
            | Self::ForStatement
            | Self::SwitchStatement
            | Self::ReturnStatement
            | Self::Break
            | Self::Continue
            | Self::TryStatement
            | Self::ThrowStatement => Statement,
            Self::ClassDeclaration | Self::InterfaceDeclaration | Self::EnumDeclaration | Self::StructDeclaration | Self::RecordDeclaration | Self::MethodDeclaration | Self::FieldDeclaration => Definition,
            Self::Parameter | Self::CatchClause | Self::SwitchCase | Self::DefaultCase => Detail,
            Self::Package | Self::Import => Statement,
            Self::BlockStatement => Container,
            Self::Error => Error,
        }
    }
}

impl From<crate::lexer::token_type::JavaTokenType> for JavaElementType {
    fn from(token: crate::lexer::token_type::JavaTokenType) -> Self {
        Self::Token(token)
    }
}
