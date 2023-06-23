use oak_core::{ElementType, UniversalElementRole};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Rust element types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RustElementType {
    /// `as`
    As,
    /// `break`
    Break,
    /// `const`
    Const,
    /// `continue`
    Continue,
    /// `crate`
    Crate,
    /// `else`
    Else,
    /// `enum`
    Enum,
    /// `extern`
    Extern,
    /// `false`
    False,
    /// `fn`
    Fn,
    /// `for`
    For,
    /// `if`
    If,
    /// `impl`
    Impl,
    /// `in`
    In,
    /// `let`
    Let,
    /// `loop`
    Loop,
    /// `match`
    Match,
    /// `mod`
    Mod,
    /// `move`
    Move,
    /// `mut`
    Mut,
    /// `pub`
    Pub,
    /// `ref`
    Ref,
    /// `return`
    Return,
    /// `self`
    SelfLower,
    /// `Self`
    SelfUpper,
    /// `static`
    Static,
    /// `struct`
    Struct,
    /// `super`
    Super,
    /// `trait`
    Trait,
    /// `true`
    True,
    /// `type`
    Type,
    /// `unsafe`
    Unsafe,
    /// `use`
    Use,
    /// `where`
    Where,
    /// `while`
    While,
    /// `abstract`
    Abstract,
    /// `become`
    Become,
    /// `box`
    Box,
    /// `do`
    Do,
    /// `final`
    Final,
    /// `macro`
    Macro,
    /// `override`
    Override,
    /// `priv`
    Priv,
    /// `typeof`
    Typeof,
    /// `unsized`
    Unsized,
    /// `virtual`
    Virtual,
    /// `yield`
    Yield,
    /// `async`
    Async,
    /// `await`
    Await,
    /// `dyn`
    Dyn,
    /// `try`
    Try,
    /// `union`
    Union,
    /// `raw`
    Raw,
    /// Integer literal
    IntegerLiteral,
    /// Float literal
    FloatLiteral,
    /// String literal
    StringLiteral,
    /// Char literal
    CharLiteral,
    /// Byte literal
    ByteLiteral,
    /// Byte string literal
    ByteStringLiteral,
    /// Raw string literal
    RawStringLiteral,
    /// Bool literal
    BoolLiteral,
    /// Identifier
    Identifier,
    /// Lifetime
    Lifetime,
    /// `(`
    LeftParen,
    /// `)`
    RightParen,
    /// `{`
    LeftBrace,
    /// `}`
    RightBrace,
    /// `[`
    LeftBracket,
    /// `]`
    RightBracket,
    /// `;`
    Semicolon,
    /// `,`
    Comma,
    /// `.`
    Dot,
    /// `..`
    DotDot,
    /// `...`
    DotDotDot,
    /// `..=`
    DotDotEq,
    /// `:`
    Colon,
    /// `::`
    DoubleColon,
    /// Path separator
    PathSep,
    /// `?`
    Question,
    /// `@`
    At,
    /// `#`
    Hash,
    /// `$`
    Dollar,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `%`
    Percent,
    /// `^`
    Caret,
    /// `&`
    Ampersand,
    /// `|`
    Pipe,
    /// `~`
    Tilde,
    /// `!`
    Bang,
    /// `=`
    Eq,
    /// `<`
    Lt,
    /// `>`
    Gt,
    /// `<`
    LessThan,
    /// `>`
    GreaterThan,
    /// `==`
    EqEq,
    /// `!=`
    Ne,
    /// `<=`
    Le,
    /// `>=`
    Ge,
    /// `<=`
    LessEq,
    /// `>=`
    GreaterEq,
    /// `&&`
    AndAnd,
    /// `||`
    OrOr,
    /// `<<`
    LeftShift,
    /// `>>`
    RightShift,
    /// `<<`
    Shl,
    /// `>>`
    Shr,
    /// `+=`
    PlusEq,
    /// `-=`
    MinusEq,
    /// `*=`
    StarEq,
    /// `/=`
    SlashEq,
    /// `%=`
    PercentEq,
    /// `^=`
    CaretEq,
    /// `&=`
    AndEq,
    /// `|=`
    OrEq,
    /// `<<=`
    ShlEq,
    /// `>>=`
    ShrEq,
    /// `<<=`
    LeftShiftEq,
    /// `>>=`
    RightShiftEq,
    /// `=`
    Assign,
    /// `+=`
    PlusAssign,
    /// `-=`
    MinusAssign,
    /// `*=`
    StarAssign,
    /// `/=`
    SlashAssign,
    /// `%=`
    PercentAssign,
    /// `&=`
    AmpAssign,
    /// `|=`
    PipeAssign,
    /// `^=`
    CaretAssign,
    /// `<<=`
    ShlAssign,
    /// `>>=`
    ShrAssign,
    /// `->`
    Arrow,
    /// `=>`
    FatArrow,
    /// Space
    Space,
    /// Newline
    Newline,
    /// Whitespace
    Whitespace,
    /// Line comment
    LineComment,
    /// Block comment
    BlockComment,
    /// Doc comment
    DocComment,
    /// `++`
    PlusPlus,
    /// `--`
    MinusMinus,
    /// End of stream
    Eof,
    /// Error
    Error,

    // Expressions
    /// Identifier expression
    IdentifierExpression,
    /// Literal expression
    LiteralExpression,
    /// Parenthesized expression
    ParenthesizedExpression,
    /// Unary expression
    UnaryExpression,
    /// Binary expression
    BinaryExpression,
    /// Call expression
    CallExpression,
    /// Index expression
    IndexExpression,
    /// Field expression
    FieldExpression,

    // Items and Statements
    /// Source file
    SourceFile,
    /// Function
    Function,
    /// Return type
    ReturnType,
    /// Use item
    UseItem,
    /// Module item
    ModuleItem,
    /// Struct item
    StructItem,
    /// Enum item
    EnumItem,
    /// Let statement
    LetStatement,
    /// If expression
    IfExpression,
    /// While expression
    WhileExpression,
    /// Loop expression
    LoopExpression,
    /// For expression
    ForExpression,
    /// Return statement
    ReturnStatement,
    /// Block
    Block,
    /// Parameter list
    ParameterList,
    /// Block expression
    BlockExpression,

    // Add missing variants used in builder
    /// Parameter
    Parameter,
    /// Expression statement
    ExpressionStatement,
    /// Item statement
    ItemStatement,
    /// Pattern
    Pattern,
    /// Expression
    Expression,
    /// Argument list
    ArgumentList,
    /// Type alias
    TypeAlias,
    /// Member expression
    MemberExpression,
}

impl ElementType for RustElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::RustTokenType> for RustElementType {
    fn from(token: crate::lexer::token_type::RustTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
