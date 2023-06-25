use core::fmt;
use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};

/// Go language token type.
pub type GoToken = Token<GoTokenType>;

/// Token types for the Go language.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GoTokenType {
    // Non-terminal nodes
    /// A source file node.
    SourceFile,
    /// A package clause node.
    PackageClause,
    /// An import declaration node.
    ImportDeclaration,
    /// An import spec node.
    ImportSpec,
    /// A function declaration node.
    FunctionDeclaration,
    /// A parameter list node.
    ParameterList,
    /// A parameter declaration node.
    ParameterDecl,
    /// A code block node.
    Block,
    /// A variable declaration node.
    VariableDeclaration,
    /// A variable spec node.
    VariableSpec,
    /// A constant declaration node.
    ConstDeclaration,
    /// A constant spec node.
    ConstSpec,
    /// A type declaration node.
    TypeDeclaration,
    /// A type spec node.
    TypeSpec,
    /// A struct type node.
    StructType,
    /// A field declaration list node.
    FieldDeclList,
    /// A field declaration node.
    FieldDecl,
    /// An interface type node.
    InterfaceType,
    /// A method spec list node.
    MethodSpecList,
    /// A method spec node.
    MethodSpec,
    /// An expression list node.
    ExpressionList,
    /// An assignment statement node.
    AssignmentStatement,
    /// A short variable declaration node.
    ShortVarDecl,
    /// A return statement node.
    ReturnStatement,
    /// An if statement node.
    IfStatement,
    /// A for statement node.
    ForStatement,
    /// A switch statement node.
    SwitchStatement,
    /// An expression case clause node.
    ExprCaseClause,
    /// A type switch statement node.
    TypeSwitchStatement,
    /// A type case clause node.
    TypeCaseClause,
    /// A select statement node.
    SelectStatement,
    /// A communication clause node.
    CommClause,
    /// A go statement node.
    GoStatement,
    /// A defer statement node.
    DeferStatement,
    /// A function call expression node.
    CallExpression,
    /// An index expression node.
    IndexExpression,
    /// A selector expression node.
    SelectorExpression,
    /// A slice expression node.
    SliceExpression,
    /// A type assertion expression node.
    TypeAssertion,
    /// A unary expression node.
    UnaryExpression,
    /// A binary expression node.
    BinaryExpression,
    /// A literal value node.
    LiteralValue,
    /// An element list node.
    ElementList,
    /// A keyed element node.
    KeyedElement,

    // Literals
    /// An integer literal.
    IntLiteral,
    /// A floating-point literal.
    FloatLiteral,
    /// A string literal.
    StringLiteral,
    /// A rune literal.
    RuneLiteral,
    /// A boolean literal.
    BoolLiteral,

    // Identifiers
    /// An identifier.
    Identifier,

    // Keywords
    /// `break` keyword.
    Break,
    /// `case` keyword.
    Case,
    /// `chan` keyword.
    Chan,
    /// `const` keyword.
    Const,
    /// `continue` keyword.
    Continue,
    /// `default` keyword.
    Default,
    /// `defer` keyword.
    Defer,
    /// `else` keyword.
    Else,
    /// `fallthrough` keyword.
    Fallthrough,
    /// `for` keyword.
    For,
    /// `func` keyword.
    Func,
    /// `go` keyword.
    Go,
    /// `goto` keyword.
    Goto,
    /// `if` keyword.
    If,
    /// `import` keyword.
    Import,
    /// `interface` keyword.
    Interface,
    /// `map` keyword.
    Map,
    /// `package` keyword.
    Package,
    /// `range` keyword.
    Range,
    /// `return` keyword.
    Return,
    /// `select` keyword.
    Select,
    /// `struct` keyword.
    Struct,
    /// `switch` keyword.
    Switch,
    /// `type` keyword.
    Type,
    /// `var` keyword.
    Var,

    // Built-in types
    /// `bool` type.
    Bool,
    /// `byte` type.
    Byte,
    /// `complex64` type.
    Complex64,
    /// `complex128` type.
    Complex128,
    /// `error` type.
    ErrorType,
    /// `float32` type.
    Float32,
    /// `float64` type.
    Float64,
    /// `int` type.
    Int,
    /// `int8` type.
    Int8,
    /// `int16` type.
    Int16,
    /// `int32` type.
    Int32,
    /// `int64` type.
    Int64,
    /// `rune` type.
    Rune,
    /// `string` type.
    String,
    /// `uint` type.
    Uint,
    /// `uint8` type.
    Uint8,
    /// `uint16` type.
    Uint16,
    /// `uint32` type.
    Uint32,
    /// `uint64` type.
    Uint64,
    /// `uintptr` type.
    Uintptr,

    // Special literals
    /// `nil` literal.
    NilLiteral,
    /// A number literal.
    NumberLiteral,
    /// A character literal.
    CharLiteral,

    // Operators
    /// `+`.
    Plus,
    /// `-`.
    Minus,
    /// `*`.
    Star,
    /// `/`.
    Slash,
    /// `%`.
    Percent,
    /// `&`.
    Ampersand,
    /// `|`.
    Pipe,
    /// `^`.
    Caret,
    /// `<<`.
    LeftShift,
    /// `>>`.
    RightShift,
    /// `&^`.
    AmpersandCaret,

    /// `+=`.
    PlusAssign,
    /// `-=`.
    MinusAssign,
    /// `*=`.
    StarAssign,
    /// `/=`.
    SlashAssign,
    /// `%=`.
    PercentAssign,
    /// `&=`.
    AmpersandAssign,
    /// `|=`.
    PipeAssign,
    /// `^=`.
    CaretAssign,
    /// `^=` (alias).
    XorAssign,
    /// `<<=`.
    LeftShiftAssign,
    /// `>>=`.
    RightShiftAssign,
    /// `&^=`.
    AmpersandCaretAssign,
    /// `&=` (alias).
    AndAssign,
    /// `|=` (alias).
    OrAssign,
    /// `&^=` (alias).
    AndNotAssign,
    /// `&^` (alias).
    AndNot,

    /// `&&`.
    LogicalAnd,
    /// `||`.
    LogicalOr,
    /// `&&` (alias).
    And,
    /// `||` (alias).
    Or,
    /// `<-`.
    Arrow,
    /// `<-` (alias).
    LeftArrow,
    /// `++`.
    Increment,
    /// `--`.
    Decrement,

    /// `==`.
    Equal,
    /// `<`.
    Less,
    /// `>`.
    Greater,
    /// `=`.
    Assign,
    /// `!`.
    LogicalNot,
    /// `!` (alias).
    Not,

    /// `!=`.
    NotEqual,
    /// `<=`.
    LessEqual,
    /// `>=`.
    GreaterEqual,
    /// `:=`.
    ColonAssign,
    /// `:=` (alias).
    Define,
    /// `...`.
    Ellipsis,

    // Delimiters
    /// `(`.
    LeftParen,
    /// `)`.
    RightParen,
    /// `[`.
    LeftBracket,
    /// `]`.
    RightBracket,
    /// `{`.
    LeftBrace,
    /// `}`.
    RightBrace,
    /// `,`.
    Comma,
    /// `.`.
    Period,
    /// `.` (alias).
    Dot,
    /// `;`.
    Semicolon,
    /// `:`.
    Colon,

    // Whitespace and comments
    /// Whitespace.
    Whitespace,
    /// Comment.
    Comment,

    // Special
    /// End of stream marker.
    Eof,
    /// Error element.
    Error,
}

impl GoTokenType {
    pub fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::Break
                | Self::Case
                | Self::Chan
                | Self::Const
                | Self::Continue
                | Self::Default
                | Self::Defer
                | Self::Else
                | Self::Fallthrough
                | Self::For
                | Self::Func
                | Self::Go
                | Self::Goto
                | Self::If
                | Self::Import
                | Self::Interface
                | Self::Map
                | Self::Package
                | Self::Range
                | Self::Return
                | Self::Select
                | Self::Struct
                | Self::Switch
                | Self::Type
                | Self::Var
        )
    }
}

impl fmt::Debug for GoTokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TokenType for GoTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Eof => UniversalTokenRole::Eof,
            Self::Identifier => UniversalTokenRole::Name,
            Self::StringLiteral | Self::IntLiteral | Self::FloatLiteral | Self::RuneLiteral | Self::BoolLiteral | Self::NilLiteral | Self::NumberLiteral | Self::CharLiteral => UniversalTokenRole::Literal,
            Self::Break
            | Self::Case
            | Self::Chan
            | Self::Const
            | Self::Continue
            | Self::Default
            | Self::Defer
            | Self::Else
            | Self::Fallthrough
            | Self::For
            | Self::Func
            | Self::Go
            | Self::Goto
            | Self::If
            | Self::Import
            | Self::Interface
            | Self::Map
            | Self::Package
            | Self::Range
            | Self::Return
            | Self::Select
            | Self::Struct
            | Self::Switch
            | Self::Type
            | Self::Var => UniversalTokenRole::Keyword,
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
