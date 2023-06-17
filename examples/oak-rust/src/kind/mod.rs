use oak_core::SyntaxKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RustSyntaxKind {
    // 关键
    As,
    Async,
    Await,
    Break,
    Const,
    Continue,
    Crate,
    Dyn,
    Else,
    Enum,
    Extern,
    False,
    Fn,
    For,
    If,
    Impl,
    In,
    Let,
    Loop,
    Match,
    Mod,
    Move,
    Mut,
    Pub,
    Ref,
    Return,
    SelfValue,
    SelfType,
    Static,
    Struct,
    Super,
    Trait,
    True,
    Type,
    Unsafe,
    Use,
    Where,
    While,

    // 标识符和字面
    Identifier,
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,
    Lifetime,

    // 操作
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    Percent,    // %
    Caret,      // ^
    Not,        // !
    And,        // &
    Or,         // |
    AndAnd,     // &&
    OrOr,       // ||
    Shl,        // <<
    Shr,        // >>
    PlusEq,     // +=
    MinusEq,    // -=
    StarEq,     // *=
    SlashEq,    // /=
    PercentEq,  // %=
    CaretEq,    // ^=
    AndEq,      // &=
    OrEq,       // |=
    ShlEq,      // <<=
    ShrEq,      // >>=
    Eq,         // =
    EqEq,       // ==
    Ne,         // !=
    Gt,         // >
    Lt,         // <
    Ge,         // >=
    Le,         // <=
    At,         // @
    Underscore, // _
    Dot,        // .
    DotDot,     // ..
    DotDotDot,  // ...
    DotDotEq,   // ..=
    Comma,      // ,
    Semicolon,  // ;
    Colon,      // :
    PathSep,    // ::
    RArrow,     // ->
    FatArrow,   // =>
    Pound,      // #
    Dollar,     // $
    Question,   // ?

    // 分隔
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]

    // 空白和注
    Whitespace,
    Newline,
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

impl SyntaxKind for RustSyntaxKind {
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
        use RustSyntaxKind::*;
        !matches!(self, 
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
        use RustSyntaxKind::*;
        matches!(self, 
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
