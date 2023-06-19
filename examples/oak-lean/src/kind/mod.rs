use oak_core::{SyntaxKind, Token};
use serde::{Deserialize, Serialize};

pub type LeanToken = Token<LeanSyntaxKind>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LeanSyntaxKind {
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
    Comment,

    // 特殊标记
    Eof,
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

impl SyntaxKind for LeanSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace)
    }

    fn is_token_type(&self) -> bool {
        use LeanSyntaxKind::*;
        !matches!(
            self,
            Error | Eof |
            // 语法节点类型 (非终结符)
            SourceFile | Function | ParameterList | Parameter | BlockExpression |
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
        use LeanSyntaxKind::*;
        matches!(
            self,
            Error | Eof |
            // 语法节点类型 (非终结符)
            SourceFile | Function | ParameterList | Parameter | BlockExpression |
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
