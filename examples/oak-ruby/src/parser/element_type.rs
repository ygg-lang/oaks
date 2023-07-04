use oak_core::{ElementType, UniversalElementRole};
use std::fmt;

/// Element types for Ruby.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum RubyElementType {
    /// An identifier.
    Identifier,
    /// A global variable (e.g., `$stdout`).
    GlobalVariable,
    /// An instance variable (e.g., `@name`).
    InstanceVariable,
    /// A class variable (e.g., `@@name`).
    ClassVariable,
    /// A constant (e.g., `MATH`).
    Constant,
    /// An integer literal.
    IntegerLiteral,
    /// A float literal.
    FloatLiteral,
    /// A string literal.
    StringLiteral,
    /// A general literal.
    Literal,
    /// A symbol (e.g., `:name`).
    Symbol,
    /// A regular expression literal.
    RegexLiteral,

    /// The `if` keyword.
    If,
    /// The `unless` keyword.
    Unless,
    /// The `elsif` keyword.
    Elsif,
    /// The `else` keyword.
    Else,
    /// The `case` keyword.
    Case,
    /// The `when` keyword.
    When,
    /// The `then` keyword.
    Then,
    /// The `for` keyword.
    For,
    /// The `while` keyword.
    While,
    /// The `until` keyword.
    Until,
    /// The `break` keyword.
    Break,
    /// The `next` keyword.
    Next,
    /// The `redo` keyword.
    Redo,
    /// The `retry` keyword.
    Retry,
    /// The `return` keyword.
    Return,
    /// The `yield` keyword.
    Yield,
    /// The `def` keyword.
    Def,
    /// The `class` keyword.
    Class,
    /// The `module` keyword.
    Module,
    /// The `end` keyword.
    End,
    /// The `lambda` keyword.
    Lambda,
    /// The `proc` keyword.
    Proc,
    /// The `begin` keyword.
    Begin,
    /// The `rescue` keyword.
    Rescue,
    /// The `ensure` keyword.
    Ensure,
    /// The `raise` keyword.
    Raise,
    /// The `require` keyword.
    Require,
    /// The `load` keyword.
    Load,
    /// The `include` keyword.
    Include,
    /// The `extend` keyword.
    Extend,
    /// The `prepend` keyword.
    Prepend,
    /// The `and` keyword.
    And,
    /// The `or` keyword.
    Or,
    /// The `not` keyword.
    Not,
    /// The `in` keyword.
    In,
    /// The `true` keyword.
    True,
    /// The `false` keyword.
    False,
    /// The `nil` keyword.
    Nil,
    /// The `super` keyword.
    Super,
    /// The `self` keyword.
    Self_,
    /// The `alias` keyword.
    Alias,
    /// The `undef` keyword.
    Undef,
    /// The `defined?` keyword.
    Defined,
    /// The `do` keyword.
    Do,

    /// Plus operator `+`.
    Plus,
    /// Minus operator `-`.
    Minus,
    /// Multiply operator `*`.
    Multiply,
    /// Divide operator `/`.
    Divide,
    /// Modulo operator `%`.
    Modulo,
    /// Power operator `**`.
    Power,
    /// Equality operator `==`.
    EqualEqual,
    /// Inequality operator `!=`.
    NotEqual,
    /// Less than operator `<`.
    Less,
    /// Greater than operator `>`.
    Greater,
    /// Less than or equal operator `<=`.
    LessEqual,
    /// Greater than or equal operator `>=`.
    GreaterEqual,
    /// Case equality operator `===`.
    EqualEqualEqual,
    /// Spaceship operator `<=>`.
    Spaceship,
    /// Assignment operator `=`.
    Assign,
    /// Plus assignment operator `+=`.
    PlusAssign,
    /// Minus assignment operator `-=`.
    MinusAssign,
    /// Multiply assignment operator `*=`.
    MultiplyAssign,
    /// Divide assignment operator `/=`.
    DivideAssign,
    /// Modulo assignment operator `%=`.
    ModuloAssign,
    /// Power assignment operator `**=`.
    PowerAssign,
    /// Bitwise AND operator `&`.
    BitAnd,
    /// Bitwise OR operator `|`.
    BitOr,
    /// Bitwise XOR operator `^`.
    Xor,
    /// Logical NOT operator `!`.
    LogicalNot,
    /// Bitwise NOT operator `~`.
    Tilde,
    /// Left shift operator `<<`.
    LeftShift,
    /// Right shift operator `>>`.
    RightShift,
    /// AND assignment operator `&=`.
    AndAssign,
    /// OR assignment operator `|=`.
    OrAssign,
    /// XOR assignment operator `^=`.
    XorAssign,
    /// Left shift assignment operator `<<=`.
    LeftShiftAssign,
    /// Right shift assignment operator `>>=`.
    RightShiftAssign,
    /// Logical AND operator `&&`.
    AndAnd,
    /// Logical OR operator `||`.
    OrOr,
    /// OR OR assignment operator `||=`.
    OrOrAssign,
    /// AND AND assignment operator `&&=`.
    AndAndAssign,
    /// Question mark `?`.
    Question,
    /// Range operator `..`.
    DotDot,
    /// Range operator `...`.
    DotDotDot,
    /// Match operator `=~`.
    Match,
    /// Not match operator `!~`.
    NotMatch,

    /// Left parenthesis `(`.
    LeftParen,
    /// Right parenthesis `)`.
    RightParen,
    /// Left bracket `[`.
    LeftBracket,
    /// Right bracket `]`.
    RightBracket,
    /// Left brace `{`.
    LeftBrace,
    /// Right brace `}`.
    RightBrace,
    /// Comma `,`.
    Comma,
    /// Colon `:`.
    Colon,
    /// Semicolon `;`.
    Semicolon,
    /// Dot `.`.
    Dot,
    /// Double colon `::`.
    DoubleColon,
    /// At symbol `@`.
    At,
    /// Dollar sign `$`.
    Dollar,

    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// Comment.
    Comment,
    /// End of file.
    Eof,
    /// Invalid token.
    Invalid,
    /// Root element.
    Root,
    /// A binary expression.
    BinaryExpression,
    /// A unary expression.
    UnaryExpression,
    /// A literal expression.
    LiteralExpression,
    /// A parenthesized expression.
    ParenExpression,
    /// A parenthesized expression (alternative).
    ParenthesizedExpression,
    /// A method definition.
    MethodDefinition,
    /// A class definition.
    ClassDefinition,
    /// A module definition.
    ModuleDefinition,
    /// An if statement.
    IfStatement,
    /// A while statement.
    WhileStatement,
    /// An unless statement.
    UnlessStatement,
    /// An until statement.
    UntilStatement,
    /// A for statement.
    ForStatement,
    /// A case statement.
    CaseStatement,
    /// A when clause.
    WhenClause,
    /// A begin statement.
    BeginStatement,
    /// A rescue clause.
    RescueClause,
    /// An ensure clause.
    EnsureClause,
    /// A return statement.
    ReturnStatement,
    /// An if expression.
    IfExpression,
    /// A call expression.
    CallExpression,
    /// A member access expression.
    MemberAccess,
    /// A parameter list.
    ParameterList,
    /// An argument list.
    ArgumentList,
    /// An error element.
    Error,
    /// Equal operator `=`.
    Equal,
}

impl RubyElementType {
    /// Returns true if the element is ignored (whitespace, newline, or comment).
    pub fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    /// Returns true if the element is a keyword.
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::If
                | Self::Unless
                | Self::Elsif
                | Self::Else
                | Self::Case
                | Self::When
                | Self::Then
                | Self::For
                | Self::While
                | Self::Until
                | Self::Break
                | Self::Next
                | Self::Redo
                | Self::Retry
                | Self::Return
                | Self::Yield
                | Self::Def
                | Self::Class
                | Self::Module
                | Self::End
                | Self::Lambda
                | Self::Proc
                | Self::Begin
                | Self::Rescue
                | Self::Ensure
                | Self::Raise
                | Self::Require
                | Self::Load
                | Self::Include
                | Self::Extend
                | Self::Prepend
                | Self::And
                | Self::Or
                | Self::Not
                | Self::In
                | Self::True
                | Self::False
                | Self::Nil
                | Self::Super
                | Self::Self_
                | Self::Alias
                | Self::Undef
                | Self::Defined
                | Self::Do
        )
    }
}

impl fmt::Display for RubyElementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Identifier => "Identifier",
            Self::GlobalVariable => "GlobalVariable",
            Self::InstanceVariable => "InstanceVariable",
            Self::ClassVariable => "ClassVariable",
            Self::Constant => "Constant",
            Self::IntegerLiteral => "IntegerLiteral",
            Self::FloatLiteral => "FloatLiteral",
            Self::StringLiteral => "StringLiteral",
            Self::Literal => "Literal",
            Self::Symbol => "Symbol",
            Self::RegexLiteral => "RegexLiteral",

            Self::If => "If",
            Self::Unless => "Unless",
            Self::Elsif => "Elsif",
            Self::Else => "Else",
            Self::Case => "Case",
            Self::When => "When",
            Self::Then => "Then",
            Self::For => "For",
            Self::While => "While",
            Self::Until => "Until",
            Self::Break => "Break",
            Self::Next => "Next",
            Self::Redo => "Redo",
            Self::Retry => "Retry",
            Self::Return => "Return",
            Self::Yield => "Yield",
            Self::Def => "Def",
            Self::Class => "Class",
            Self::Module => "Module",
            Self::End => "End",
            Self::Lambda => "Lambda",
            Self::Proc => "Proc",
            Self::Begin => "Begin",
            Self::Rescue => "Rescue",
            Self::Ensure => "Ensure",
            Self::Raise => "Raise",
            Self::Require => "Require",
            Self::Load => "Load",
            Self::Include => "Include",
            Self::Extend => "Extend",
            Self::Prepend => "Prepend",
            Self::And => "And",
            Self::Or => "Or",
            Self::Not => "Not",
            Self::In => "In",
            Self::True => "True",
            Self::False => "False",
            Self::Nil => "Nil",
            Self::Super => "Super",
            Self::Self_ => "Self",
            Self::Alias => "Alias",
            Self::Undef => "Undef",
            Self::Defined => "Defined",
            Self::Do => "Do",

            Self::Plus => "Plus",
            Self::Minus => "Minus",
            Self::Multiply => "Multiply",
            Self::Divide => "Divide",
            Self::Modulo => "Modulo",
            Self::Power => "Power",
            Self::EqualEqual => "EqualEqual",
            Self::NotEqual => "NotEqual",
            Self::Less => "Less",
            Self::Greater => "Greater",
            Self::LessEqual => "LessEqual",
            Self::GreaterEqual => "GreaterEqual",
            Self::EqualEqualEqual => "EqualEqualEqual",
            Self::Spaceship => "Spaceship",
            Self::Assign => "Assign",
            Self::PlusAssign => "PlusAssign",
            Self::MinusAssign => "MinusAssign",
            Self::MultiplyAssign => "MultiplyAssign",
            Self::DivideAssign => "DivideAssign",
            Self::ModuloAssign => "ModuloAssign",
            Self::PowerAssign => "PowerAssign",
            Self::BitAnd => "BitAnd",
            Self::BitOr => "BitOr",
            Self::Xor => "Xor",
            Self::LogicalNot => "LogicalNot",
            Self::Tilde => "Tilde",
            Self::LeftShift => "LeftShift",
            Self::RightShift => "RightShift",
            Self::AndAssign => "AndAssign",
            Self::OrAssign => "OrAssign",
            Self::XorAssign => "XorAssign",
            Self::LeftShiftAssign => "LeftShiftAssign",
            Self::RightShiftAssign => "RightShiftAssign",
            Self::AndAnd => "AndAnd",
            Self::OrOr => "OrOr",
            Self::OrOrAssign => "OrOrAssign",
            Self::AndAndAssign => "AndAndAssign",
            Self::Question => "Question",
            Self::DotDot => "DotDot",
            Self::DotDotDot => "DotDotDot",
            Self::Match => "Match",
            Self::NotMatch => "NotMatch",

            Self::LeftParen => "LeftParen",
            Self::RightParen => "RightParen",
            Self::LeftBracket => "LeftBracket",
            Self::RightBracket => "RightBracket",
            Self::LeftBrace => "LeftBrace",
            Self::RightBrace => "RightBrace",
            Self::Comma => "Comma",
            Self::Colon => "Colon",
            Self::Semicolon => "Semicolon",
            Self::Dot => "Dot",
            Self::DoubleColon => "DoubleColon",
            Self::At => "At",
            Self::Dollar => "Dollar",

            Self::Whitespace => "Whitespace",
            Self::Newline => "Newline",
            Self::Comment => "Comment",
            Self::Eof => "Eof",
            Self::Invalid => "Invalid",
            Self::Root => "Root",
            Self::BinaryExpression => "BinaryExpression",
            Self::UnaryExpression => "UnaryExpression",
            Self::LiteralExpression => "LiteralExpression",
            Self::ParenExpression => "ParenExpression",
            Self::ParenthesizedExpression => "ParenthesizedExpression",
            Self::MethodDefinition => "MethodDefinition",
            Self::ClassDefinition => "ClassDefinition",
            Self::ModuleDefinition => "ModuleDefinition",
            Self::IfStatement => "IfStatement",
            Self::WhileStatement => "WhileStatement",
            Self::UnlessStatement => "UnlessStatement",
            Self::UntilStatement => "UntilStatement",
            Self::ForStatement => "ForStatement",
            Self::CaseStatement => "CaseStatement",
            Self::WhenClause => "WhenClause",
            Self::BeginStatement => "BeginStatement",
            Self::RescueClause => "RescueClause",
            Self::EnsureClause => "EnsureClause",
            Self::ReturnStatement => "ReturnStatement",
            Self::IfExpression => "IfExpression",
            Self::CallExpression => "CallExpression",
            Self::MemberAccess => "MemberAccess",
            Self::ParameterList => "ParameterList",
            Self::ArgumentList => "ArgumentList",
            Self::Error => "Error",
            Self::Equal => "Equal",
        };
        write!(f, "{}", name)
    }
}

impl ElementType for RubyElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::RubyTokenType> for RubyElementType {
    fn from(token: crate::lexer::token_type::RubyTokenType) -> Self {
                match token {
            crate::lexer::token_type::RubyTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::RubyTokenType::GlobalVariable => Self::GlobalVariable,
            crate::lexer::token_type::RubyTokenType::InstanceVariable => Self::InstanceVariable,
            crate::lexer::token_type::RubyTokenType::ClassVariable => Self::ClassVariable,
            crate::lexer::token_type::RubyTokenType::Constant => Self::Constant,
            crate::lexer::token_type::RubyTokenType::IntegerLiteral => Self::IntegerLiteral,
            crate::lexer::token_type::RubyTokenType::FloatLiteral => Self::FloatLiteral,
            crate::lexer::token_type::RubyTokenType::StringLiteral => Self::StringLiteral,
            crate::lexer::token_type::RubyTokenType::Literal => Self::Literal,
            crate::lexer::token_type::RubyTokenType::Symbol => Self::Symbol,
            crate::lexer::token_type::RubyTokenType::RegexLiteral => Self::RegexLiteral,
            crate::lexer::token_type::RubyTokenType::If => Self::If,
            crate::lexer::token_type::RubyTokenType::Unless => Self::Unless,
            crate::lexer::token_type::RubyTokenType::Elsif => Self::Elsif,
            crate::lexer::token_type::RubyTokenType::Else => Self::Else,
            crate::lexer::token_type::RubyTokenType::Case => Self::Case,
            crate::lexer::token_type::RubyTokenType::When => Self::When,
            crate::lexer::token_type::RubyTokenType::Then => Self::Then,
            crate::lexer::token_type::RubyTokenType::For => Self::For,
            crate::lexer::token_type::RubyTokenType::While => Self::While,
            crate::lexer::token_type::RubyTokenType::Until => Self::Until,
            crate::lexer::token_type::RubyTokenType::Break => Self::Break,
            crate::lexer::token_type::RubyTokenType::Next => Self::Next,
            crate::lexer::token_type::RubyTokenType::Redo => Self::Redo,
            crate::lexer::token_type::RubyTokenType::Retry => Self::Retry,
            crate::lexer::token_type::RubyTokenType::Return => Self::Return,
            crate::lexer::token_type::RubyTokenType::Yield => Self::Yield,
            crate::lexer::token_type::RubyTokenType::Def => Self::Def,
            crate::lexer::token_type::RubyTokenType::Class => Self::Class,
            crate::lexer::token_type::RubyTokenType::Module => Self::Module,
            crate::lexer::token_type::RubyTokenType::End => Self::End,
            crate::lexer::token_type::RubyTokenType::Lambda => Self::Lambda,
            crate::lexer::token_type::RubyTokenType::Proc => Self::Proc,
            crate::lexer::token_type::RubyTokenType::Begin => Self::Begin,
            crate::lexer::token_type::RubyTokenType::Rescue => Self::Rescue,
            crate::lexer::token_type::RubyTokenType::Ensure => Self::Ensure,
            crate::lexer::token_type::RubyTokenType::Raise => Self::Raise,
            crate::lexer::token_type::RubyTokenType::Require => Self::Require,
            crate::lexer::token_type::RubyTokenType::Load => Self::Load,
            crate::lexer::token_type::RubyTokenType::Include => Self::Include,
            crate::lexer::token_type::RubyTokenType::Extend => Self::Extend,
            crate::lexer::token_type::RubyTokenType::Prepend => Self::Prepend,
            crate::lexer::token_type::RubyTokenType::And => Self::And,
            crate::lexer::token_type::RubyTokenType::Or => Self::Or,
            crate::lexer::token_type::RubyTokenType::Not => Self::Not,
            crate::lexer::token_type::RubyTokenType::In => Self::In,
            crate::lexer::token_type::RubyTokenType::True => Self::True,
            crate::lexer::token_type::RubyTokenType::False => Self::False,
            crate::lexer::token_type::RubyTokenType::Nil => Self::Nil,
            crate::lexer::token_type::RubyTokenType::Super => Self::Super,
            crate::lexer::token_type::RubyTokenType::Self_ => Self::Self_,
            crate::lexer::token_type::RubyTokenType::Alias => Self::Alias,
            crate::lexer::token_type::RubyTokenType::Undef => Self::Undef,
            crate::lexer::token_type::RubyTokenType::Defined => Self::Defined,
            crate::lexer::token_type::RubyTokenType::Do => Self::Do,
            crate::lexer::token_type::RubyTokenType::Plus => Self::Plus,
            crate::lexer::token_type::RubyTokenType::Minus => Self::Minus,
            crate::lexer::token_type::RubyTokenType::Multiply => Self::Multiply,
            crate::lexer::token_type::RubyTokenType::Divide => Self::Divide,
            crate::lexer::token_type::RubyTokenType::Modulo => Self::Modulo,
            crate::lexer::token_type::RubyTokenType::Power => Self::Power,
            crate::lexer::token_type::RubyTokenType::EqualEqual => Self::EqualEqual,
            crate::lexer::token_type::RubyTokenType::NotEqual => Self::NotEqual,
            crate::lexer::token_type::RubyTokenType::Less => Self::Less,
            crate::lexer::token_type::RubyTokenType::Greater => Self::Greater,
            crate::lexer::token_type::RubyTokenType::LessEqual => Self::LessEqual,
            crate::lexer::token_type::RubyTokenType::GreaterEqual => Self::GreaterEqual,
            crate::lexer::token_type::RubyTokenType::EqualEqualEqual => Self::EqualEqualEqual,
            crate::lexer::token_type::RubyTokenType::Spaceship => Self::Spaceship,
            crate::lexer::token_type::RubyTokenType::Assign => Self::Assign,
            crate::lexer::token_type::RubyTokenType::PlusAssign => Self::PlusAssign,
            crate::lexer::token_type::RubyTokenType::MinusAssign => Self::MinusAssign,
            crate::lexer::token_type::RubyTokenType::MultiplyAssign => Self::MultiplyAssign,
            crate::lexer::token_type::RubyTokenType::DivideAssign => Self::DivideAssign,
            crate::lexer::token_type::RubyTokenType::ModuloAssign => Self::ModuloAssign,
            crate::lexer::token_type::RubyTokenType::PowerAssign => Self::PowerAssign,
            crate::lexer::token_type::RubyTokenType::BitAnd => Self::BitAnd,
            crate::lexer::token_type::RubyTokenType::BitOr => Self::BitOr,
            crate::lexer::token_type::RubyTokenType::Xor => Self::Xor,
            crate::lexer::token_type::RubyTokenType::LogicalNot => Self::LogicalNot,
            crate::lexer::token_type::RubyTokenType::Tilde => Self::Tilde,
            crate::lexer::token_type::RubyTokenType::LeftShift => Self::LeftShift,
            crate::lexer::token_type::RubyTokenType::RightShift => Self::RightShift,
            crate::lexer::token_type::RubyTokenType::AndAssign => Self::AndAssign,
            crate::lexer::token_type::RubyTokenType::OrAssign => Self::OrAssign,
            crate::lexer::token_type::RubyTokenType::XorAssign => Self::XorAssign,
            crate::lexer::token_type::RubyTokenType::LeftShiftAssign => Self::LeftShiftAssign,
            crate::lexer::token_type::RubyTokenType::RightShiftAssign => Self::RightShiftAssign,
            crate::lexer::token_type::RubyTokenType::AndAnd => Self::AndAnd,
            crate::lexer::token_type::RubyTokenType::OrOr => Self::OrOr,
            crate::lexer::token_type::RubyTokenType::OrOrAssign => Self::OrOrAssign,
            crate::lexer::token_type::RubyTokenType::AndAndAssign => Self::AndAndAssign,
            crate::lexer::token_type::RubyTokenType::Question => Self::Question,
            crate::lexer::token_type::RubyTokenType::DotDot => Self::DotDot,
            crate::lexer::token_type::RubyTokenType::DotDotDot => Self::DotDotDot,
            crate::lexer::token_type::RubyTokenType::Match => Self::Match,
            crate::lexer::token_type::RubyTokenType::NotMatch => Self::NotMatch,
            crate::lexer::token_type::RubyTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::RubyTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::RubyTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::RubyTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::RubyTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::RubyTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::RubyTokenType::Comma => Self::Comma,
            crate::lexer::token_type::RubyTokenType::Colon => Self::Colon,
            crate::lexer::token_type::RubyTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::RubyTokenType::Dot => Self::Dot,
            crate::lexer::token_type::RubyTokenType::DoubleColon => Self::DoubleColon,
            crate::lexer::token_type::RubyTokenType::At => Self::At,
            crate::lexer::token_type::RubyTokenType::Dollar => Self::Dollar,
            crate::lexer::token_type::RubyTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::RubyTokenType::Newline => Self::Newline,
            crate::lexer::token_type::RubyTokenType::Comment => Self::Comment,
            crate::lexer::token_type::RubyTokenType::Eof => Self::Eof,
            crate::lexer::token_type::RubyTokenType::Invalid => Self::Invalid,
            crate::lexer::token_type::RubyTokenType::Root => Self::Root,
            crate::lexer::token_type::RubyTokenType::BinaryExpression => Self::BinaryExpression,
            crate::lexer::token_type::RubyTokenType::UnaryExpression => Self::UnaryExpression,
            crate::lexer::token_type::RubyTokenType::LiteralExpression => Self::LiteralExpression,
            crate::lexer::token_type::RubyTokenType::ParenExpression => Self::ParenExpression,
            crate::lexer::token_type::RubyTokenType::ParenthesizedExpression => Self::ParenthesizedExpression,
            crate::lexer::token_type::RubyTokenType::MethodDefinition => Self::MethodDefinition,
            crate::lexer::token_type::RubyTokenType::ClassDefinition => Self::ClassDefinition,
            crate::lexer::token_type::RubyTokenType::ModuleDefinition => Self::ModuleDefinition,
            crate::lexer::token_type::RubyTokenType::IfStatement => Self::IfStatement,
            crate::lexer::token_type::RubyTokenType::WhileStatement => Self::WhileStatement,
            crate::lexer::token_type::RubyTokenType::ReturnStatement => Self::ReturnStatement,
            crate::lexer::token_type::RubyTokenType::IfExpression => Self::IfExpression,
            crate::lexer::token_type::RubyTokenType::CallExpression => Self::CallExpression,
            crate::lexer::token_type::RubyTokenType::MemberAccess => Self::MemberAccess,
            crate::lexer::token_type::RubyTokenType::ParameterList => Self::ParameterList,
            crate::lexer::token_type::RubyTokenType::ArgumentList => Self::ArgumentList,
            crate::lexer::token_type::RubyTokenType::Error => Self::Error,
            crate::lexer::token_type::RubyTokenType::Equal => Self::Equal,
            _ => Self::Error,
        }
    }
}
