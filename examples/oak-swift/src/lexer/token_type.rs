use oak_core::{Token, TokenType, UniversalTokenRole};

/// Token type for Swift lexer output.
pub type SwiftToken = Token<SwiftTokenType>;

/// Token types for the Swift lexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum SwiftTokenType {
    /// Whitespace characters.
    Whitespace,
    /// Line breaks.
    Newline,
    /// Comments.
    Comment,
    /// Identifiers.
    Identifier,
    /// Error token.
    Error,
    /// End of stream.
    Eof,
    /// Numeric literals.
    NumberLiteral,
    /// String literals.
    StringLiteral,
    /// Character literals.
    CharLiteral,
    /// Boolean literals.
    BooleanLiteral,

    /// The `class` keyword.
    Class,
    /// The `struct` keyword.
    Struct,
    /// The `enum` keyword.
    Enum,
    /// The `protocol` keyword.
    Protocol,
    /// The `extension` keyword.
    Extension,
    /// The `func` keyword.
    Func,
    /// The `var` keyword.
    Var,
    /// The `let` keyword.
    Let,
    /// The `init` keyword.
    Init,
    /// The `deinit` keyword.
    Deinit,
    /// The `subscript` keyword.
    Subscript,
    /// The `typealias` keyword.
    Typealias,
    /// The `import` keyword.
    Import,
    /// The `if` keyword.
    If,
    /// The `else` keyword.
    Else,
    /// The `switch` keyword.
    Switch,
    /// The `case` keyword.
    Case,
    /// The `default` keyword.
    Default,
    /// The `for` keyword.
    For,
    /// The `while` keyword.
    While,
    /// The `repeat` keyword.
    Repeat,
    /// The `do` keyword.
    Do,
    /// The `break` keyword.
    Break,
    /// The `continue` keyword.
    Continue,
    /// The `fallthrough` keyword.
    Fallthrough,
    /// The `return` keyword.
    Return,
    /// The `throw` keyword.
    Throw,
    /// The `try` keyword.
    Try,
    /// The `catch` keyword.
    Catch,
    /// The `finally` keyword.
    Finally,
    /// The `guard` keyword.
    Guard,
    /// The `defer` keyword.
    Defer,
    /// The `public` keyword.
    Public,
    /// The `private` keyword.
    Private,
    /// The `internal` keyword.
    Internal,
    /// The `fileprivate` keyword.
    Fileprivate,
    /// The `open` keyword.
    Open,
    /// The `static` keyword.
    Static,
    /// The `final` keyword.
    Final,
    /// The `override` keyword.
    Override,
    /// The `mutating` keyword.
    Mutating,
    /// The `nonmutating` keyword.
    Nonmutating,
    /// The `lazy` keyword.
    Lazy,
    /// The `weak` keyword.
    Weak,
    /// The `unowned` keyword.
    Unowned,
    /// The `optional` keyword.
    Optional,
    /// The `required` keyword.
    Required,
    /// The `convenience` keyword.
    Convenience,
    /// The `dynamic` keyword.
    Dynamic,
    /// The `infix` keyword.
    Infix,
    /// The `prefix` keyword.
    Prefix,
    /// The `postfix` keyword.
    Postfix,
    /// The `Any` keyword.
    Any,
    /// The `AnyObject` keyword.
    AnyObject,
    /// The `self` keyword.
    Self_,
    /// The `Self` keyword.
    Type,
    /// The `Protocol` keyword.
    Protocol_,
    /// The `true` keyword.
    True,
    /// The `false` keyword.
    False,
    /// The `nil` keyword.
    Nil,
    /// The `as` keyword.
    As,
    /// The `is` keyword.
    Is,
    /// The `in` keyword.
    In,
    /// The `where` keyword.
    Where,
    /// The `associatedtype` keyword.
    Associatedtype,
    /// The `operator` keyword.
    Operator,
    /// The `precedencegroup` keyword.
    Precedencegroup,
    /// The `indirect` keyword.
    Indirect,
    /// The `rethrows` keyword.
    Rethrows,
    /// The `throws` keyword.
    Throws,
    /// The `inout` keyword.
    Inout,

    /// Plus operator (`+`).
    Plus,
    /// Minus operator (`-`).
    Minus,
    /// Multiplication operator (`*`).
    Star,
    /// Division operator (`/`).
    Slash,
    /// Modulo operator (`%`).
    Percent,
    /// Equality operator (`==`).
    Equal,
    /// Inequality operator (`!=`).
    NotEqual,
    /// Less than operator (`<`).
    Less,
    /// Greater than operator (`>`).
    Greater,
    /// Less than or equal to operator (`<=`).
    LessEqual,
    /// Greater than or equal to operator (`>=`).
    GreaterEqual,
    /// Logical AND operator (`&&`).
    LogicalAnd,
    /// Logical OR operator (`||`).
    LogicalOr,
    /// Logical NOT operator (`!`).
    LogicalNot,
    /// Bitwise AND operator (`&`).
    BitAnd,
    /// Bitwise OR operator (`|`).
    BitOr,
    /// Bitwise XOR operator (`^`).
    BitXor,
    /// Bitwise NOT operator (`~`).
    BitNot,
    /// Left shift operator (`<<`).
    LeftShift,
    /// Right shift operator (`>>`).
    RightShift,
    /// Assignment operator (`=`).
    Assign,
    /// Plus assignment operator (`+=`).
    PlusAssign,
    /// Minus assignment operator (`-=`).
    MinusAssign,
    /// Multiplication assignment operator (`*=`).
    StarAssign,
    /// Division assignment operator (`/=`).
    SlashAssign,
    /// Modulo assignment operator (`%=`).
    PercentAssign,
    /// Bitwise AND assignment operator (`&=`).
    AndAssign,
    /// Bitwise OR assignment operator (`|=`).
    OrAssign,
    /// Bitwise XOR assignment operator (`^=`).
    XorAssign,
    /// Left shift assignment operator (`<<=`).
    LeftShiftAssign,
    /// Right shift assignment operator (`>>=`).
    RightShiftAssign,

    /// Question mark (`?`).
    Question,
    /// Nil-coalescing operator (`??`).
    QuestionQuestion,
    /// Dot operator (`.`).
    Dot,
    /// Arrow operator (`->`).
    Arrow,
    /// Half-open range operator (`..<`).
    Range,
    /// Closed range operator (`...`).
    ClosedRange,
    /// Left parenthesis (`(`).
    LeftParen,
    /// Right parenthesis (`)`).
    RightParen,
    /// Left bracket (`[`).
    LeftBracket,
    /// Right bracket (`]`).
    RightBracket,
    /// Left brace (`{`).
    LeftBrace,
    /// Right brace (`}`).
    RightBrace,
    /// Comma (`,`).
    Comma,
    /// Semicolon (`;`).
    Semicolon,
    /// Colon (`:`).
    Colon,
    /// At sign (`@`).
    At,
    /// Hash sign (`#`).
    Hash,
    /// Dollar sign (`$`).
    Dollar,
    /// Underscore (`_`).
    Underscore,
    /// Backslash (`\`).
    Backslash,

    /// Source file node.
    SourceFile,
    /// Function declaration node.
    FunctionDeclaration,
    /// Parameter node.
    Parameter,
    /// Parameter list node.
    ParameterList,
    /// Variable declaration node.
    VariableDeclaration,
    /// Class declaration node.
    ClassDeclaration,
    /// Struct declaration node.
    StructDeclaration,
    /// Enum declaration node.
    EnumDeclaration,
    /// Protocol declaration node.
    ProtocolDeclaration,
    /// If statement node.
    IfStatement,
    /// While statement node.
    WhileStatement,
    /// For statement node.
    ForStatement,
    /// Return statement node.
    ReturnStatement,
    /// Break statement node.
    BreakStatement,
    /// Continue statement node.
    ContinueStatement,
    /// Expression statement node.
    ExpressionStatement,
    /// Code block node.
    Block,
    /// Binary expression node.
    BinaryExpression,
    /// Unary expression node.
    UnaryExpression,
    /// Function call expression node.
    CallExpression,
    /// Member access expression node.
    MemberExpression,
    /// Identifier expression node.
    IdentifierExpression,
    /// Literal expression node.
    LiteralExpression,
}

impl SwiftTokenType {
    /// Returns true if this token type is trivia (whitespace, newline, or comment).
    pub fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }
}

impl TokenType for SwiftTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
