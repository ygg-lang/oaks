use oak_core::{SyntaxKind, Token};
use serde::{Deserialize, Serialize};

pub type ElixirToken = Token<ElixirSyntaxKind>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElixirSyntaxKind {
    // 基本 kind
    Whitespace,
    Newline,
    Comment,

    // 标识符和字面量
    Identifier,
    Atom,
    Variable,
    Number,
    Float,
    String,
    Character,
    Sigil,

    // Elixir 关键字
    After,
    And,
    Case,
    Catch,
    Cond,
    Def,
    Defp,
    Defmodule,
    Defstruct,
    Defprotocol,
    Defimpl,
    Defmacro,
    Defmacrop,
    Do,
    Else,
    Elsif,
    End,
    False,
    Fn,
    If,
    In,
    Not,
    Or,
    Receive,
    Rescue,
    True,
    Try,
    Unless,
    When,
    With,

    // 操作符
    Plus,            // +
    Minus,           // -
    Star,            // *
    Slash,           // /
    Equal,           // =
    EqualEqual,      // ==
    NotEqual,        // !=
    EqualEqualEqual, // ===
    NotEqualEqual,   // !==
    Less,            // <
    Greater,         // >
    LessEqual,       // <=
    GreaterEqual,    // >=
    PlusPlus,        // ++
    MinusMinus,      // --
    StarStar,        // **
    Exclamation,     // !
    Question,        // ?
    Ampersand,       // &
    At,              // @
    Caret,           // ^
    Tilde,           // ~
    LeftShift,       // <<
    RightShift,      // >>
    MatchOp,         // =~
    PipeRight,       // |>

    // 分隔符
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Comma,        // ,
    Semicolon,    // ;
    Dot,          // .
    Colon,        // :
    Arrow,        // ->
    Pipe,         // |
    PipePipe,     // ||
    Hash,         // #

    // 特殊
    Error,
    Eof,

    // 语法节点类型 (非终结符)
    SourceFile,
    Module,
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

impl SyntaxKind for ElixirSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        use ElixirSyntaxKind::*;
        !matches!(
            self,
            Error | Eof |
            // 语法节点类型 (非终结符)
            SourceFile | Module | Function | ParameterList | Parameter | BlockExpression |
            LetStatement | ExpressionStatement | IdentifierExpression | LiteralExpression |
            BooleanLiteral | ParenthesizedExpression | BinaryExpression | UnaryExpression |
            CallExpression | FieldExpression | IndexExpression | IfExpression |
            MatchExpression | LoopExpression | WhileExpression | ForExpression |
            BreakExpression | ContinueExpression | ReturnExpression | StructExpression |
            TupleExpression | ArrayExpression | RangeExpression | ClosureExpression |
            AsyncBlockExpression | UnsafeBlockExpression | TryExpression | AwaitExpression |
            MacroCall | Path | PathSegment | GenericArgs | TypePath | TupleType |
            ArrayType | SliceType | ReferenceType | PointerType | FunctionType |
            TraitObjectType | ImplTraitType | InferredType | NeverType | Pattern |
            IdentifierPattern | WildcardPattern | TuplePattern | StructPattern |
            TupleStructPattern | SlicePattern | ReferencePattern | LiteralPattern |
            RangePattern | OrPattern | RestPattern | StructDeclaration | EnumDeclaration | UnionDeclaration |
             TraitDeclaration | ImplDeclaration | ModuleDeclaration | UseDeclaration | ConstDeclaration |
             StaticDeclaration | TypeAliasDeclaration | ExternBlock | ExternFunction |
            Attribute | Visibility | GenericParams | GenericParam | TypeParam |
            ConstParam | LifetimeParam | WhereClause | WherePredicate |
            ReturnType | FieldList | Field | Variant | VariantList |
            AssociatedItem | TraitItem | ImplItem
        )
    }

    fn is_element_type(&self) -> bool {
        use ElixirSyntaxKind::*;
        matches!(
            self,
            Error | Eof |
            // 语法节点类型 (非终结符)
            SourceFile | Module | Function | ParameterList | Parameter | BlockExpression |
            LetStatement | ExpressionStatement | IdentifierExpression | LiteralExpression |
            BooleanLiteral | ParenthesizedExpression | BinaryExpression | UnaryExpression |
            CallExpression | FieldExpression | IndexExpression | IfExpression |
            MatchExpression | LoopExpression | WhileExpression | ForExpression |
            BreakExpression | ContinueExpression | ReturnExpression | StructExpression |
            TupleExpression | ArrayExpression | RangeExpression | ClosureExpression |
            AsyncBlockExpression | UnsafeBlockExpression | TryExpression | AwaitExpression |
            MacroCall | Path | PathSegment | GenericArgs | TypePath | TupleType |
            ArrayType | SliceType | ReferenceType | PointerType | FunctionType |
            TraitObjectType | ImplTraitType | InferredType | NeverType | Pattern |
            IdentifierPattern | WildcardPattern | TuplePattern | StructPattern |
            TupleStructPattern | SlicePattern | ReferencePattern | LiteralPattern |
            RangePattern | OrPattern | RestPattern | StructDeclaration | EnumDeclaration | UnionDeclaration |
             TraitDeclaration | ImplDeclaration | ModuleDeclaration | UseDeclaration | ConstDeclaration |
             StaticDeclaration | TypeAliasDeclaration | ExternBlock | ExternFunction |
            Attribute | Visibility | GenericParams | GenericParam | TypeParam |
            ConstParam | LifetimeParam | WhereClause | WherePredicate |
            ReturnType | FieldList | Field | Variant | VariantList |
            AssociatedItem | TraitItem | ImplItem
        )
    }
}

impl ElixirSyntaxKind {
    pub fn is_keyword(self) -> bool {
        matches!(
            self,
            Self::After
                | Self::And
                | Self::Case
                | Self::Catch
                | Self::Cond
                | Self::Def
                | Self::Defp
                | Self::Defmodule
                | Self::Defstruct
                | Self::Defprotocol
                | Self::Defimpl
                | Self::Defmacro
                | Self::Defmacrop
                | Self::Do
                | Self::Else
                | Self::Elsif
                | Self::End
                | Self::False
                | Self::Fn
                | Self::If
                | Self::In
                | Self::Not
                | Self::Or
                | Self::Receive
                | Self::Rescue
                | Self::True
                | Self::Try
                | Self::Unless
                | Self::When
                | Self::With
        )
    }
}
