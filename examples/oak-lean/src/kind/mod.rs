use oak_core::{ElementType, Token, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

pub type LeanToken = Token<LeanSyntaxKind>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LeanSyntaxKind {
    // 节点种类
    Root,
    Eof,

    // 关键字
    Axiom,
    Constant,
    Def,
    Example,
    Inductive,
    Lemma,
    Namespace,
    Open,
    Private,
    Protected,
    Section,
    Structure,
    Theorem,
    Universe,
    Variable,
    Variables,
    End,
    Import,
    Export,
    Prelude,
    Noncomputable,
    Partial,
    Unsafe,
    Mutual,
    Where,
    Have,
    Show,
    Suffices,
    Let,
    In,
    If,
    Then,
    Else,
    Match,
    With,
    Fun,
    Do,
    For,
    While,
    Break,
    Continue,
    Return,
    Try,
    Catch,
    Finally,
    Throw,

    // 标识符和字面量
    Identifier,
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,

    // 操作符
    Plus,      // +
    Minus,     // -
    Star,      // *
    Slash,     // /
    Percent,   // %
    Caret,     // ^
    Hash,      // #
    Ampersand, // &
    Pipe,      // |
    Tilde,     // ~
    Bang,      // !
    Question,  // ?
    At,        // @
    Dollar,    // $
    Arrow,     // ->
    FatArrow,  // =>
    Eq,        // =
    Ne,        // !=
    Lt,        // <
    Le,        // <=
    Gt,        // >
    Ge,        // >=
    And,       // &&
    Or,        // ||
    Not,       // not
    Append,    // ++
    Cons,      // ::

    // 分隔符
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    LeftAngle,    // ⟨
    RightAngle,   // ⟩
    Semicolon,    // ;
    Colon,        // :
    Comma,        // ,
    Dot,          // .
    DotDot,       // ..
    ColonEq,      // :=
    ColonColon,   // ::

    // 空白和注释
    Whitespace,
    Newline,
    Comment,

    // 特殊标记
    Error,

    // 语法节点类型 (非终结符)
    SourceFile,
    Function,
    ParameterList,
    Parameter,
    BlockExpression,
    LetStatement,
    ExpressionStatement,
    IdentifierExpression,
    LiteralExpression,
    BooleanLiteral,
    ParenthesizedExpression,
    BinaryExpression,
    UnaryExpression,
    CallExpression,
    FieldExpression,
    IndexExpression,
    IfExpression,
    MatchExpression,
    LoopExpression,
    WhileExpression,
    ForExpression,
    BreakExpression,
    ContinueExpression,
    ReturnExpression,
    StructExpression,
    TupleExpression,
    ArrayExpression,
    RangeExpression,
    ClosureExpression,
    AsyncBlockExpression,
    UnsafeBlockExpression,
    TryExpression,
    AwaitExpression,
    MacroCall,
    Path,
    PathSegment,
    GenericArgs,
    TypePath,
    TupleType,
    ArrayType,
    SliceType,
    ReferenceType,
    PointerType,
    FunctionType,
    TraitObjectType,
    ImplTraitType,
    InferredType,
    NeverType,
    Pattern,
    IdentifierPattern,
    WildcardPattern,
    TuplePattern,
    StructPattern,
    TupleStructPattern,
    SlicePattern,
    ReferencePattern,
    LiteralPattern,
    RangePattern,
    OrPattern,
    RestPattern,
    StructDeclaration,
    EnumDeclaration,
    UnionDeclaration,
    TraitDeclaration,
    ImplDeclaration,
    ModuleDeclaration,
    UseDeclaration,
    ConstDeclaration,
    StaticDeclaration,
    TypeAliasDeclaration,
    ExternBlock,
    ExternFunction,
    Attribute,
    Visibility,
    GenericParams,
    GenericParam,
    TypeParam,
    ConstParam,
    LifetimeParam,
    WhereClause,
    WherePredicate,
    ReturnType,
    FieldList,
    Field,
    Variant,
    VariantList,
    AssociatedItem,
    TraitItem,
    ImplItem,
}

impl TokenType for LeanSyntaxKind {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Axiom
            | Self::Constant
            | Self::Def
            | Self::Example
            | Self::Inductive
            | Self::Lemma
            | Self::Namespace
            | Self::Open
            | Self::Private
            | Self::Protected
            | Self::Section
            | Self::Structure
            | Self::Theorem
            | Self::Universe
            | Self::Variable
            | Self::Variables
            | Self::End
            | Self::Import
            | Self::Export
            | Self::Prelude
            | Self::Noncomputable
            | Self::Partial
            | Self::Unsafe
            | Self::Mutual
            | Self::Where
            | Self::Have
            | Self::Show
            | Self::Suffices
            | Self::Let
            | Self::In
            | Self::If
            | Self::Then
            | Self::Else
            | Self::Match
            | Self::With
            | Self::Fun
            | Self::Do
            | Self::For
            | Self::While
            | Self::Break
            | Self::Continue
            | Self::Return
            | Self::Try
            | Self::Catch
            | Self::Finally
            | Self::Throw => UniversalTokenRole::Keyword,

            Self::Identifier => UniversalTokenRole::Name,

            Self::IntegerLiteral | Self::FloatLiteral | Self::StringLiteral | Self::CharLiteral | Self::BooleanLiteral => UniversalTokenRole::Literal,

            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::Caret
            | Self::Hash
            | Self::Ampersand
            | Self::Pipe
            | Self::Tilde
            | Self::Bang
            | Self::Question
            | Self::At
            | Self::Dollar
            | Self::Arrow
            | Self::FatArrow
            | Self::Eq
            | Self::Ne
            | Self::Lt
            | Self::Le
            | Self::Gt
            | Self::Ge
            | Self::And
            | Self::Or
            | Self::Not
            | Self::Append
            | Self::Cons => UniversalTokenRole::Operator,

            Self::LeftParen
            | Self::RightParen
            | Self::LeftBrace
            | Self::RightBrace
            | Self::LeftBracket
            | Self::RightBracket
            | Self::LeftAngle
            | Self::RightAngle
            | Self::Semicolon
            | Self::Colon
            | Self::Comma
            | Self::Dot
            | Self::DotDot
            | Self::ColonEq
            | Self::ColonColon => UniversalTokenRole::Punctuation,

            Self::Comment => UniversalTokenRole::Comment,
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for LeanSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root | Self::SourceFile => UniversalElementRole::Root,
            Self::Function | Self::StructDeclaration | Self::EnumDeclaration => UniversalElementRole::Definition,
            Self::BlockExpression | Self::ParameterList | Self::FieldList => UniversalElementRole::Container,
            Self::ExpressionStatement | Self::LetStatement => UniversalElementRole::Statement,
            Self::IdentifierExpression => UniversalElementRole::Reference,
            Self::LiteralExpression | Self::BooleanLiteral => UniversalElementRole::Value,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
