use oak_core::{Token, TokenType, UniversalTokenRole};

/// Token type for PHP
pub type PhpToken = Token<PhpTokenType>;

impl PhpTokenType {
    /// Checks if this syntax kind represents a token (leaf node).
    pub fn is_token(&self) -> bool {
        !self.is_element()
    }
}

impl PhpTokenType {
    /// Checks if this syntax kind represents a composite element (non-leaf node).
    pub fn is_element(&self) -> bool {
        matches!(
            self,
            Self::Root
                | Self::ClassDef
                | Self::FunctionDef
                | Self::MethodDef
                | Self::PropertyDef
                | Self::ConstDef
                | Self::TraitDef
                | Self::InterfaceDef
                | Self::NamespaceDef
                | Self::UseStatement
                | Self::IfStatement
                | Self::WhileStatement
                | Self::DoWhileStatement
                | Self::ForStatement
                | Self::ForeachStatement
                | Self::SwitchStatement
                | Self::TryStatement
                | Self::CatchBlock
                | Self::FinallyBlock
                | Self::ExpressionStatement
                | Self::ReturnStatement
                | Self::ThrowStatement
                | Self::BreakStatement
                | Self::ContinueStatement
                | Self::EchoStatement
                | Self::GlobalStatement
                | Self::StaticStatement
                | Self::UnsetStatement
                | Self::CompoundStatement
                | Self::Literal
                | Self::ParenthesizedExpression
                | Self::CallExpression
                | Self::ArrayAccessExpression
                | Self::MemberAccessExpression
                | Self::BinaryExpression
        )
    }
}

impl TokenType for PhpTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Identifier | Self::Variable => UniversalTokenRole::Name,
            Self::StringLiteral | Self::NumberLiteral => UniversalTokenRole::Literal,
            Self::BooleanLiteral | Self::NullLiteral => UniversalTokenRole::Literal,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            _ => UniversalTokenRole::None,
        }
    }
}

impl PhpTokenType {
    fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::Abstract
                | Self::And
                | Self::Array
                | Self::As
                | Self::Break
                | Self::Callable
                | Self::Case
                | Self::Catch
                | Self::Class
                | Self::Clone
                | Self::Const
                | Self::Continue
                | Self::Declare
                | Self::Default
                | Self::Do
                | Self::Echo
                | Self::Else
                | Self::Elseif
                | Self::Empty
                | Self::Enddeclare
                | Self::Endfor
                | Self::Endforeach
                | Self::Endif
                | Self::Endswitch
                | Self::Endwhile
                | Self::Eval
                | Self::Exit
                | Self::Extends
                | Self::Final
                | Self::Finally
                | Self::For
                | Self::Foreach
                | Self::Function
                | Self::Global
                | Self::Goto
                | Self::If
                | Self::Implements
                | Self::Include
                | Self::IncludeOnce
                | Self::Instanceof
                | Self::Insteadof
                | Self::Interface
                | Self::Isset
                | Self::List
                | Self::Namespace
                | Self::New
                | Self::Or
                | Self::Print
                | Self::Private
                | Self::Protected
                | Self::Public
                | Self::Require
                | Self::RequireOnce
                | Self::Return
                | Self::Static
                | Self::Switch
                | Self::Throw
                | Self::Trait
                | Self::Try
                | Self::Unset
                | Self::Use
                | Self::Var
                | Self::While
                | Self::Xor
                | Self::Yield
                | Self::YieldFrom
        )
    }
}

/// Enum representing all possible token types in PHP
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PhpTokenType {
    /// Whitespace characters
    Whitespace,
    /// Newline characters
    Newline,

    /// Comment
    Comment,

    /// String literal
    StringLiteral,
    /// Number literal
    NumberLiteral,
    /// Boolean literal (true, false)
    BooleanLiteral,
    /// Null literal
    NullLiteral,

    /// Identifier
    Identifier,
    /// Variable identifier (starting with $)
    Variable,
    /// 'abstract' keyword
    Abstract,
    /// 'and' operator
    And,
    /// 'array' keyword/type
    Array,
    /// 'as' keyword
    As,
    /// 'break' keyword
    Break,
    /// 'callable' type
    Callable,
    /// 'case' keyword
    Case,
    /// 'catch' keyword
    Catch,
    /// 'class' keyword
    Class,
    /// 'clone' keyword
    Clone,
    /// 'const' keyword
    Const,
    /// 'continue' keyword
    Continue,
    /// 'declare' keyword
    Declare,
    /// 'default' keyword
    Default,
    /// 'do' keyword
    Do,
    /// 'echo' keyword
    Echo,
    /// 'else' keyword
    Else,
    /// 'elseif' keyword
    Elseif,
    /// 'empty' keyword
    Empty,
    /// 'enddeclare' keyword
    Enddeclare,
    /// 'endfor' keyword
    Endfor,
    /// 'endforeach' keyword
    Endforeach,
    /// 'endif' keyword
    Endif,
    /// 'endswitch' keyword
    Endswitch,
    /// 'endwhile' keyword
    Endwhile,
    /// 'eval' keyword
    Eval,
    /// 'exit' keyword
    Exit,
    /// 'extends' keyword
    Extends,
    /// 'final' keyword
    Final,
    /// 'finally' keyword
    Finally,
    /// 'for' keyword
    For,
    /// 'foreach' keyword
    Foreach,
    /// 'function' keyword
    Function,
    /// 'global' keyword
    Global,
    /// 'goto' keyword
    Goto,
    /// 'if' keyword
    If,
    /// 'implements' keyword
    Implements,
    /// 'include' keyword
    Include,
    /// 'include_once' keyword
    IncludeOnce,
    /// 'instanceof' operator
    Instanceof,
    /// 'insteadof' keyword
    Insteadof,
    /// 'interface' keyword
    Interface,
    /// 'isset' keyword
    Isset,
    /// 'list' keyword
    List,
    /// 'namespace' keyword
    Namespace,
    /// 'new' keyword
    New,
    /// 'or' operator
    Or,
    /// 'print' keyword
    Print,
    /// 'private' keyword
    Private,
    /// 'protected' keyword
    Protected,
    /// 'public' keyword
    Public,
    /// 'require' keyword
    Require,
    /// 'require_once' keyword
    RequireOnce,
    /// 'return' keyword
    Return,
    /// 'static' keyword
    Static,
    /// 'switch' keyword
    Switch,
    /// 'throw' keyword
    Throw,
    /// 'trait' keyword
    Trait,
    /// 'try' keyword
    Try,
    /// 'unset' keyword
    Unset,
    /// 'use' keyword
    Use,
    /// 'var' keyword
    Var,
    /// 'while' keyword
    While,
    /// 'xor' operator
    Xor,
    /// 'yield' keyword
    Yield,
    /// 'yield from' keyword
    YieldFrom,

    /// Plus operator (+)
    Plus,
    /// Minus operator (-)
    Minus,
    /// Multiply operator (*)
    Multiply,
    /// Divide operator (/)
    Divide,
    /// Modulo operator (%)
    Modulo,
    /// Power operator (**)
    Power,
    /// Concatenation operator (.)
    Concat,
    /// Equality operator (==)
    Equal,
    /// Identity operator (===)
    Identical,
    /// Inequality operator (!= or <>)
    NotEqual,
    /// Non-identity operator (!==)
    NotIdentical,
    /// Less than operator (<)
    Less,
    /// Greater than operator (>)
    Greater,
    /// Less than or equal operator (<=)
    LessEqual,
    /// Greater than or equal operator (>=)
    GreaterEqual,
    /// Spaceship operator (<=>)
    Spaceship,
    /// Logical AND operator (&&)
    LogicalAnd,
    /// Logical OR operator (||)
    LogicalOr,
    /// Logical XOR operator
    LogicalXor,
    /// Logical NOT operator (!)
    LogicalNot,
    /// Bitwise AND operator (&)
    BitwiseAnd,
    /// Bitwise OR operator (|)
    BitwiseOr,
    /// Bitwise XOR operator (^)
    BitwiseXor,
    /// Bitwise NOT operator (~)
    BitwiseNot,
    /// Left shift operator (<<)
    LeftShift,
    /// Right shift operator (>>)
    RightShift,
    /// Assignment operator (=)
    Assign,
    /// Plus assignment operator (+=)
    PlusAssign,
    /// Minus assignment operator (-=)
    MinusAssign,
    /// Multiply assignment operator (*=)
    MultiplyAssign,
    /// Divide assignment operator (/=)
    DivideAssign,
    /// Modulo assignment operator (%=)
    ModuloAssign,
    /// Power assignment operator (**=)
    PowerAssign,
    /// Concatenation assignment operator (.=)
    ConcatAssign,
    /// Bitwise AND assignment operator (&=)
    BitwiseAndAssign,
    /// Bitwise OR assignment operator (|=)
    BitwiseOrAssign,
    /// Bitwise XOR assignment operator (^=)
    BitwiseXorAssign,
    /// Left shift assignment operator (<<=)
    LeftShiftAssign,
    /// Right shift assignment operator (>>=)
    RightShiftAssign,
    /// Increment operator (++)
    Increment,
    /// Decrement operator (--)
    Decrement,
    /// Object member access operator (->)
    Arrow,
    /// Array element arrow (=>)
    DoubleArrow,
    /// Null coalescing operator (??)
    NullCoalesce,
    /// Null coalescing assignment operator (??=)
    NullCoalesceAssign,
    /// Ellipsis operator (...)
    Ellipsis,

    /// Left parenthesis (()
    LeftParen,
    /// Right parenthesis ())
    RightParen,
    /// Left bracket ([)
    LeftBracket,
    /// Right bracket (])
    RightBracket,
    /// Left brace ({)
    LeftBrace,
    /// Right brace (})
    RightBrace,
    /// Semicolon (;)
    Semicolon,
    /// Comma (,)
    Comma,
    /// Dot operator (.)
    Dot,
    /// Question mark (?)
    Question,
    /// Colon operator (:)
    Colon,
    /// Scope resolution operator (::)
    DoubleColon,
    /// Backslash (\)
    Backslash,
    /// Error suppression operator (@)
    At,
    /// Dollar sign ($)
    Dollar,

    /// PHP opening tag (<?php)
    OpenTag,
    /// PHP closing tag (?>)
    CloseTag,
    /// PHP echo tag (<?=)
    EchoTag,

    /// End of file
    Eof,
    /// Error token
    Error,

    /// Root node of the document
    Root,
    /// Class definition
    ClassDef,
    /// Function definition
    FunctionDef,
    /// Method definition
    MethodDef,
    /// Property definition
    PropertyDef,
    /// Constant definition
    ConstDef,
    /// Trait definition
    TraitDef,
    /// Interface definition
    InterfaceDef,
    /// Namespace definition
    NamespaceDef,
    /// Use statement
    UseStatement,
    /// If statement
    IfStatement,
    /// While statement
    WhileStatement,
    /// Do-while statement
    DoWhileStatement,
    /// For statement
    ForStatement,
    /// Foreach statement
    ForeachStatement,
    /// Switch statement
    SwitchStatement,
    /// Try statement
    TryStatement,
    /// Catch block
    CatchBlock,
    /// Finally block
    FinallyBlock,
    /// Expression statement
    ExpressionStatement,
    /// Return statement
    ReturnStatement,
    /// Throw statement
    ThrowStatement,
    /// Break statement
    BreakStatement,
    /// Continue statement
    ContinueStatement,
    /// Echo statement
    EchoStatement,
    /// Global statement
    GlobalStatement,
    /// Static statement
    StaticStatement,
    /// Unset statement
    UnsetStatement,
    /// Compound statement (block)
    CompoundStatement,

    /// Literal expression
    Literal,
    /// Parenthesized expression
    ParenthesizedExpression,
    /// Function or method call
    CallExpression,
    /// Array access expression
    ArrayAccessExpression,
    /// Member access expression
    MemberAccessExpression,
    /// Binary expression
    BinaryExpression,
}
