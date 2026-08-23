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
                match token {
            crate::lexer::token_type::GoTokenType::SourceFile => Self::SourceFile,
            crate::lexer::token_type::GoTokenType::PackageClause => Self::PackageClause,
            crate::lexer::token_type::GoTokenType::ImportDeclaration => Self::ImportDeclaration,
            crate::lexer::token_type::GoTokenType::ImportSpec => Self::ImportSpec,
            crate::lexer::token_type::GoTokenType::FunctionDeclaration => Self::FunctionDeclaration,
            crate::lexer::token_type::GoTokenType::ParameterList => Self::ParameterList,
            crate::lexer::token_type::GoTokenType::ParameterDecl => Self::ParameterDecl,
            crate::lexer::token_type::GoTokenType::Block => Self::Block,
            crate::lexer::token_type::GoTokenType::VariableDeclaration => Self::VariableDeclaration,
            crate::lexer::token_type::GoTokenType::VariableSpec => Self::VariableSpec,
            crate::lexer::token_type::GoTokenType::ConstDeclaration => Self::ConstDeclaration,
            crate::lexer::token_type::GoTokenType::ConstSpec => Self::ConstSpec,
            crate::lexer::token_type::GoTokenType::TypeDeclaration => Self::TypeDeclaration,
            crate::lexer::token_type::GoTokenType::TypeSpec => Self::TypeSpec,
            crate::lexer::token_type::GoTokenType::StructType => Self::StructType,
            crate::lexer::token_type::GoTokenType::FieldDeclList => Self::FieldDeclList,
            crate::lexer::token_type::GoTokenType::FieldDecl => Self::FieldDecl,
            crate::lexer::token_type::GoTokenType::InterfaceType => Self::InterfaceType,
            crate::lexer::token_type::GoTokenType::MethodSpecList => Self::MethodSpecList,
            crate::lexer::token_type::GoTokenType::MethodSpec => Self::MethodSpec,
            crate::lexer::token_type::GoTokenType::ExpressionList => Self::ExpressionList,
            crate::lexer::token_type::GoTokenType::AssignmentStatement => Self::AssignmentStatement,
            crate::lexer::token_type::GoTokenType::ShortVarDecl => Self::ShortVarDecl,
            crate::lexer::token_type::GoTokenType::ReturnStatement => Self::ReturnStatement,
            crate::lexer::token_type::GoTokenType::IfStatement => Self::IfStatement,
            crate::lexer::token_type::GoTokenType::ForStatement => Self::ForStatement,
            crate::lexer::token_type::GoTokenType::SwitchStatement => Self::SwitchStatement,
            crate::lexer::token_type::GoTokenType::ExprCaseClause => Self::ExprCaseClause,
            crate::lexer::token_type::GoTokenType::TypeSwitchStatement => Self::TypeSwitchStatement,
            crate::lexer::token_type::GoTokenType::TypeCaseClause => Self::TypeCaseClause,
            crate::lexer::token_type::GoTokenType::SelectStatement => Self::SelectStatement,
            crate::lexer::token_type::GoTokenType::CommClause => Self::CommClause,
            crate::lexer::token_type::GoTokenType::GoStatement => Self::GoStatement,
            crate::lexer::token_type::GoTokenType::DeferStatement => Self::DeferStatement,
            crate::lexer::token_type::GoTokenType::CallExpression => Self::CallExpression,
            crate::lexer::token_type::GoTokenType::IndexExpression => Self::IndexExpression,
            crate::lexer::token_type::GoTokenType::SelectorExpression => Self::SelectorExpression,
            crate::lexer::token_type::GoTokenType::SliceExpression => Self::SliceExpression,
            crate::lexer::token_type::GoTokenType::TypeAssertion => Self::TypeAssertion,
            crate::lexer::token_type::GoTokenType::UnaryExpression => Self::UnaryExpression,
            crate::lexer::token_type::GoTokenType::BinaryExpression => Self::BinaryExpression,
            crate::lexer::token_type::GoTokenType::LiteralValue => Self::LiteralValue,
            crate::lexer::token_type::GoTokenType::ElementList => Self::ElementList,
            crate::lexer::token_type::GoTokenType::KeyedElement => Self::KeyedElement,
            crate::lexer::token_type::GoTokenType::IntLiteral => Self::IntLiteral,
            crate::lexer::token_type::GoTokenType::FloatLiteral => Self::FloatLiteral,
            crate::lexer::token_type::GoTokenType::StringLiteral => Self::StringLiteral,
            crate::lexer::token_type::GoTokenType::RuneLiteral => Self::RuneLiteral,
            crate::lexer::token_type::GoTokenType::BoolLiteral => Self::BoolLiteral,
            crate::lexer::token_type::GoTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::GoTokenType::Break => Self::Break,
            crate::lexer::token_type::GoTokenType::Case => Self::Case,
            crate::lexer::token_type::GoTokenType::Chan => Self::Chan,
            crate::lexer::token_type::GoTokenType::Const => Self::Const,
            crate::lexer::token_type::GoTokenType::Continue => Self::Continue,
            crate::lexer::token_type::GoTokenType::Default => Self::Default,
            crate::lexer::token_type::GoTokenType::Defer => Self::Defer,
            crate::lexer::token_type::GoTokenType::Else => Self::Else,
            crate::lexer::token_type::GoTokenType::Fallthrough => Self::Fallthrough,
            crate::lexer::token_type::GoTokenType::For => Self::For,
            crate::lexer::token_type::GoTokenType::Func => Self::Func,
            crate::lexer::token_type::GoTokenType::Go => Self::Go,
            crate::lexer::token_type::GoTokenType::Goto => Self::Goto,
            crate::lexer::token_type::GoTokenType::If => Self::If,
            crate::lexer::token_type::GoTokenType::Import => Self::Import,
            crate::lexer::token_type::GoTokenType::Interface => Self::Interface,
            crate::lexer::token_type::GoTokenType::Map => Self::Map,
            crate::lexer::token_type::GoTokenType::Package => Self::Package,
            crate::lexer::token_type::GoTokenType::Range => Self::Range,
            crate::lexer::token_type::GoTokenType::Return => Self::Return,
            crate::lexer::token_type::GoTokenType::Select => Self::Select,
            crate::lexer::token_type::GoTokenType::Struct => Self::Struct,
            crate::lexer::token_type::GoTokenType::Switch => Self::Switch,
            crate::lexer::token_type::GoTokenType::Type => Self::Type,
            crate::lexer::token_type::GoTokenType::Var => Self::Var,
            crate::lexer::token_type::GoTokenType::Bool => Self::Bool,
            crate::lexer::token_type::GoTokenType::Byte => Self::Byte,
            crate::lexer::token_type::GoTokenType::Complex64 => Self::Complex64,
            crate::lexer::token_type::GoTokenType::Complex128 => Self::Complex128,
            crate::lexer::token_type::GoTokenType::ErrorType => Self::ErrorType,
            crate::lexer::token_type::GoTokenType::Float32 => Self::Float32,
            crate::lexer::token_type::GoTokenType::Float64 => Self::Float64,
            crate::lexer::token_type::GoTokenType::Int => Self::Int,
            crate::lexer::token_type::GoTokenType::Int8 => Self::Int8,
            crate::lexer::token_type::GoTokenType::Int16 => Self::Int16,
            crate::lexer::token_type::GoTokenType::Int32 => Self::Int32,
            crate::lexer::token_type::GoTokenType::Int64 => Self::Int64,
            crate::lexer::token_type::GoTokenType::Rune => Self::Rune,
            crate::lexer::token_type::GoTokenType::String => Self::String,
            crate::lexer::token_type::GoTokenType::Uint => Self::Uint,
            crate::lexer::token_type::GoTokenType::Uint8 => Self::Uint8,
            crate::lexer::token_type::GoTokenType::Uint16 => Self::Uint16,
            crate::lexer::token_type::GoTokenType::Uint32 => Self::Uint32,
            crate::lexer::token_type::GoTokenType::Uint64 => Self::Uint64,
            crate::lexer::token_type::GoTokenType::Uintptr => Self::Uintptr,
            crate::lexer::token_type::GoTokenType::NilLiteral => Self::NilLiteral,
            crate::lexer::token_type::GoTokenType::NumberLiteral => Self::NumberLiteral,
            crate::lexer::token_type::GoTokenType::CharLiteral => Self::CharLiteral,
            crate::lexer::token_type::GoTokenType::Plus => Self::Plus,
            crate::lexer::token_type::GoTokenType::Minus => Self::Minus,
            crate::lexer::token_type::GoTokenType::Star => Self::Star,
            crate::lexer::token_type::GoTokenType::Slash => Self::Slash,
            crate::lexer::token_type::GoTokenType::Percent => Self::Percent,
            crate::lexer::token_type::GoTokenType::Ampersand => Self::Ampersand,
            crate::lexer::token_type::GoTokenType::Pipe => Self::Pipe,
            crate::lexer::token_type::GoTokenType::Caret => Self::Caret,
            crate::lexer::token_type::GoTokenType::LeftShift => Self::LeftShift,
            crate::lexer::token_type::GoTokenType::RightShift => Self::RightShift,
            crate::lexer::token_type::GoTokenType::AmpersandCaret => Self::AmpersandCaret,
            crate::lexer::token_type::GoTokenType::PlusAssign => Self::PlusAssign,
            crate::lexer::token_type::GoTokenType::MinusAssign => Self::MinusAssign,
            crate::lexer::token_type::GoTokenType::StarAssign => Self::StarAssign,
            crate::lexer::token_type::GoTokenType::SlashAssign => Self::SlashAssign,
            crate::lexer::token_type::GoTokenType::PercentAssign => Self::PercentAssign,
            crate::lexer::token_type::GoTokenType::AmpersandAssign => Self::AmpersandAssign,
            crate::lexer::token_type::GoTokenType::PipeAssign => Self::PipeAssign,
            crate::lexer::token_type::GoTokenType::CaretAssign => Self::CaretAssign,
            crate::lexer::token_type::GoTokenType::XorAssign => Self::XorAssign,
            crate::lexer::token_type::GoTokenType::LeftShiftAssign => Self::LeftShiftAssign,
            crate::lexer::token_type::GoTokenType::RightShiftAssign => Self::RightShiftAssign,
            crate::lexer::token_type::GoTokenType::AmpersandCaretAssign => Self::AmpersandCaretAssign,
            crate::lexer::token_type::GoTokenType::AndAssign => Self::AndAssign,
            crate::lexer::token_type::GoTokenType::OrAssign => Self::OrAssign,
            crate::lexer::token_type::GoTokenType::AndNotAssign => Self::AndNotAssign,
            crate::lexer::token_type::GoTokenType::AndNot => Self::AndNot,
            crate::lexer::token_type::GoTokenType::LogicalAnd => Self::LogicalAnd,
            crate::lexer::token_type::GoTokenType::LogicalOr => Self::LogicalOr,
            crate::lexer::token_type::GoTokenType::And => Self::And,
            crate::lexer::token_type::GoTokenType::Or => Self::Or,
            crate::lexer::token_type::GoTokenType::Arrow => Self::Arrow,
            crate::lexer::token_type::GoTokenType::LeftArrow => Self::LeftArrow,
            crate::lexer::token_type::GoTokenType::Increment => Self::Increment,
            crate::lexer::token_type::GoTokenType::Decrement => Self::Decrement,
            crate::lexer::token_type::GoTokenType::Equal => Self::Equal,
            crate::lexer::token_type::GoTokenType::Less => Self::Less,
            crate::lexer::token_type::GoTokenType::Greater => Self::Greater,
            crate::lexer::token_type::GoTokenType::Assign => Self::Assign,
            crate::lexer::token_type::GoTokenType::LogicalNot => Self::LogicalNot,
            crate::lexer::token_type::GoTokenType::Not => Self::Not,
            crate::lexer::token_type::GoTokenType::NotEqual => Self::NotEqual,
            crate::lexer::token_type::GoTokenType::LessEqual => Self::LessEqual,
            crate::lexer::token_type::GoTokenType::GreaterEqual => Self::GreaterEqual,
            crate::lexer::token_type::GoTokenType::ColonAssign => Self::ColonAssign,
            crate::lexer::token_type::GoTokenType::Define => Self::Define,
            crate::lexer::token_type::GoTokenType::Ellipsis => Self::Ellipsis,
            crate::lexer::token_type::GoTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::GoTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::GoTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::GoTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::GoTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::GoTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::GoTokenType::Comma => Self::Comma,
            crate::lexer::token_type::GoTokenType::Period => Self::Period,
            crate::lexer::token_type::GoTokenType::Dot => Self::Dot,
            crate::lexer::token_type::GoTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::GoTokenType::Colon => Self::Colon,
            crate::lexer::token_type::GoTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::GoTokenType::Comment => Self::Comment,
            crate::lexer::token_type::GoTokenType::Eof => Self::Eof,
            crate::lexer::token_type::GoTokenType::Error => Self::Error,
            _ => Self::Error,
        }
    }
}
