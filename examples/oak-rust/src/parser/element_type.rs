use oak_core::{ElementType, UniversalElementRole};

/// Rust element types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    /// Struct body
    StructBody,
    /// Enum body
    EnumBody,
    /// Trait body
    TraitBody,
    /// Impl body
    ImplBody,
    /// Module body
    ModuleBody,
    /// Variant
    Variant,
    /// Trait item
    TraitItem,
    /// Impl item
    ImplItem,
    /// Trait reference
    TraitRef,
    /// Tuple body
    TupleBody,
    /// Reference type
    ReferenceType,
    /// Tuple type
    TupleType,
    /// Array type
    ArrayType,
    /// Tuple pattern
    TuplePattern,
    /// Use path
    UsePath,
    /// Field
    Field,
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
                match token {
            crate::lexer::token_type::RustTokenType::As => Self::As,
            crate::lexer::token_type::RustTokenType::Break => Self::Break,
            crate::lexer::token_type::RustTokenType::Const => Self::Const,
            crate::lexer::token_type::RustTokenType::Continue => Self::Continue,
            crate::lexer::token_type::RustTokenType::Crate => Self::Crate,
            crate::lexer::token_type::RustTokenType::Else => Self::Else,
            crate::lexer::token_type::RustTokenType::Enum => Self::Enum,
            crate::lexer::token_type::RustTokenType::Extern => Self::Extern,
            crate::lexer::token_type::RustTokenType::False => Self::False,
            crate::lexer::token_type::RustTokenType::Fn => Self::Fn,
            crate::lexer::token_type::RustTokenType::For => Self::For,
            crate::lexer::token_type::RustTokenType::If => Self::If,
            crate::lexer::token_type::RustTokenType::Impl => Self::Impl,
            crate::lexer::token_type::RustTokenType::In => Self::In,
            crate::lexer::token_type::RustTokenType::Let => Self::Let,
            crate::lexer::token_type::RustTokenType::Loop => Self::Loop,
            crate::lexer::token_type::RustTokenType::Match => Self::Match,
            crate::lexer::token_type::RustTokenType::Mod => Self::Mod,
            crate::lexer::token_type::RustTokenType::Move => Self::Move,
            crate::lexer::token_type::RustTokenType::Mut => Self::Mut,
            crate::lexer::token_type::RustTokenType::Pub => Self::Pub,
            crate::lexer::token_type::RustTokenType::Ref => Self::Ref,
            crate::lexer::token_type::RustTokenType::Return => Self::Return,
            crate::lexer::token_type::RustTokenType::SelfLower => Self::SelfLower,
            crate::lexer::token_type::RustTokenType::SelfUpper => Self::SelfUpper,
            crate::lexer::token_type::RustTokenType::Static => Self::Static,
            crate::lexer::token_type::RustTokenType::Struct => Self::Struct,
            crate::lexer::token_type::RustTokenType::Super => Self::Super,
            crate::lexer::token_type::RustTokenType::Trait => Self::Trait,
            crate::lexer::token_type::RustTokenType::True => Self::True,
            crate::lexer::token_type::RustTokenType::Type => Self::Type,
            crate::lexer::token_type::RustTokenType::Unsafe => Self::Unsafe,
            crate::lexer::token_type::RustTokenType::Use => Self::Use,
            crate::lexer::token_type::RustTokenType::Where => Self::Where,
            crate::lexer::token_type::RustTokenType::While => Self::While,
            crate::lexer::token_type::RustTokenType::Abstract => Self::Abstract,
            crate::lexer::token_type::RustTokenType::Become => Self::Become,
            crate::lexer::token_type::RustTokenType::Box => Self::Box,
            crate::lexer::token_type::RustTokenType::Do => Self::Do,
            crate::lexer::token_type::RustTokenType::Final => Self::Final,
            crate::lexer::token_type::RustTokenType::Macro => Self::Macro,
            crate::lexer::token_type::RustTokenType::Override => Self::Override,
            crate::lexer::token_type::RustTokenType::Priv => Self::Priv,
            crate::lexer::token_type::RustTokenType::Typeof => Self::Typeof,
            crate::lexer::token_type::RustTokenType::Unsized => Self::Unsized,
            crate::lexer::token_type::RustTokenType::Virtual => Self::Virtual,
            crate::lexer::token_type::RustTokenType::Yield => Self::Yield,
            crate::lexer::token_type::RustTokenType::Async => Self::Async,
            crate::lexer::token_type::RustTokenType::Await => Self::Await,
            crate::lexer::token_type::RustTokenType::Dyn => Self::Dyn,
            crate::lexer::token_type::RustTokenType::Try => Self::Try,
            crate::lexer::token_type::RustTokenType::Union => Self::Union,
            crate::lexer::token_type::RustTokenType::Raw => Self::Raw,
            crate::lexer::token_type::RustTokenType::IntegerLiteral => Self::IntegerLiteral,
            crate::lexer::token_type::RustTokenType::FloatLiteral => Self::FloatLiteral,
            crate::lexer::token_type::RustTokenType::StringLiteral => Self::StringLiteral,
            crate::lexer::token_type::RustTokenType::CharLiteral => Self::CharLiteral,
            crate::lexer::token_type::RustTokenType::ByteLiteral => Self::ByteLiteral,
            crate::lexer::token_type::RustTokenType::ByteStringLiteral => Self::ByteStringLiteral,
            crate::lexer::token_type::RustTokenType::RawStringLiteral => Self::RawStringLiteral,
            crate::lexer::token_type::RustTokenType::BoolLiteral => Self::BoolLiteral,
            crate::lexer::token_type::RustTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::RustTokenType::Lifetime => Self::Lifetime,
            crate::lexer::token_type::RustTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::RustTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::RustTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::RustTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::RustTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::RustTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::RustTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::RustTokenType::Comma => Self::Comma,
            crate::lexer::token_type::RustTokenType::Dot => Self::Dot,
            crate::lexer::token_type::RustTokenType::DotDot => Self::DotDot,
            crate::lexer::token_type::RustTokenType::DotDotDot => Self::DotDotDot,
            crate::lexer::token_type::RustTokenType::DotDotEq => Self::DotDotEq,
            crate::lexer::token_type::RustTokenType::Colon => Self::Colon,
            crate::lexer::token_type::RustTokenType::DoubleColon => Self::DoubleColon,
            crate::lexer::token_type::RustTokenType::PathSep => Self::PathSep,
            crate::lexer::token_type::RustTokenType::Question => Self::Question,
            crate::lexer::token_type::RustTokenType::At => Self::At,
            crate::lexer::token_type::RustTokenType::Hash => Self::Hash,
            crate::lexer::token_type::RustTokenType::Dollar => Self::Dollar,
            crate::lexer::token_type::RustTokenType::Plus => Self::Plus,
            crate::lexer::token_type::RustTokenType::Minus => Self::Minus,
            crate::lexer::token_type::RustTokenType::Star => Self::Star,
            crate::lexer::token_type::RustTokenType::Slash => Self::Slash,
            crate::lexer::token_type::RustTokenType::Percent => Self::Percent,
            crate::lexer::token_type::RustTokenType::Caret => Self::Caret,
            crate::lexer::token_type::RustTokenType::Ampersand => Self::Ampersand,
            crate::lexer::token_type::RustTokenType::Pipe => Self::Pipe,
            crate::lexer::token_type::RustTokenType::Tilde => Self::Tilde,
            crate::lexer::token_type::RustTokenType::Bang => Self::Bang,
            crate::lexer::token_type::RustTokenType::Eq => Self::Eq,
            crate::lexer::token_type::RustTokenType::Lt => Self::Lt,
            crate::lexer::token_type::RustTokenType::Gt => Self::Gt,
            crate::lexer::token_type::RustTokenType::LessThan => Self::LessThan,
            crate::lexer::token_type::RustTokenType::GreaterThan => Self::GreaterThan,
            crate::lexer::token_type::RustTokenType::EqEq => Self::EqEq,
            crate::lexer::token_type::RustTokenType::Ne => Self::Ne,
            crate::lexer::token_type::RustTokenType::Le => Self::Le,
            crate::lexer::token_type::RustTokenType::Ge => Self::Ge,
            crate::lexer::token_type::RustTokenType::LessEq => Self::LessEq,
            crate::lexer::token_type::RustTokenType::GreaterEq => Self::GreaterEq,
            crate::lexer::token_type::RustTokenType::AndAnd => Self::AndAnd,
            crate::lexer::token_type::RustTokenType::OrOr => Self::OrOr,
            crate::lexer::token_type::RustTokenType::LeftShift => Self::LeftShift,
            crate::lexer::token_type::RustTokenType::RightShift => Self::RightShift,
            crate::lexer::token_type::RustTokenType::Shl => Self::Shl,
            crate::lexer::token_type::RustTokenType::Shr => Self::Shr,
            crate::lexer::token_type::RustTokenType::PlusEq => Self::PlusEq,
            crate::lexer::token_type::RustTokenType::MinusEq => Self::MinusEq,
            crate::lexer::token_type::RustTokenType::StarEq => Self::StarEq,
            crate::lexer::token_type::RustTokenType::SlashEq => Self::SlashEq,
            crate::lexer::token_type::RustTokenType::PercentEq => Self::PercentEq,
            crate::lexer::token_type::RustTokenType::CaretEq => Self::CaretEq,
            crate::lexer::token_type::RustTokenType::AndEq => Self::AndEq,
            crate::lexer::token_type::RustTokenType::OrEq => Self::OrEq,
            crate::lexer::token_type::RustTokenType::ShlEq => Self::ShlEq,
            crate::lexer::token_type::RustTokenType::ShrEq => Self::ShrEq,
            crate::lexer::token_type::RustTokenType::LeftShiftEq => Self::LeftShiftEq,
            crate::lexer::token_type::RustTokenType::RightShiftEq => Self::RightShiftEq,
            crate::lexer::token_type::RustTokenType::Assign => Self::Assign,
            crate::lexer::token_type::RustTokenType::PlusAssign => Self::PlusAssign,
            crate::lexer::token_type::RustTokenType::MinusAssign => Self::MinusAssign,
            crate::lexer::token_type::RustTokenType::StarAssign => Self::StarAssign,
            crate::lexer::token_type::RustTokenType::SlashAssign => Self::SlashAssign,
            crate::lexer::token_type::RustTokenType::PercentAssign => Self::PercentAssign,
            crate::lexer::token_type::RustTokenType::AmpAssign => Self::AmpAssign,
            crate::lexer::token_type::RustTokenType::PipeAssign => Self::PipeAssign,
            crate::lexer::token_type::RustTokenType::CaretAssign => Self::CaretAssign,
            crate::lexer::token_type::RustTokenType::ShlAssign => Self::ShlAssign,
            crate::lexer::token_type::RustTokenType::ShrAssign => Self::ShrAssign,
            crate::lexer::token_type::RustTokenType::Arrow => Self::Arrow,
            crate::lexer::token_type::RustTokenType::FatArrow => Self::FatArrow,
            crate::lexer::token_type::RustTokenType::Space => Self::Space,
            crate::lexer::token_type::RustTokenType::Newline => Self::Newline,
            crate::lexer::token_type::RustTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::RustTokenType::LineComment => Self::LineComment,
            crate::lexer::token_type::RustTokenType::BlockComment => Self::BlockComment,
            crate::lexer::token_type::RustTokenType::DocComment => Self::DocComment,
            crate::lexer::token_type::RustTokenType::PlusPlus => Self::PlusPlus,
            crate::lexer::token_type::RustTokenType::MinusMinus => Self::MinusMinus,
            crate::lexer::token_type::RustTokenType::Eof => Self::Eof,
            crate::lexer::token_type::RustTokenType::Error => Self::Error,
            _ => Self::Error,
        }
    }
}
