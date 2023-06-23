use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VueElementType {
    Root,
    Program,
    TemplateElement,
    Element,
    Tag,
    CloseTag,
    DocType,
    Attribute,
    AttributeName,
    AttributeValue,
    Directive,
    Modifier,
    Interpolation,
    TextNode,
    CommentNode,
    Identifier,

    // JS/Expression Elements
    Expression,
    Literal,
    BinaryExpr,
    UnaryExpr,
    CallExpr,
    MemberExpr,
    ArrayExpr,
    ObjectExpr,
    ObjectProperty,
    ArrowFunction,
    ConditionalExpr,
    TemplateLiteral,
    ForExpr,
    ForInExpr,
    ForOfExpr,
    Pattern,

    // Statements & Declarations
    ImportStmt,
    ImportSpecifier,
    ExportStmt,
    VariableDecl,
    VariableDeclarator,
    FunctionDecl,
    ExpressionStmt,
    ReturnStmt,
    IfStmt,
    WhileStmt,
    ForStmt,
    BlockStmt,
    BreakStmt,
    ContinueStmt,
    TseElement,
    TseAttribute,

    Whitespace,
    Newline,
    Comment,
    Error,
}

impl ElementType for VueElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root | Self::Program | Self::TemplateElement => UniversalElementRole::Root,
            Self::Element => UniversalElementRole::Container,
            Self::Tag | Self::CloseTag => UniversalElementRole::Name,
            Self::Attribute => UniversalElementRole::Attribute,
            Self::Interpolation => UniversalElementRole::Expression,
            Self::TextNode => UniversalElementRole::Value,
            Self::CommentNode | Self::Comment => UniversalElementRole::None,
            Self::Identifier => UniversalElementRole::Name,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::VueTokenType> for VueElementType {
    fn from(token: crate::lexer::token_type::VueTokenType) -> Self {
        use crate::lexer::token_type::VueTokenType as T;
        match token {
            T::Root => Self::Root,
            T::Program => Self::Program,
            T::TemplateElement => Self::TemplateElement,
            T::Element => Self::Element,
            T::Tag => Self::Tag,
            T::CloseTag => Self::CloseTag,
            T::DocType => Self::DocType,
            T::Attribute => Self::Attribute,
            T::AttributeName => Self::AttributeName,
            T::AttributeValue => Self::AttributeValue,
            T::Directive => Self::Directive,
            T::Modifier => Self::Modifier,
            T::Interpolation => Self::Interpolation,
            T::TextNode => Self::TextNode,
            T::CommentNode => Self::CommentNode,
            T::Identifier => Self::Identifier,
            T::Expression => Self::Expression,
            T::Literal => Self::Literal,
            T::BinaryExpr => Self::BinaryExpr,
            T::UnaryExpr => Self::UnaryExpr,
            T::CallExpr => Self::CallExpr,
            T::MemberExpr => Self::MemberExpr,
            T::ArrayExpr => Self::ArrayExpr,
            T::ObjectExpr => Self::ObjectExpr,
            T::ObjectProperty => Self::ObjectProperty,
            T::ArrowFunction => Self::ArrowFunction,
            T::ConditionalExpr => Self::ConditionalExpr,
            T::TemplateLiteral => Self::TemplateLiteral,
            T::ForExpr => Self::ForExpr,
            T::ForInExpr => Self::ForInExpr,
            T::ForOfExpr => Self::ForOfExpr,
            T::Pattern => Self::Pattern,
            T::ImportStmt => Self::ImportStmt,
            T::ImportSpecifier => Self::ImportSpecifier,
            T::ExportStmt => Self::ExportStmt,
            T::VariableDecl => Self::VariableDecl,
            T::VariableDeclarator => Self::VariableDeclarator,
            T::FunctionDecl => Self::FunctionDecl,
            T::ExpressionStmt => Self::ExpressionStmt,
            T::ReturnStmt => Self::ReturnStmt,
            T::IfStmt => Self::IfStmt,
            T::WhileStmt => Self::WhileStmt,
            T::ForStmt => Self::ForStmt,
            T::BlockStmt => Self::BlockStmt,
            T::BreakStmt => Self::BreakStmt,
            T::ContinueStmt => Self::ContinueStmt,
            T::TseElement => Self::TseElement,
            T::TseAttribute => Self::TseAttribute,
            T::Whitespace => Self::Whitespace,
            T::Comment => Self::Comment,
            T::Error => Self::Error,
            _ => Self::Error,
        }
    }
}
