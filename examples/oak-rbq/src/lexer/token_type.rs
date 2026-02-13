use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type RbqToken = Token<RbqTokenType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RbqTokenType {
    Root,
    NamespaceDef,
    ImportDef,
    StructDef,
    ClassDef,
    EnumDef,
    UnionDef,
    TraitDef,
    TypeDef,
    MicroDef,
    FieldDef,
    UsingDef,
    EnumMember,
    UnionMember,
    TypeRef,
    GenericArgs,
    Annotation,
    AnnotationArgs,
    QueryPipeline,
    PipelineStep,
    Closure,
    ClosureArgs,
    Block,
    Expression,
    Literal,
    MagicVar,
    BinaryExpr,
    UnaryExpr,
    CallExpr,
    MemberExpr,
    ErrorNode,
    StructKw,
    ClassKw,
    EnumKw,
    UnionKw,
    TraitKw,
    UsingKw,
    NamespaceKw,
    UseKw,
    TypeKw,
    MicroKw,
    Utf8Kw,
    TrueKw,
    FalseKw,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    Colon,
    Semicolon,
    Comma,
    Dot,
    Question,
    At,
    Hash,
    Dollar,
    Ampersand,
    Arrow,
    Eq,
    EqEq,
    NotEq,
    Gt,
    Lt,
    GtEq,
    LtEq,
    AndAnd,
    OrOr,
    Not,
    Plus,
    Minus,
    Star,
    Slash,
    Pipe,
    Ident,
    StringLiteral,
    NumberLiteral,
    Whitespace,
    Newline,
    Comment,
    BlockComment,
    Eof,
    Error,
}

impl RbqTokenType {
    /// Alias for identifier name.
    pub const IDENTIFIER: Self = Self::Ident;
    /// Alias for namespace definition.
    pub const NAMESPACE_DEFINITION: Self = Self::NamespaceDef;
    /// Alias for struct definition.
    pub const STRUCT_DEFINITION: Self = Self::StructDef;
    /// Alias for enum definition.
    pub const ENUM_DEFINITION: Self = Self::EnumDef;
    /// Alias for type reference.
    pub const TYPE_REFERENCE: Self = Self::TypeRef;
    /// Alias for annotation.
    pub const ANNOTATION: Self = Self::Annotation;
    /// Alias for field definition.
    pub const FIELD_DEFINITION: Self = Self::FieldDef;
    /// Alias for enum variant.
    pub const ENUM_VARIANT: Self = Self::EnumMember;
    /// Alias for float literal.
    pub const FLOAT_LITERAL: Self = Self::NumberLiteral;

    /// Returns the operator precedence for this kind.
    pub fn precedence(&self) -> u8 {
        match self {
            Self::OrOr => 1,
            Self::AndAnd => 2,
            Self::Eq => 3,
            Self::EqEq | Self::NotEq => 4,
            Self::Lt | Self::Gt | Self::LtEq | Self::GtEq => 5,
            Self::Plus | Self::Minus => 6,
            Self::Star | Self::Slash => 7,
            _ => 0,
        }
    }
}

impl From<crate::parser::element_type::RbqElementType> for RbqTokenType {
    fn from(kind: crate::parser::element_type::RbqElementType) -> Self {
        match kind {
            crate::parser::element_type::RbqElementType::Root => Self::Root,
            crate::parser::element_type::RbqElementType::NamespaceDef => Self::NamespaceDef,
            crate::parser::element_type::RbqElementType::ImportDef => Self::ImportDef,
            crate::parser::element_type::RbqElementType::StructDef => Self::StructDef,
            crate::parser::element_type::RbqElementType::ClassDef => Self::ClassDef,
            crate::parser::element_type::RbqElementType::EnumDef => Self::EnumDef,
            crate::parser::element_type::RbqElementType::UnionDef => Self::UnionDef,
            crate::parser::element_type::RbqElementType::TraitDef => Self::TraitDef,
            crate::parser::element_type::RbqElementType::TypeDef => Self::TypeDef,
            crate::parser::element_type::RbqElementType::MicroDef => Self::MicroDef,
            crate::parser::element_type::RbqElementType::FieldDef => Self::FieldDef,
            crate::parser::element_type::RbqElementType::UsingDef => Self::UsingDef,
            crate::parser::element_type::RbqElementType::EnumMember => Self::EnumMember,
            crate::parser::element_type::RbqElementType::UnionMember => Self::UnionMember,
            crate::parser::element_type::RbqElementType::TypeRef => Self::TypeRef,
            crate::parser::element_type::RbqElementType::GenericArgs => Self::GenericArgs,
            crate::parser::element_type::RbqElementType::Annotation => Self::Annotation,
            crate::parser::element_type::RbqElementType::AnnotationArgs => Self::AnnotationArgs,
            crate::parser::element_type::RbqElementType::QueryPipeline => Self::QueryPipeline,
            crate::parser::element_type::RbqElementType::PipelineStep => Self::PipelineStep,
            crate::parser::element_type::RbqElementType::Closure => Self::Closure,
            crate::parser::element_type::RbqElementType::ClosureArgs => Self::ClosureArgs,
            crate::parser::element_type::RbqElementType::Block => Self::Block,
            crate::parser::element_type::RbqElementType::Expression => Self::Expression,
            crate::parser::element_type::RbqElementType::Literal => Self::Literal,
            crate::parser::element_type::RbqElementType::MagicVar => Self::MagicVar,
            crate::parser::element_type::RbqElementType::BinaryExpr => Self::BinaryExpr,
            crate::parser::element_type::RbqElementType::UnaryExpr => Self::UnaryExpr,
            crate::parser::element_type::RbqElementType::CallExpr => Self::CallExpr,
            crate::parser::element_type::RbqElementType::MemberExpr => Self::MemberExpr,
            crate::parser::element_type::RbqElementType::ErrorNode => Self::ErrorNode,
            crate::parser::element_type::RbqElementType::StructKw => Self::StructKw,
            crate::parser::element_type::RbqElementType::ClassKw => Self::ClassKw,
            crate::parser::element_type::RbqElementType::EnumKw => Self::EnumKw,
            crate::parser::element_type::RbqElementType::UnionKw => Self::UnionKw,
            crate::parser::element_type::RbqElementType::TraitKw => Self::TraitKw,
            crate::parser::element_type::RbqElementType::UsingKw => Self::UsingKw,
            crate::parser::element_type::RbqElementType::NamespaceKw => Self::NamespaceKw,
            crate::parser::element_type::RbqElementType::UseKw => Self::UseKw,
            crate::parser::element_type::RbqElementType::TypeKw => Self::TypeKw,
            crate::parser::element_type::RbqElementType::MicroKw => Self::MicroKw,
            crate::parser::element_type::RbqElementType::Utf8Kw => Self::Utf8Kw,
            crate::parser::element_type::RbqElementType::TrueKw => Self::TrueKw,
            crate::parser::element_type::RbqElementType::FalseKw => Self::FalseKw,
            crate::parser::element_type::RbqElementType::LeftBrace => Self::LeftBrace,
            crate::parser::element_type::RbqElementType::RightBrace => Self::RightBrace,
            crate::parser::element_type::RbqElementType::LeftBracket => Self::LeftBracket,
            crate::parser::element_type::RbqElementType::RightBracket => Self::RightBracket,
            crate::parser::element_type::RbqElementType::LeftParen => Self::LeftParen,
            crate::parser::element_type::RbqElementType::RightParen => Self::RightParen,
            crate::parser::element_type::RbqElementType::Colon => Self::Colon,
            crate::parser::element_type::RbqElementType::Semicolon => Self::Semicolon,
            crate::parser::element_type::RbqElementType::Comma => Self::Comma,
            crate::parser::element_type::RbqElementType::Dot => Self::Dot,
            crate::parser::element_type::RbqElementType::Question => Self::Question,
            crate::parser::element_type::RbqElementType::At => Self::At,
            crate::parser::element_type::RbqElementType::Hash => Self::Hash,
            crate::parser::element_type::RbqElementType::Dollar => Self::Dollar,
            crate::parser::element_type::RbqElementType::Ampersand => Self::Ampersand,
            crate::parser::element_type::RbqElementType::Arrow => Self::Arrow,
            crate::parser::element_type::RbqElementType::Eq => Self::Eq,
            crate::parser::element_type::RbqElementType::EqEq => Self::EqEq,
            crate::parser::element_type::RbqElementType::NotEq => Self::NotEq,
            crate::parser::element_type::RbqElementType::Gt => Self::Gt,
            crate::parser::element_type::RbqElementType::Lt => Self::Lt,
            crate::parser::element_type::RbqElementType::GtEq => Self::GtEq,
            crate::parser::element_type::RbqElementType::LtEq => Self::LtEq,
            crate::parser::element_type::RbqElementType::AndAnd => Self::AndAnd,
            crate::parser::element_type::RbqElementType::OrOr => Self::OrOr,
            crate::parser::element_type::RbqElementType::Not => Self::Not,
            crate::parser::element_type::RbqElementType::Plus => Self::Plus,
            crate::parser::element_type::RbqElementType::Minus => Self::Minus,
            crate::parser::element_type::RbqElementType::Star => Self::Star,
            crate::parser::element_type::RbqElementType::Slash => Self::Slash,
            crate::parser::element_type::RbqElementType::Pipe => Self::Pipe,
            crate::parser::element_type::RbqElementType::Ident => Self::Ident,
            crate::parser::element_type::RbqElementType::StringLiteral => Self::StringLiteral,
            crate::parser::element_type::RbqElementType::NumberLiteral => Self::NumberLiteral,
            crate::parser::element_type::RbqElementType::Whitespace => Self::Whitespace,
            crate::parser::element_type::RbqElementType::Newline => Self::Newline,
            crate::parser::element_type::RbqElementType::Comment => Self::Comment,
            crate::parser::element_type::RbqElementType::BlockComment => Self::BlockComment,
            crate::parser::element_type::RbqElementType::Eof => Self::Eof,
            crate::parser::element_type::RbqElementType::Error => Self::Error,
        }
    }
}

impl TokenType for RbqTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment | Self::BlockComment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::BlockComment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
