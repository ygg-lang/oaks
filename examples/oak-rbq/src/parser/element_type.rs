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
        unsafe { std::mem::transmute(token) }
    }
}
