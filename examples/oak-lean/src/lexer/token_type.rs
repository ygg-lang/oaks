use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};

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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Token types for the Lean language.
pub enum LeanTokenType {
    // Node kinds
    /// Root node.
    Root,
    /// End of stream.
    Eof,

    // Keywords
    /// `axiom` keyword.
    Axiom,
    /// `constant` keyword.
    Constant,
    /// `def` keyword.
    Def,
    /// `example` keyword.
    Example,
    /// `inductive` keyword.
    Inductive,
    /// `lemma` keyword.
    Lemma,
    /// `namespace` keyword.
    Namespace,
    /// `open` keyword.
    Open,
    /// `private` keyword.
    Private,
    /// `protected` keyword.
    Protected,
    /// `section` keyword.
    Section,
    /// `structure` keyword.
    Structure,
    /// `theorem` keyword.
    Theorem,
    /// `universe` keyword.
    Universe,
    /// `variable` keyword.
    Variable,
    /// `variables` keyword.
    Variables,
    /// `end` keyword.
    End,
    /// `import` keyword.
    Import,
    /// `export` keyword.
    Export,
    /// `prelude` keyword.
    Prelude,
    /// `noncomputable` keyword.
    Noncomputable,
    /// `partial` keyword.
    Partial,
    /// `unsafe` keyword.
    Unsafe,
    /// `mutual` keyword.
    Mutual,
    /// `where` keyword.
    Where,
    /// `have` keyword.
    Have,
    /// `show` keyword.
    Show,
    /// `suffices` keyword.
    Suffices,
    /// `let` keyword.
    Let,
    /// `in` keyword.
    In,
    /// `if` keyword.
    If,
    /// `then` keyword.
    Then,
    /// `else` keyword.
    Else,
    /// `match` keyword.
    Match,
    /// `with` keyword.
    With,
    /// `fun` keyword.
    Fun,
    /// `do` keyword.
    Do,
    /// `for` keyword.
    For,
    /// `while` keyword.
    While,
    /// `break` keyword.
    Break,
    /// `continue` keyword.
    Continue,
    /// `return` keyword.
    Return,
    /// `try` keyword.
    Try,
    /// `catch` keyword.
    Catch,
    /// `finally` keyword.
    Finally,
    /// `throw` keyword.
    Throw,

    // Identifiers and literals
    /// Identifier.
    Identifier,
    /// Integer literal.
    IntegerLiteral,
    /// Floating point literal.
    FloatLiteral,
    /// String literal.
    StringLiteral,
    /// Character literal.
    CharLiteral,

    // Operators
    /// Plus operator `+`.
    Plus,
    /// Minus operator `-`.
    Minus,
    /// Multiplication operator `*`.
    Star,
    /// Division operator `/`.
    Slash,
    /// Modulo operator `%`.
    Percent,
    /// Exponentiation operator `^`.
    Caret,
    /// Hash sign `#`.
    Hash,
    /// Bitwise AND operator `&`.
    Ampersand,
    /// Bitwise OR operator `|`.
    Pipe,
    /// Bitwise NOT operator `~`.
    Tilde,
    /// Logical NOT operator `!`.
    Bang,
    /// Question mark `?`.
    Question,
    /// At sign `@`.
    At,
    /// Dollar sign `$`.
    Dollar,
    /// Arrow `->`.
    Arrow,
    /// Fat arrow `=>`.
    FatArrow,
    /// Equality operator `=`.
    Eq,
    /// Inequality operator `!=`.
    Ne,
    /// Less than operator `<`.
    Lt,
    /// Less than or equal operator `<=`.
    Le,
    /// Greater than operator `>`.
    Gt,
    /// Greater than or equal operator `>=`.
    Ge,
    /// Logical AND operator `&&`.
    And,
    /// Logical OR operator `||`.
    Or,
    /// `not` operator.
    Not,
    /// Append operator `++`.
    Append,
    /// Cons operator `::`.
    Cons,

    // Delimiters
    /// Left parenthesis `(`.
    LeftParen,
    /// Right parenthesis `)`.
    RightParen,
    /// Left brace `{`.
    LeftBrace,
    /// Right brace `}`.
    RightBrace,
    /// Left bracket `[`.
    LeftBracket,
    /// Right bracket `]`.
    RightBracket,
    /// Left angle bracket `⟨`.
    LeftAngle,
    /// Right angle bracket `⟩`.
    RightAngle,
    /// Semicolon `;`.
    Semicolon,
    /// Colon `:`.
    Colon,
    /// Comma `,`.
    Comma,
    /// Dot `.`.
    Dot,
    /// Double dot `..`.
    DotDot,
    /// Assignment operator `:=`.
    ColonEq,
    /// Double colon `::`.
    ColonColon,

    // Whitespace and comments
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// Comment.
    Comment,

    // Special markers
    /// Error token.
    Error,

    // Syntax node types (Non-terminals)
    /// Source file.
    SourceFile,
    /// Function declaration.
    Function,
    /// Parameter list.
    ParameterList,
    /// Parameter.
    Parameter,
    /// Block expression.
    BlockExpression,
    /// Let statement.
    LetStatement,
    /// Expression statement.
    ExpressionStatement,
    /// Identifier expression.
    IdentifierExpression,
    /// Literal expression.
    LiteralExpression,
    /// Boolean literal.
    BooleanLiteral,
    /// Parenthesized expression.
    ParenthesizedExpression,
    /// Binary expression.
    BinaryExpression,
    /// Unary expression.
    UnaryExpression,
    /// Call expression.
    CallExpression,
    /// Field access expression.
    FieldExpression,
    /// Index access expression.
    IndexExpression,
    /// If expression.
    IfExpression,
    /// Match expression.
    MatchExpression,
    /// Loop expression.
    LoopExpression,
    /// While loop expression.
    WhileExpression,
    /// For loop expression.
    ForExpression,
    /// Break expression.
    BreakExpression,
    /// Continue expression.
    ContinueExpression,
    /// Return expression.
    ReturnExpression,
    /// Struct literal expression.
    StructExpression,
    /// Tuple literal expression.
    TupleExpression,
    /// Array literal expression.
    ArrayExpression,
    /// Range expression.
    RangeExpression,
    /// Closure expression.
    ClosureExpression,
    /// Async block expression.
    AsyncBlockExpression,
    /// Unsafe block expression.
    UnsafeBlockExpression,
    /// Try expression.
    TryExpression,
    /// Await expression.
    AwaitExpression,
    /// Macro call.
    MacroCall,
    /// Path.
    Path,
    /// Path segment.
    PathSegment,
    /// Generic arguments.
    GenericArgs,
    /// Type path.
    TypePath,
    /// Tuple type.
    TupleType,
    /// Array type.
    ArrayType,
    /// Slice type.
    SliceType,
    /// Reference type.
    ReferenceType,
    /// Pointer type.
    PointerType,
    /// Function type.
    FunctionType,
    /// Trait object type.
    TraitObjectType,
    /// Impl trait type.
    ImplTraitType,
    /// Inferred type `_`.
    InferredType,
    /// Never type `!`.
    NeverType,
    /// Pattern.
    Pattern,
    /// Identifier pattern.
    IdentifierPattern,
    /// Wildcard pattern `_`.
    WildcardPattern,
    /// Tuple pattern.
    TuplePattern,
    /// Struct pattern.
    StructPattern,
    /// Tuple struct pattern.
    TupleStructPattern,
    /// Slice pattern.
    SlicePattern,
    /// Reference pattern.
    ReferencePattern,
    /// Literal pattern.
    LiteralPattern,
    /// Range pattern.
    RangePattern,
    /// Or pattern.
    OrPattern,
    /// Rest pattern `..`.
    RestPattern,
    /// Struct declaration.
    StructDeclaration,
    /// Enum declaration.
    EnumDeclaration,
    /// Union declaration.
    UnionDeclaration,
    /// Trait declaration.
    TraitDeclaration,
    /// Impl block declaration.
    ImplDeclaration,
    /// Module declaration.
    ModuleDeclaration,
    /// Use declaration.
    UseDeclaration,
    /// Const declaration.
    ConstDeclaration,
    /// Static declaration.
    StaticDeclaration,
    /// Type alias declaration.
    TypeAliasDeclaration,
    /// Extern block.
    ExternBlock,
    /// Extern function.
    ExternFunction,
    /// Attribute.
    Attribute,
    /// Visibility modifier.
    Visibility,
    /// Generic parameters.
    GenericParams,
    /// Generic parameter.
    GenericParam,
    /// Type parameter.
    TypeParam,
    /// Const parameter.
    ConstParam,
    /// Lifetime parameter.
    LifetimeParam,
    /// Where clause.
    WhereClause,
    /// Where predicate.
    WherePredicate,
    /// Return type specification.
    ReturnType,
    /// Field list.
    FieldList,
    /// Field declaration.
    Field,
    /// Enum variant.
    Variant,
    /// Variant list.
    VariantList,
    /// Associated item.
    AssociatedItem,
    /// Trait item.
    TraitItem,
    /// Impl item.
    ImplItem,
}
