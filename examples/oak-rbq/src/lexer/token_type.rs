use oak_core::{Token, TokenType, UniversalTokenRole};

/// RBQ token.
pub type RbqToken = Token<RbqTokenType>;

/// RBQ token type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RbqTokenType {
    /// Root node.
    Root,
    /// Namespace definition.
    NamespaceDef,
    /// Import definition.
    ImportDef,
    /// Struct definition.
    StructDef,
    /// Class definition.
    ClassDef,
    /// Enum definition.
    EnumDef,
    /// Union definition.
    UnionDef,
    /// Trait definition.
    TraitDef,
    /// Type definition.
    TypeDef,
    /// Micro definition.
    MicroDef,
    /// Field definition.
    FieldDef,
    /// Using definition.
    UsingDef,
    /// Enum member.
    EnumMember,
    /// Union member.
    UnionMember,
    /// Type reference.
    TypeRef,
    /// Generic arguments.
    GenericArgs,
    /// Annotation.
    Annotation,
    /// Annotation arguments.
    AnnotationArgs,
    /// Query pipeline.
    QueryPipeline,
    /// Pipeline step.
    PipelineStep,
    /// Closure.
    Closure,
    /// Closure arguments.
    ClosureArgs,
    /// Block.
    Block,
    /// Expression.
    Expression,
    /// Literal.
    Literal,
    /// Magic variable.
    MagicVar,
    /// Binary expression.
    BinaryExpr,
    /// Unary expression.
    UnaryExpr,
    /// Call expression.
    CallExpr,
    /// Member access expression.
    MemberExpr,
    /// Error node.
    ErrorNode,
    /// `struct` keyword.
    StructKw,
    /// `class` keyword.
    ClassKw,
    /// `enum` keyword.
    EnumKw,
    /// `union` keyword.
    UnionKw,
    /// `trait` keyword.
    TraitKw,
    /// `using` keyword.
    UsingKw,
    /// `namespace` keyword.
    NamespaceKw,
    /// `use` keyword.
    UseKw,
    /// `type` keyword.
    TypeKw,
    /// `micro` keyword.
    MicroKw,
    /// `utf8` keyword.
    Utf8Kw,
    /// `true` keyword.
    TrueKw,
    /// `false` keyword.
    FalseKw,
    /// Left brace `{`.
    LeftBrace,
    /// Right brace `}`.
    RightBrace,
    /// Left bracket `[`.
    LeftBracket,
    /// Right bracket `]`.
    RightBracket,
    /// Left parenthesis `(`.
    LeftParen,
    /// Right parenthesis `)`.
    RightParen,
    /// Colon `:`.
    Colon,
    /// Semicolon `;`.
    Semicolon,
    /// Comma `,`.
    Comma,
    /// Dot `.`.
    Dot,
    /// Question mark `?`.
    Question,
    /// At symbol `@`.
    At,
    /// Hash symbol `#`.
    Hash,
    /// Dollar symbol `$`.
    Dollar,
    /// Ampersand `&`.
    Ampersand,
    /// Arrow `->`.
    Arrow,
    /// Equal `=`.
    Eq,
    /// Double equal `==`.
    EqEq,
    /// Not equal `!=`.
    NotEq,
    /// Greater than `>`.
    Gt,
    /// Less than `<`.
    Lt,
    /// Greater than or equal to `>=`.
    GtEq,
    /// Less than or equal to `<=`.
    LtEq,
    /// Double ampersand `&&`.
    AndAnd,
    /// Double pipe `||`.
    OrOr,
    /// Bang `!`.
    Not,
    /// Plus `+`.
    Plus,
    /// Minus `-`.
    Minus,
    /// Star `*`.
    Star,
    /// Slash `/`.
    Slash,
    /// Pipe `|`.
    Pipe,
    /// Identifier.
    Ident,
    /// String literal.
    StringLiteral,
    /// Number literal.
    NumberLiteral,
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// Comment.
    Comment,
    /// Block comment.
    BlockComment,
    /// End of file.
    Eof,
    /// Error.
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
            Self::EqEq | Self::NotEq => 3,
            Self::Lt | Self::Gt | Self::LtEq | Self::GtEq => 4,
            Self::Plus | Self::Minus => 5,
            Self::Star | Self::Slash => 6,
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
