use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RbqElementType {
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
    Block,
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

impl RbqElementType {
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
            Self::Pipe => 8,
            _ => 0,
        }
    }
}

impl ElementType for RbqElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,

            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::RbqTokenType> for RbqElementType {
    fn from(token: crate::lexer::token_type::RbqTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
