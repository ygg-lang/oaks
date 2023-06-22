use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};

/// RBQ 语法种类（包含节点与词法）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum RbqSyntaxKind {
    // --- 节点种类 (Elements) ---
    Root,

    // 顶级定义
    NamespaceDef,
    ImportDef, // use auth.User
    StructDef,
    ClassDef,
    EnumDef,
    UnionDef,
    TraitDef,
    TypeDef, // type Email = string
    MicroDef,

    // 结构体成员
    FieldDef,
    UsingDef, // using Timestamp

    // 枚举成员
    EnumMember,
    UnionMember,

    // 类型引用
    TypeRef,
    GenericArgs,

    // 注解
    Annotation,
    AnnotationArgs,

    // DSL / 查询
    QueryPipeline,
    PipelineStep, // filter, sort, etc.
    Closure,
    ClosureArgs,
    Expression,
    Literal,
    MagicVar, // $
    BinaryExpr,
    UnaryExpr,
    CallExpr,
    MemberExpr,

    // 错误节点
    ErrorNode,

    // --- 词法种类 (Tokens) ---
    // 关键字
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

    // 逻辑值
    TrueKw,
    FalseKw,

    // 符号
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    LeftParen,    // (
    RightParen,   // )
    Colon,        // :
    Semicolon,    // ;
    Comma,        // ,
    Dot,          // .
    Question,     // ?
    At,           // @
    Hash,         // #
    Dollar,       // $
    Ampersand,    // &
    Arrow,        // ->

    // 运算符
    Eq,     // =
    EqEq,   // ==
    NotEq,  // !=
    Gt,     // >
    Lt,     // <
    GtEq,   // >=
    LtEq,   // <=
    AndAnd, // &&
    OrOr,   // ||
    Not,    // !
    Plus,   // +
    Minus,  // -
    Star,   // *
    Slash,  // /

    // 标识符与字面量
    Ident,
    StringLiteral,
    NumberLiteral,

    // 其它
    Whitespace,
    Newline,
    Comment,
    BlockComment,
    Eof,
    Error,
}

impl RbqSyntaxKind {
    // Aliases for LSP/Semantic Tokens
    pub const IDENTIFIER: Self = Self::Ident;
    pub const NAMESPACE_DEFINITION: Self = Self::NamespaceDef;
    pub const STRUCT_DEFINITION: Self = Self::StructDef;
    pub const ENUM_DEFINITION: Self = Self::EnumDef;
    pub const TYPE_REFERENCE: Self = Self::TypeRef;
    pub const ANNOTATION: Self = Self::Annotation;
    pub const FIELD_DEFINITION: Self = Self::FieldDef;
    pub const ENUM_VARIANT: Self = Self::EnumMember;
    pub const FLOAT_LITERAL: Self = Self::NumberLiteral;

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

impl TokenType for RbqSyntaxKind {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment | Self::BlockComment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn role(&self) -> Self::Role {
        use UniversalTokenRole::*;
        match self {
            Self::StructKw | Self::ClassKw | Self::EnumKw | Self::UnionKw | Self::TraitKw | Self::UsingKw | Self::NamespaceKw | Self::UseKw | Self::TypeKw | Self::MicroKw | Self::Utf8Kw | Self::TrueKw | Self::FalseKw => Keyword,

            Self::LeftBrace
            | Self::RightBrace
            | Self::LeftBracket
            | Self::RightBracket
            | Self::LeftParen
            | Self::RightParen
            | Self::Colon
            | Self::Semicolon
            | Self::Comma
            | Self::Dot
            | Self::Question
            | Self::At
            | Self::Hash
            | Self::Dollar
            | Self::Ampersand
            | Self::Arrow => Punctuation,

            Self::Eq | Self::EqEq | Self::NotEq | Self::Gt | Self::Lt | Self::GtEq | Self::LtEq | Self::AndAnd | Self::OrOr | Self::Not | Self::Plus | Self::Minus | Self::Star | Self::Slash => Operator,

            Self::StringLiteral | Self::NumberLiteral => Literal,

            Self::Ident => Name,
            Self::Whitespace | Self::Newline => Whitespace,
            Self::Comment | Self::BlockComment => Comment,
            Self::Error => Error,
            _ => None,
        }
    }
}

impl ElementType for RbqSyntaxKind {
    type Role = UniversalElementRole;

    fn is_root(&self) -> bool {
        matches!(self, Self::Root)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::ErrorNode)
    }

    fn role(&self) -> Self::Role {
        use UniversalElementRole::*;
        match self {
            Self::Root => Root,

            Self::NamespaceDef | Self::ImportDef | Self::StructDef | Self::ClassDef | Self::EnumDef | Self::UnionDef | Self::TraitDef | Self::TypeDef | Self::MicroDef => Definition,

            Self::FieldDef | Self::UsingDef | Self::EnumMember | Self::UnionMember => Definition,

            Self::QueryPipeline | Self::PipelineStep => Container,

            Self::Expression | Self::BinaryExpr | Self::UnaryExpr | Self::CallExpr | Self::MemberExpr | Self::Literal | Self::MagicVar => Value,

            Self::ErrorNode => Error,
            _ => None,
        }
    }
}
