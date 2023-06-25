use oak_core::{ElementType, UniversalElementRole};

/// Element types for the Swift parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum SwiftElementType {
    /// Whitespace characters.
    Whitespace,
    /// Line breaks.
    Newline,
    /// Comments.
    Comment,
    /// Identifiers.
    Identifier,
    /// Error element.
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

impl SwiftElementType {
    pub fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }
}

impl ElementType for SwiftElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::SourceFile => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::SwiftTokenType> for SwiftElementType {
    fn from(token: crate::lexer::token_type::SwiftTokenType) -> Self {
        use crate::lexer::token_type::SwiftTokenType as T;
        match token {
            T::Whitespace => Self::Whitespace,
            T::Newline => Self::Newline,
            T::Comment => Self::Comment,
            T::Identifier => Self::Identifier,
            T::Error => Self::Error,
            T::Eof => Self::Eof,
            T::NumberLiteral => Self::NumberLiteral,
            T::StringLiteral => Self::StringLiteral,
            T::CharLiteral => Self::CharLiteral,
            T::BooleanLiteral => Self::BooleanLiteral,
            T::Class => Self::Class,
            T::Struct => Self::Struct,
            T::Enum => Self::Enum,
            T::Protocol => Self::Protocol,
            T::Extension => Self::Extension,
            T::Func => Self::Func,
            T::Var => Self::Var,
            T::Let => Self::Let,
            T::Init => Self::Init,
            T::Deinit => Self::Deinit,
            T::Subscript => Self::Subscript,
            T::Typealias => Self::Typealias,
            T::Import => Self::Import,
            T::If => Self::If,
            T::Else => Self::Else,
            T::Switch => Self::Switch,
            T::Case => Self::Case,
            T::Default => Self::Default,
            T::For => Self::For,
            T::While => Self::While,
            T::Repeat => Self::Repeat,
            T::Do => Self::Do,
            T::Break => Self::Break,
            T::Continue => Self::Continue,
            T::Fallthrough => Self::Fallthrough,
            T::Return => Self::Return,
            T::Throw => Self::Throw,
            T::Try => Self::Try,
            T::Catch => Self::Catch,
            T::Finally => Self::Finally,
            T::Guard => Self::Guard,
            T::Defer => Self::Defer,
            T::Public => Self::Public,
            T::Private => Self::Private,
            T::Internal => Self::Internal,
            T::Fileprivate => Self::Fileprivate,
            T::Open => Self::Open,
            T::Static => Self::Static,
            T::Final => Self::Final,
            T::Override => Self::Override,
            T::Mutating => Self::Mutating,
            T::Nonmutating => Self::Nonmutating,
            T::Lazy => Self::Lazy,
            T::Weak => Self::Weak,
            T::Unowned => Self::Unowned,
            T::Optional => Self::Optional,
            T::Required => Self::Required,
            T::Convenience => Self::Convenience,
            T::Dynamic => Self::Dynamic,
            T::Infix => Self::Infix,
            T::Prefix => Self::Prefix,
            T::Postfix => Self::Postfix,
            T::Any => Self::Any,
            T::AnyObject => Self::AnyObject,
            T::Self_ => Self::Self_,
            T::Type => Self::Type,
            T::Protocol_ => Self::Protocol_,
            T::True => Self::True,
            T::False => Self::False,
            T::Nil => Self::Nil,
            T::As => Self::As,
            T::Is => Self::Is,
            T::In => Self::In,
            T::Where => Self::Where,
            T::Associatedtype => Self::Associatedtype,
            T::Operator => Self::Operator,
            T::Precedencegroup => Self::Precedencegroup,
            T::Indirect => Self::Indirect,
            T::Rethrows => Self::Rethrows,
            T::Throws => Self::Throws,
            T::Inout => Self::Inout,
            T::Plus => Self::Plus,
            T::Minus => Self::Minus,
            T::Star => Self::Star,
            T::Slash => Self::Slash,
            T::Percent => Self::Percent,
            T::Equal => Self::Equal,
            T::NotEqual => Self::NotEqual,
            T::Less => Self::Less,
            T::Greater => Self::Greater,
            T::LessEqual => Self::LessEqual,
            T::GreaterEqual => Self::GreaterEqual,
            T::LogicalAnd => Self::LogicalAnd,
            T::LogicalOr => Self::LogicalOr,
            T::LogicalNot => Self::LogicalNot,
            T::BitAnd => Self::BitAnd,
            T::BitOr => Self::BitOr,
            T::BitXor => Self::BitXor,
            T::BitNot => Self::BitNot,
            T::LeftShift => Self::LeftShift,
            T::RightShift => Self::RightShift,
            T::Assign => Self::Assign,
            T::PlusAssign => Self::PlusAssign,
            T::MinusAssign => Self::MinusAssign,
            T::StarAssign => Self::StarAssign,
            T::SlashAssign => Self::SlashAssign,
            T::PercentAssign => Self::PercentAssign,
            T::AndAssign => Self::AndAssign,
            T::OrAssign => Self::OrAssign,
            T::XorAssign => Self::XorAssign,
            T::LeftShiftAssign => Self::LeftShiftAssign,
            T::RightShiftAssign => Self::RightShiftAssign,
            T::Question => Self::Question,
            T::QuestionQuestion => Self::QuestionQuestion,
            T::Dot => Self::Dot,
            T::Arrow => Self::Arrow,
            T::Range => Self::Range,
            T::ClosedRange => Self::ClosedRange,
            T::LeftParen => Self::LeftParen,
            T::RightParen => Self::RightParen,
            T::LeftBracket => Self::LeftBracket,
            T::RightBracket => Self::RightBracket,
            T::LeftBrace => Self::LeftBrace,
            T::RightBrace => Self::RightBrace,
            T::Comma => Self::Comma,
            T::Semicolon => Self::Semicolon,
            T::Colon => Self::Colon,
            T::At => Self::At,
            T::Hash => Self::Hash,
            T::Dollar => Self::Dollar,
            T::Underscore => Self::Underscore,
            T::Backslash => Self::Backslash,
            T::SourceFile => Self::SourceFile,
            T::FunctionDeclaration => Self::FunctionDeclaration,
            T::Parameter => Self::Parameter,
            T::ParameterList => Self::ParameterList,
            T::VariableDeclaration => Self::VariableDeclaration,
            T::ClassDeclaration => Self::ClassDeclaration,
            T::StructDeclaration => Self::StructDeclaration,
            T::EnumDeclaration => Self::EnumDeclaration,
            T::ProtocolDeclaration => Self::ProtocolDeclaration,
            T::IfStatement => Self::IfStatement,
            T::WhileStatement => Self::WhileStatement,
            T::ForStatement => Self::ForStatement,
            T::ReturnStatement => Self::ReturnStatement,
            T::BreakStatement => Self::BreakStatement,
            T::ContinueStatement => Self::ContinueStatement,
            T::ExpressionStatement => Self::ExpressionStatement,
            T::Block => Self::Block,
            T::BinaryExpression => Self::BinaryExpression,
            T::UnaryExpression => Self::UnaryExpression,
            T::CallExpression => Self::CallExpression,
            T::MemberExpression => Self::MemberExpression,
            T::IdentifierExpression => Self::IdentifierExpression,
            T::LiteralExpression => Self::LiteralExpression,
        }
    }
}
