//! JavaScript element types.

use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// JavaScript element types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum JavaScriptElementType {
    /// Root node
    Root,
    /// Statement
    Statement,
    /// Expression
    Expression,
    /// Block
    Block,
    /// Error node
    Error,

    /// Function declaration
    FunctionDeclaration,
    /// Variable declaration
    VariableDeclaration,
    /// If statement
    IfStatement,
    /// While statement
    WhileStatement,
    /// For statement
    ForStatement,
    /// Return statement
    ReturnStatement,
    /// Block statement
    BlockStatement,

    /// Identifier
    Identifier,
    /// Literal
    Literal,

    /// Call expression
    CallExpression,
    /// Member expression
    MemberExpression,
    /// Assignment expression
    AssignmentExpression,
    /// Logical expression
    LogicalExpression,
    /// Binary expression
    BinaryExpression,

    // Keywords
    /// `abstract`
    Abstract,
    /// `as`
    As,
    /// `async`
    Async,
    /// `await`
    Await,
    /// `break`
    Break,
    /// `case`
    Case,
    /// `catch`
    Catch,
    /// `class`
    Class,
    /// `const`
    Const,
    /// `continue`
    Continue,
    /// `debugger`
    Debugger,
    /// `default`
    Default,
    /// `delete`
    Delete,
    /// `do`
    Do,
    /// `else`
    Else,
    /// `enum`
    Enum,
    /// `export`
    Export,
    /// `extends`
    Extends,
    /// `false`
    False,
    /// `finally`
    Finally,
    /// `for`
    For,
    /// `function`
    Function,
    /// `if`
    If,
    /// `implements`
    Implements,
    /// `import`
    Import,
    /// `in`
    In,
    /// `instanceof`
    Instanceof,
    /// `interface`
    Interface,
    /// `let`
    Let,
    /// `new`
    New,
    /// `null`
    Null,
    /// `package`
    Package,
    /// `private`
    Private,
    /// `protected`
    Protected,
    /// `public`
    Public,
    /// `return`
    Return,
    /// `static`
    Static,
    /// `super`
    Super,
    /// `switch`
    Switch,
    /// `this`
    This,
    /// `throw`
    Throw,
    /// `true`
    True,
    /// `try`
    Try,
    /// `typeof`
    Typeof,
    /// `undefined`
    Undefined,
    /// `var`
    Var,
    /// `void`
    Void,
    /// `while`
    While,
    /// `with`
    With,
    /// `yield`
    Yield,
}

impl ElementType for JavaScriptElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        use UniversalElementRole::*;
        match self {
            Self::Root => Root,
            Self::Statement | Self::Block | Self::BlockStatement | Self::IfStatement | Self::WhileStatement | Self::ForStatement | Self::ReturnStatement | Self::FunctionDeclaration | Self::VariableDeclaration => Statement,
            Self::Expression | Self::AssignmentExpression | Self::LogicalExpression | Self::BinaryExpression | Self::CallExpression | Self::MemberExpression | Self::Identifier | Self::Literal => Expression,
            Self::Error => Error,
            _ => None,
        }
    }
}

impl JavaScriptElementType {
    /// Returns true if the element type is a keyword.
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::Abstract
                | Self::As
                | Self::Async
                | Self::Await
                | Self::Break
                | Self::Case
                | Self::Catch
                | Self::Class
                | Self::Const
                | Self::Continue
                | Self::Debugger
                | Self::Default
                | Self::Delete
                | Self::Do
                | Self::Else
                | Self::Enum
                | Self::Export
                | Self::Extends
                | Self::False
                | Self::Finally
                | Self::For
                | Self::Function
                | Self::If
                | Self::Implements
                | Self::Import
                | Self::In
                | Self::Instanceof
                | Self::Interface
                | Self::Let
                | Self::New
                | Self::Null
                | Self::Package
                | Self::Private
                | Self::Protected
                | Self::Public
                | Self::Return
                | Self::Static
                | Self::Super
                | Self::Switch
                | Self::This
                | Self::Throw
                | Self::True
                | Self::Try
                | Self::Typeof
                | Self::Undefined
                | Self::Var
                | Self::Void
                | Self::While
                | Self::With
                | Self::Yield
        )
    }

    /// Returns the element type for the given keyword string.
    pub fn from_keyword(s: &str) -> Option<Self> {
        match s {
            "abstract" => Some(Self::Abstract),
            "as" => Some(Self::As),
            "async" => Some(Self::Async),
            "await" => Some(Self::Await),
            "break" => Some(Self::Break),
            "case" => Some(Self::Case),
            "catch" => Some(Self::Catch),
            "class" => Some(Self::Class),
            "const" => Some(Self::Const),
            "continue" => Some(Self::Continue),
            "debugger" => Some(Self::Debugger),
            "default" => Some(Self::Default),
            "delete" => Some(Self::Delete),
            "do" => Some(Self::Do),
            "else" => Some(Self::Else),
            "enum" => Some(Self::Enum),
            "export" => Some(Self::Export),
            "extends" => Some(Self::Extends),
            "false" => Some(Self::False),
            "finally" => Some(Self::Finally),
            "for" => Some(Self::For),
            "function" => Some(Self::Function),
            "if" => Some(Self::If),
            "implements" => Some(Self::Implements),
            "import" => Some(Self::Import),
            "in" => Some(Self::In),
            "instanceof" => Some(Self::Instanceof),
            "interface" => Some(Self::Interface),
            "let" => Some(Self::Let),
            "new" => Some(Self::New),
            "null" => Some(Self::Null),
            "package" => Some(Self::Package),
            "private" => Some(Self::Private),
            "protected" => Some(Self::Protected),
            "public" => Some(Self::Public),
            "return" => Some(Self::Return),
            "static" => Some(Self::Static),
            "super" => Some(Self::Super),
            "switch" => Some(Self::Switch),
            "this" => Some(Self::This),
            "throw" => Some(Self::Throw),
            "true" => Some(Self::True),
            "try" => Some(Self::Try),
            "typeof" => Some(Self::Typeof),
            "undefined" => Some(Self::Undefined),
            "var" => Some(Self::Var),
            "void" => Some(Self::Void),
            "while" => Some(Self::While),
            "with" => Some(Self::With),
            "yield" => Some(Self::Yield),
            _ => None,
        }
    }
}

impl From<crate::lexer::token_type::JavaScriptTokenType> for JavaScriptElementType {
    fn from(token: crate::lexer::token_type::JavaScriptTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
