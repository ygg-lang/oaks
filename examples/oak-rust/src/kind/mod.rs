use oak_core::{SyntaxKind, Token};
use serde::{Deserialize, Serialize};

/// Type alias for Token with RustSyntaxKind
pub type RustToken = Token<RustSyntaxKind>;

/// Represents all possible kind kinds in the Rust language.
///
/// This enum includes both terminal tokens (keywords, identifiers, literals, operators, etc.)
/// and non-terminal kind nodes (expressions, statements, declarations, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RustSyntaxKind {
    /// The `as` keyword for type casting
    As,
    /// The `async` keyword for async functions or blocks
    Async,
    /// The `await` keyword for awaiting futures
    Await,
    /// The `break` keyword for breaking out of loops
    Break,
    /// The `const` keyword for constant definitions
    Const,
    /// The `continue` keyword for continuing to the next loop iteration
    Continue,
    /// The `crate` keyword for referring to the crate root
    Crate,
    /// The `dyn` keyword for dynamic trait objects
    Dyn,
    /// The `else` keyword for else branches in conditionals
    Else,
    /// The `enum` keyword for enum definitions
    Enum,
    /// The `extern` keyword for external function/block declarations
    Extern,
    /// The `false` boolean literal
    False,
    /// The `fn` keyword for function definitions
    Fn,
    /// The `for` keyword for loops
    For,
    /// The `if` keyword for conditional expressions
    If,
    /// The `impl` keyword for implementation blocks
    Impl,
    /// The `in` keyword for loops and patterns
    In,
    /// The `let` keyword for variable bindings
    Let,
    /// The `loop` keyword for infinite loops
    Loop,
    /// The `match` keyword for pattern matching
    Match,
    /// The `mod` keyword for module definitions
    Mod,
    /// The `move` keyword for closures
    Move,
    /// The `mut` keyword for mutable bindings/references
    Mut,
    /// The `pub` keyword for visibility
    Pub,
    /// The `ref` keyword for pattern bindings
    Ref,
    /// The `return` keyword for returning from functions
    Return,
    /// The `self` keyword for the current value
    SelfValue,
    /// The `Self` keyword for the implementing type
    SelfType,
    /// The `static` keyword for static variables
    Static,
    /// The `struct` keyword for struct definitions
    Struct,
    /// The `super` keyword for parent modules
    Super,
    /// The `trait` keyword for trait definitions
    Trait,
    /// The `true` boolean literal
    True,
    /// The `type` keyword for type aliases
    Type,
    /// The `unsafe` keyword for unsafe blocks/functions
    Unsafe,
    /// The `use` keyword for importing items
    Use,
    /// The `where` keyword for where clauses
    Where,
    /// The `while` keyword for while loops
    While,
    /// An identifier (variable name, function name, etc.)
    Identifier,
    /// An integer literal (e.g., `42`, `0xFF`, `0o755`, `0b1010`)
    IntegerLiteral,
    /// A floating-point literal (e.g., `3.14`, `2.0e10`)
    FloatLiteral,
    /// A string literal (e.g., `"hello"`)
    StringLiteral,
    /// A character literal (e.g., `'a'`)
    CharLiteral,
    /// A lifetime annotation (e.g., `'a`)
    Lifetime,
    /// Addition operator: `+`
    Plus,
    /// Subtraction operator: `-`
    Minus,
    /// Multiplication operator: `*`
    Star,
    /// Division operator: `/`
    Slash,
    /// Modulo operator: `%`
    Percent,
    /// Bitwise XOR operator: `^`
    Caret,
    /// Logical NOT/bitwise NOT operator: `!`
    Not,
    /// Bitwise AND operator: `&`
    And,
    /// Bitwise OR operator: `|`
    Or,
    /// Logical AND operator: `&&`
    AndAnd,
    /// Logical OR operator: `||`
    OrOr,
    /// Left shift operator: `<<`
    Shl,
    /// Right shift operator: `>>`
    Shr,
    /// Addition assignment operator: `+=`
    PlusEq,
    /// Subtraction assignment operator: `-=`
    MinusEq,
    /// Multiplication assignment operator: `*=`
    StarEq,
    /// Division assignment operator: `/=`
    SlashEq,
    /// Modulo assignment operator: `%=`
    PercentEq,
    /// Bitwise XOR assignment operator: `^=`
    CaretEq,
    /// Bitwise AND assignment operator: `&=`
    AndEq,
    /// Bitwise OR assignment operator: `|=`
    OrEq,
    /// Left shift assignment operator: `<<=`
    ShlEq,
    /// Right shift assignment operator: `>>=`
    ShrEq,
    /// Assignment operator: `=`
    Eq,
    /// Equality comparison operator: `==`
    EqEq,
    /// Inequality comparison operator: `!=`
    Ne,
    /// Greater-than operator: `>`
    Gt,
    /// Less-than operator: `<`
    Lt,
    /// Greater-than-or-equal operator: `>=`
    Ge,
    /// Less-than-or-equal operator: `<=`
    Le,
    /// At symbol: `@`
    At,
    /// Underscore: `_`
    Underscore,
    /// Dot: `.`
    Dot,
    /// Range operator: `..`
    DotDot,
    /// Inclusive range operator: `...`
    DotDotDot,
    /// Range-to operator: `..=`
    DotDotEq,
    /// Comma: `,`
    Comma,
    /// Semicolon: `;`
    Semicolon,
    /// Colon: `:`
    Colon,
    /// Path separator: `::`
    PathSep,
    /// Right arrow: `->`
    RArrow,
    /// Fat arrow: `=>`
    FatArrow,
    /// Pound sign: `#`
    Pound,
    /// Dollar sign: `$`
    Dollar,
    /// Question mark: `?`
    Question,
    /// Left parenthesis: `(`
    LeftParen,
    /// Right parenthesis: `)`
    RightParen,
    /// Left brace: `{`
    LeftBrace,
    /// Right brace: `}`
    RightBrace,
    /// Left bracket: `[`
    LeftBracket,
    /// Right bracket: `]`
    RightBracket,
    /// Whitespace characters
    Whitespace,
    /// Comments (both line and block comments)
    Comment,
    /// The root of a source file
    SourceFile,
    /// A function definition
    Function,
    /// A list of parameters in a function signature
    ParameterList,
    /// A single parameter in a function signature
    Parameter,
    /// A block expression `{ ... }`
    BlockExpression,
    /// A let statement `let x = 5;`
    LetStatement,
    /// An expression statement
    ExpressionStatement,
    /// An identifier expression
    IdentifierExpression,
    /// A literal expression
    LiteralExpression,
    /// A boolean literal expression
    BooleanLiteral,
    /// A parenthesized expression `(expr)`
    ParenthesizedExpression,
    /// A binary expression `a + b`
    BinaryExpression,
    /// A unary expression `!x` or `-x`
    UnaryExpression,
    /// A function call expression `func(arg)`
    CallExpression,
    /// A field access expression `obj.field`
    FieldExpression,
    /// An index expression `arr[index]`
    IndexExpression,
    /// An if expression `if cond { ... } else { ... }`
    IfExpression,
    /// A match expression `match value { ... }`
    MatchExpression,
    /// A loop expression `loop { ... }`
    LoopExpression,
    /// A while expression `while cond { ... }`
    WhileExpression,
    /// A for expression `for pat in iter { ... }`
    ForExpression,
    /// A break expression `break value?`
    BreakExpression,
    /// A continue expression `continue`
    ContinueExpression,
    /// A return expression `return value?`
    ReturnExpression,
    /// A struct expression `Struct { field: value }`
    StructExpression,
    /// A tuple expression `(a, b, c)`
    TupleExpression,
    /// An array expression `[a, b, c]`
    ArrayExpression,
    /// A range expression `start..end`
    RangeExpression,
    /// A closure expression `|args| body`
    ClosureExpression,
    /// An async block expression `async { ... }`
    AsyncBlockExpression,
    /// An unsafe block expression `unsafe { ... }`
    UnsafeBlockExpression,
    /// A try expression `expr?`
    TryExpression,
    /// An await expression `expr.await`
    AwaitExpression,
    /// A macro call `macro!(args)`
    MacroCall,
    /// A path `module::item`
    Path,
    /// A segment in a path
    PathSegment,
    /// Generic arguments `<T, U>`
    GenericArgs,
    /// A type path
    TypePath,
    /// A tuple type `(T, U)`
    TupleType,
    /// An array type `[T; N]`
    ArrayType,
    /// A slice type `[T]`
    SliceType,
    /// A reference type `&T` or `&mut T`
    ReferenceType,
    /// A raw pointer type `*const T` or `*mut T`
    PointerType,
    /// A function type `fn(T) -> U`
    FunctionType,
    /// A trait object type `dyn Trait`
    TraitObjectType,
    /// An impl trait type `impl Trait`
    ImplTraitType,
    /// An inferred type `_`
    InferredType,
    /// The never type `!`
    NeverType,
    /// A pattern in match arms or function parameters
    Pattern,
    /// An identifier pattern
    IdentifierPattern,
    /// A wildcard pattern `_`
    WildcardPattern,
    /// A tuple pattern `(a, b)`
    TuplePattern,
    /// A struct pattern `Struct { field: pattern }`
    StructPattern,
    /// A tuple struct pattern `Tuple(a, b)`
    TupleStructPattern,
    /// A slice pattern `[a, b, ..]`
    SlicePattern,
    /// A reference pattern `&pattern`
    ReferencePattern,
    /// A literal pattern `42` or `"hello"`
    LiteralPattern,
    /// A range pattern `start..end`
    RangePattern,
    /// An or pattern `pat1 | pat2`
    OrPattern,
    /// A rest pattern `..`
    RestPattern,
    /// A struct declaration
    StructDeclaration,
    /// An enum declaration
    EnumDeclaration,
    /// A union declaration
    UnionDeclaration,
    /// A trait declaration
    TraitDeclaration,
    /// An impl declaration
    ImplDeclaration,
    /// A module declaration
    ModuleDeclaration,
    /// A use declaration
    UseDeclaration,
    /// A const declaration
    ConstDeclaration,
    /// A static declaration
    StaticDeclaration,
    /// A type alias declaration
    TypeAliasDeclaration,
    /// An extern block
    ExternBlock,
    /// An extern function declaration
    ExternFunction,
    /// An attribute `#[attr]` or `#![attr]`
    Attribute,
    /// A visibility specifier `pub`, `pub(crate)`, etc.
    Visibility,
    /// Generic parameters `<T: Trait>`
    GenericParams,
    /// A single generic parameter
    GenericParam,
    /// A type parameter
    TypeParam,
    /// A const parameter
    ConstParam,
    /// A lifetime parameter
    LifetimeParam,
    /// A where clause
    WhereClause,
    /// A single where predicate
    WherePredicate,
    /// A return type in a function signature
    ReturnType,
    /// A list of fields in a struct or enum variant
    FieldList,
    /// A single field in a struct or enum variant
    Field,
    /// An enum variant
    Variant,
    /// A list of enum variants
    VariantList,
    /// An associated item in a trait or impl
    AssociatedItem,
    /// An item in a trait
    TraitItem,
    /// An item in an impl
    ImplItem,
    /// End of file marker
    Eof,
    /// Represents a kind error
    Error,
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
        matches!(self, Error | Eof)
    }

    fn is_element_type(&self) -> bool {
        use RustSyntaxKind::*;
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
