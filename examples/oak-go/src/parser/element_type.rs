use oak_core::{ElementType, UniversalElementRole};

/// Element types for the Go language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GoElementType {
    // Non-terminal nodes
    /// A source file.
    SourceFile,
    /// A package clause.
    PackageClause,
    /// An import declaration.
    ImportDeclaration,
    /// An import spec.
    ImportSpec,
    /// A function declaration.
    FunctionDeclaration,
    /// A receiver declaration.
    Receiver,
    /// A parameter list.
    ParameterList,
    /// A parameter declaration.
    ParameterDecl,
    /// A code block.
    Block,
    /// A variable declaration.
    VariableDeclaration,
    /// A variable spec.
    VariableSpec,
    /// A constant declaration.
    ConstDeclaration,
    /// A constant spec.
    ConstSpec,
    /// A type declaration.
    TypeDeclaration,
    /// A type spec.
    TypeSpec,
    /// A struct type.
    StructType,
    /// A field declaration list.
    FieldDeclList,
    /// A field declaration.
    FieldDecl,
    /// An interface type.
    InterfaceType,
    /// A method spec list.
    MethodSpecList,
    /// A method spec.
    MethodSpec,
    /// An expression list.
    ExpressionList,
    /// An assignment statement.
    AssignmentStatement,
    /// A short variable declaration.
    ShortVarDecl,
    /// A return statement.
    ReturnStatement,
    /// An if statement.
    IfStatement,
    /// A for statement.
    ForStatement,
    /// A switch statement.
    SwitchStatement,
    /// An expression case clause.
    ExprCaseClause,
    /// A type switch statement.
    TypeSwitchStatement,
    /// A type case clause.
    TypeCaseClause,
    /// A select statement.
    SelectStatement,
    /// A communication clause.
    CommClause,
    /// A go statement.
    GoStatement,
    /// A defer statement.
    DeferStatement,
    /// A function call expression.
    CallExpression,
    /// An index expression.
    IndexExpression,
    /// A selector expression.
    SelectorExpression,
    /// A slice expression.
    SliceExpression,
    /// A type assertion expression.
    TypeAssertion,
    /// A unary expression.
    UnaryExpression,
    /// A binary expression.
    BinaryExpression,
    /// A literal value.
    LiteralValue,
    /// An element list.
    ElementList,
    /// A keyed element.
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

impl GoElementType {
    /// Returns true if the element is whitespace or a comment.
    pub fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    /// Returns true if the element is a Go keyword.
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

    /// Returns true if the element is a Go literal.
    pub fn is_literal(&self) -> bool {
        matches!(self, Self::IntLiteral | Self::FloatLiteral | Self::StringLiteral | Self::RuneLiteral | Self::BoolLiteral | Self::NilLiteral | Self::NumberLiteral | Self::CharLiteral)
    }

    /// Returns true if the element is a Go operator.
    pub fn is_operator(&self) -> bool {
        matches!(
            self,
            Self::Plus
                | Self::Minus
                | Self::Star
                | Self::Slash
                | Self::Percent
                | Self::Ampersand
                | Self::Pipe
                | Self::Caret
                | Self::LeftShift
                | Self::RightShift
                | Self::AmpersandCaret
                | Self::PlusAssign
                | Self::MinusAssign
                | Self::StarAssign
                | Self::SlashAssign
                | Self::PercentAssign
                | Self::AmpersandAssign
                | Self::PipeAssign
                | Self::CaretAssign
                | Self::XorAssign
                | Self::LeftShiftAssign
                | Self::RightShiftAssign
                | Self::AmpersandCaretAssign
                | Self::AndAssign
                | Self::OrAssign
                | Self::AndNotAssign
                | Self::AndNot
                | Self::LogicalAnd
                | Self::LogicalOr
                | Self::And
                | Self::Or
                | Self::Arrow
                | Self::LeftArrow
                | Self::Increment
                | Self::Decrement
                | Self::Equal
                | Self::Less
                | Self::Greater
                | Self::Assign
                | Self::LogicalNot
                | Self::Not
                | Self::NotEqual
                | Self::LessEqual
                | Self::GreaterEqual
                | Self::ColonAssign
                | Self::Define
                | Self::Ellipsis
        )
    }
}

impl ElementType for GoElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::GoTokenType> for GoElementType {
    fn from(token: crate::lexer::token_type::GoTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
