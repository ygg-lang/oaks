use oak_core::{ElementType, UniversalElementRole};

/// RBQ element type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RbqElementType {
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
                match token {
            crate::lexer::token_type::RbqTokenType::Root => Self::Root,
            crate::lexer::token_type::RbqTokenType::NamespaceDef => Self::NamespaceDef,
            crate::lexer::token_type::RbqTokenType::ImportDef => Self::ImportDef,
            crate::lexer::token_type::RbqTokenType::StructDef => Self::StructDef,
            crate::lexer::token_type::RbqTokenType::ClassDef => Self::ClassDef,
            crate::lexer::token_type::RbqTokenType::EnumDef => Self::EnumDef,
            crate::lexer::token_type::RbqTokenType::UnionDef => Self::UnionDef,
            crate::lexer::token_type::RbqTokenType::TraitDef => Self::TraitDef,
            crate::lexer::token_type::RbqTokenType::TypeDef => Self::TypeDef,
            crate::lexer::token_type::RbqTokenType::MicroDef => Self::MicroDef,
            crate::lexer::token_type::RbqTokenType::FieldDef => Self::FieldDef,
            crate::lexer::token_type::RbqTokenType::UsingDef => Self::UsingDef,
            crate::lexer::token_type::RbqTokenType::EnumMember => Self::EnumMember,
            crate::lexer::token_type::RbqTokenType::UnionMember => Self::UnionMember,
            crate::lexer::token_type::RbqTokenType::TypeRef => Self::TypeRef,
            crate::lexer::token_type::RbqTokenType::GenericArgs => Self::GenericArgs,
            crate::lexer::token_type::RbqTokenType::Annotation => Self::Annotation,
            crate::lexer::token_type::RbqTokenType::AnnotationArgs => Self::AnnotationArgs,
            crate::lexer::token_type::RbqTokenType::QueryPipeline => Self::QueryPipeline,
            crate::lexer::token_type::RbqTokenType::PipelineStep => Self::PipelineStep,
            crate::lexer::token_type::RbqTokenType::Closure => Self::Closure,
            crate::lexer::token_type::RbqTokenType::ClosureArgs => Self::ClosureArgs,
            crate::lexer::token_type::RbqTokenType::Expression => Self::Expression,
            crate::lexer::token_type::RbqTokenType::Literal => Self::Literal,
            crate::lexer::token_type::RbqTokenType::MagicVar => Self::MagicVar,
            crate::lexer::token_type::RbqTokenType::BinaryExpr => Self::BinaryExpr,
            crate::lexer::token_type::RbqTokenType::UnaryExpr => Self::UnaryExpr,
            crate::lexer::token_type::RbqTokenType::CallExpr => Self::CallExpr,
            crate::lexer::token_type::RbqTokenType::MemberExpr => Self::MemberExpr,
            crate::lexer::token_type::RbqTokenType::ErrorNode => Self::ErrorNode,
            crate::lexer::token_type::RbqTokenType::StructKw => Self::StructKw,
            crate::lexer::token_type::RbqTokenType::ClassKw => Self::ClassKw,
            crate::lexer::token_type::RbqTokenType::EnumKw => Self::EnumKw,
            crate::lexer::token_type::RbqTokenType::UnionKw => Self::UnionKw,
            crate::lexer::token_type::RbqTokenType::TraitKw => Self::TraitKw,
            crate::lexer::token_type::RbqTokenType::UsingKw => Self::UsingKw,
            crate::lexer::token_type::RbqTokenType::NamespaceKw => Self::NamespaceKw,
            crate::lexer::token_type::RbqTokenType::UseKw => Self::UseKw,
            crate::lexer::token_type::RbqTokenType::TypeKw => Self::TypeKw,
            crate::lexer::token_type::RbqTokenType::MicroKw => Self::MicroKw,
            crate::lexer::token_type::RbqTokenType::Utf8Kw => Self::Utf8Kw,
            crate::lexer::token_type::RbqTokenType::TrueKw => Self::TrueKw,
            crate::lexer::token_type::RbqTokenType::FalseKw => Self::FalseKw,
            crate::lexer::token_type::RbqTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::RbqTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::RbqTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::RbqTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::RbqTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::RbqTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::RbqTokenType::Colon => Self::Colon,
            crate::lexer::token_type::RbqTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::RbqTokenType::Comma => Self::Comma,
            crate::lexer::token_type::RbqTokenType::Dot => Self::Dot,
            crate::lexer::token_type::RbqTokenType::Question => Self::Question,
            crate::lexer::token_type::RbqTokenType::At => Self::At,
            crate::lexer::token_type::RbqTokenType::Hash => Self::Hash,
            crate::lexer::token_type::RbqTokenType::Dollar => Self::Dollar,
            crate::lexer::token_type::RbqTokenType::Ampersand => Self::Ampersand,
            crate::lexer::token_type::RbqTokenType::Arrow => Self::Arrow,
            crate::lexer::token_type::RbqTokenType::Eq => Self::Eq,
            crate::lexer::token_type::RbqTokenType::EqEq => Self::EqEq,
            crate::lexer::token_type::RbqTokenType::NotEq => Self::NotEq,
            crate::lexer::token_type::RbqTokenType::Gt => Self::Gt,
            crate::lexer::token_type::RbqTokenType::Lt => Self::Lt,
            crate::lexer::token_type::RbqTokenType::GtEq => Self::GtEq,
            crate::lexer::token_type::RbqTokenType::LtEq => Self::LtEq,
            crate::lexer::token_type::RbqTokenType::AndAnd => Self::AndAnd,
            crate::lexer::token_type::RbqTokenType::OrOr => Self::OrOr,
            crate::lexer::token_type::RbqTokenType::Not => Self::Not,
            crate::lexer::token_type::RbqTokenType::Plus => Self::Plus,
            crate::lexer::token_type::RbqTokenType::Minus => Self::Minus,
            crate::lexer::token_type::RbqTokenType::Star => Self::Star,
            crate::lexer::token_type::RbqTokenType::Slash => Self::Slash,
            crate::lexer::token_type::RbqTokenType::Ident => Self::Ident,
            crate::lexer::token_type::RbqTokenType::StringLiteral => Self::StringLiteral,
            crate::lexer::token_type::RbqTokenType::NumberLiteral => Self::NumberLiteral,
            crate::lexer::token_type::RbqTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::RbqTokenType::Newline => Self::Newline,
            crate::lexer::token_type::RbqTokenType::Comment => Self::Comment,
            crate::lexer::token_type::RbqTokenType::BlockComment => Self::BlockComment,
            crate::lexer::token_type::RbqTokenType::Eof => Self::Eof,
            crate::lexer::token_type::RbqTokenType::Error => Self::Error,
            _ => Self::Error,
        }
    }
}
