use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

impl TokenType for LeanTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        false
    }

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalTokenRole::None,
        }
    }
}

pub type LeanToken = Token<LeanTokenType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LeanTokenType {
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
