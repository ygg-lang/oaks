use oak_core::{ElementType, UniversalElementRole};

/// Element type for PHP AST
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PhpElementType {
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
    /// Error node
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

impl ElementType for PhpElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::PhpTokenType> for PhpElementType {
    fn from(token: crate::lexer::token_type::PhpTokenType) -> Self {
        match token {
            // Whitespace and newlines
            crate::lexer::token_type::PhpTokenType::Whitespace => PhpElementType::Whitespace,
            crate::lexer::token_type::PhpTokenType::Newline => PhpElementType::Newline,

            // Comments
            crate::lexer::token_type::PhpTokenType::Comment => PhpElementType::Comment,

            // Literals
            crate::lexer::token_type::PhpTokenType::StringLiteral => PhpElementType::StringLiteral,
            crate::lexer::token_type::PhpTokenType::NumberLiteral => PhpElementType::NumberLiteral,
            crate::lexer::token_type::PhpTokenType::BooleanLiteral => PhpElementType::BooleanLiteral,
            crate::lexer::token_type::PhpTokenType::NullLiteral => PhpElementType::NullLiteral,

            // Identifiers and keywords
            crate::lexer::token_type::PhpTokenType::Identifier => PhpElementType::Identifier,
            crate::lexer::token_type::PhpTokenType::Variable => PhpElementType::Variable,
            crate::lexer::token_type::PhpTokenType::Abstract => PhpElementType::Abstract,
            crate::lexer::token_type::PhpTokenType::And => PhpElementType::And,
            crate::lexer::token_type::PhpTokenType::Array => PhpElementType::Array,
            crate::lexer::token_type::PhpTokenType::As => PhpElementType::As,
            crate::lexer::token_type::PhpTokenType::Break => PhpElementType::Break,
            crate::lexer::token_type::PhpTokenType::Callable => PhpElementType::Callable,
            crate::lexer::token_type::PhpTokenType::Case => PhpElementType::Case,
            crate::lexer::token_type::PhpTokenType::Catch => PhpElementType::Catch,
            crate::lexer::token_type::PhpTokenType::Class => PhpElementType::Class,
            crate::lexer::token_type::PhpTokenType::Clone => PhpElementType::Clone,
            crate::lexer::token_type::PhpTokenType::Const => PhpElementType::Const,
            crate::lexer::token_type::PhpTokenType::Continue => PhpElementType::Continue,
            crate::lexer::token_type::PhpTokenType::Declare => PhpElementType::Declare,
            crate::lexer::token_type::PhpTokenType::Default => PhpElementType::Default,
            crate::lexer::token_type::PhpTokenType::Do => PhpElementType::Do,
            crate::lexer::token_type::PhpTokenType::Echo => PhpElementType::Echo,
            crate::lexer::token_type::PhpTokenType::Else => PhpElementType::Else,
            crate::lexer::token_type::PhpTokenType::Elseif => PhpElementType::Elseif,
            crate::lexer::token_type::PhpTokenType::Empty => PhpElementType::Empty,
            crate::lexer::token_type::PhpTokenType::Enddeclare => PhpElementType::Enddeclare,
            crate::lexer::token_type::PhpTokenType::Endfor => PhpElementType::Endfor,
            crate::lexer::token_type::PhpTokenType::Endforeach => PhpElementType::Endforeach,
            crate::lexer::token_type::PhpTokenType::Endif => PhpElementType::Endif,
            crate::lexer::token_type::PhpTokenType::Endswitch => PhpElementType::Endswitch,
            crate::lexer::token_type::PhpTokenType::Endwhile => PhpElementType::Endwhile,
            crate::lexer::token_type::PhpTokenType::Eval => PhpElementType::Eval,
            crate::lexer::token_type::PhpTokenType::Exit => PhpElementType::Exit,
            crate::lexer::token_type::PhpTokenType::Extends => PhpElementType::Extends,
            crate::lexer::token_type::PhpTokenType::Final => PhpElementType::Final,
            crate::lexer::token_type::PhpTokenType::Finally => PhpElementType::Finally,
            crate::lexer::token_type::PhpTokenType::For => PhpElementType::For,
            crate::lexer::token_type::PhpTokenType::Foreach => PhpElementType::Foreach,
            crate::lexer::token_type::PhpTokenType::Function => PhpElementType::Function,
            crate::lexer::token_type::PhpTokenType::Global => PhpElementType::Global,
            crate::lexer::token_type::PhpTokenType::Goto => PhpElementType::Goto,
            crate::lexer::token_type::PhpTokenType::If => PhpElementType::If,
            crate::lexer::token_type::PhpTokenType::Implements => PhpElementType::Implements,
            crate::lexer::token_type::PhpTokenType::Include => PhpElementType::Include,
            crate::lexer::token_type::PhpTokenType::IncludeOnce => PhpElementType::IncludeOnce,
            crate::lexer::token_type::PhpTokenType::Instanceof => PhpElementType::Instanceof,
            crate::lexer::token_type::PhpTokenType::Insteadof => PhpElementType::Insteadof,
            crate::lexer::token_type::PhpTokenType::Interface => PhpElementType::Interface,
            crate::lexer::token_type::PhpTokenType::Isset => PhpElementType::Isset,
            crate::lexer::token_type::PhpTokenType::List => PhpElementType::List,
            crate::lexer::token_type::PhpTokenType::Namespace => PhpElementType::Namespace,
            crate::lexer::token_type::PhpTokenType::New => PhpElementType::New,
            crate::lexer::token_type::PhpTokenType::Or => PhpElementType::Or,
            crate::lexer::token_type::PhpTokenType::Print => PhpElementType::Print,
            crate::lexer::token_type::PhpTokenType::Private => PhpElementType::Private,
            crate::lexer::token_type::PhpTokenType::Protected => PhpElementType::Protected,
            crate::lexer::token_type::PhpTokenType::Public => PhpElementType::Public,
            crate::lexer::token_type::PhpTokenType::Require => PhpElementType::Require,
            crate::lexer::token_type::PhpTokenType::RequireOnce => PhpElementType::RequireOnce,
            crate::lexer::token_type::PhpTokenType::Return => PhpElementType::Return,
            crate::lexer::token_type::PhpTokenType::Static => PhpElementType::Static,
            crate::lexer::token_type::PhpTokenType::Switch => PhpElementType::Switch,
            crate::lexer::token_type::PhpTokenType::Throw => PhpElementType::Throw,
            crate::lexer::token_type::PhpTokenType::Trait => PhpElementType::Trait,
            crate::lexer::token_type::PhpTokenType::Try => PhpElementType::Try,
            crate::lexer::token_type::PhpTokenType::Unset => PhpElementType::Unset,
            crate::lexer::token_type::PhpTokenType::Use => PhpElementType::Use,
            crate::lexer::token_type::PhpTokenType::Var => PhpElementType::Var,
            crate::lexer::token_type::PhpTokenType::While => PhpElementType::While,
            crate::lexer::token_type::PhpTokenType::Xor => PhpElementType::Xor,
            crate::lexer::token_type::PhpTokenType::Yield => PhpElementType::Yield,
            crate::lexer::token_type::PhpTokenType::YieldFrom => PhpElementType::YieldFrom,

            // Operators
            crate::lexer::token_type::PhpTokenType::Plus => PhpElementType::Plus,
            crate::lexer::token_type::PhpTokenType::Minus => PhpElementType::Minus,
            crate::lexer::token_type::PhpTokenType::Multiply => PhpElementType::Multiply,
            crate::lexer::token_type::PhpTokenType::Divide => PhpElementType::Divide,
            crate::lexer::token_type::PhpTokenType::Modulo => PhpElementType::Modulo,
            crate::lexer::token_type::PhpTokenType::Power => PhpElementType::Power,
            crate::lexer::token_type::PhpTokenType::Concat => PhpElementType::Concat,
            crate::lexer::token_type::PhpTokenType::Equal => PhpElementType::Equal,
            crate::lexer::token_type::PhpTokenType::Identical => PhpElementType::Identical,
            crate::lexer::token_type::PhpTokenType::NotEqual => PhpElementType::NotEqual,
            crate::lexer::token_type::PhpTokenType::NotIdentical => PhpElementType::NotIdentical,
            crate::lexer::token_type::PhpTokenType::Less => PhpElementType::Less,
            crate::lexer::token_type::PhpTokenType::Greater => PhpElementType::Greater,
            crate::lexer::token_type::PhpTokenType::LessEqual => PhpElementType::LessEqual,
            crate::lexer::token_type::PhpTokenType::GreaterEqual => PhpElementType::GreaterEqual,
            crate::lexer::token_type::PhpTokenType::Spaceship => PhpElementType::Spaceship,
            crate::lexer::token_type::PhpTokenType::LogicalAnd => PhpElementType::LogicalAnd,
            crate::lexer::token_type::PhpTokenType::LogicalOr => PhpElementType::LogicalOr,
            crate::lexer::token_type::PhpTokenType::LogicalXor => PhpElementType::LogicalXor,
            crate::lexer::token_type::PhpTokenType::LogicalNot => PhpElementType::LogicalNot,
            crate::lexer::token_type::PhpTokenType::BitwiseAnd => PhpElementType::BitwiseAnd,
            crate::lexer::token_type::PhpTokenType::BitwiseOr => PhpElementType::BitwiseOr,
            crate::lexer::token_type::PhpTokenType::BitwiseXor => PhpElementType::BitwiseXor,
            crate::lexer::token_type::PhpTokenType::BitwiseNot => PhpElementType::BitwiseNot,
            crate::lexer::token_type::PhpTokenType::LeftShift => PhpElementType::LeftShift,
            crate::lexer::token_type::PhpTokenType::RightShift => PhpElementType::RightShift,
            crate::lexer::token_type::PhpTokenType::Assign => PhpElementType::Assign,
            crate::lexer::token_type::PhpTokenType::PlusAssign => PhpElementType::PlusAssign,
            crate::lexer::token_type::PhpTokenType::MinusAssign => PhpElementType::MinusAssign,
            crate::lexer::token_type::PhpTokenType::MultiplyAssign => PhpElementType::MultiplyAssign,
            crate::lexer::token_type::PhpTokenType::DivideAssign => PhpElementType::DivideAssign,
            crate::lexer::token_type::PhpTokenType::ModuloAssign => PhpElementType::ModuloAssign,
            crate::lexer::token_type::PhpTokenType::PowerAssign => PhpElementType::PowerAssign,
            crate::lexer::token_type::PhpTokenType::ConcatAssign => PhpElementType::ConcatAssign,
            crate::lexer::token_type::PhpTokenType::BitwiseAndAssign => PhpElementType::BitwiseAndAssign,
            crate::lexer::token_type::PhpTokenType::BitwiseOrAssign => PhpElementType::BitwiseOrAssign,
            crate::lexer::token_type::PhpTokenType::BitwiseXorAssign => PhpElementType::BitwiseXorAssign,
            crate::lexer::token_type::PhpTokenType::LeftShiftAssign => PhpElementType::LeftShiftAssign,
            crate::lexer::token_type::PhpTokenType::RightShiftAssign => PhpElementType::RightShiftAssign,
            crate::lexer::token_type::PhpTokenType::Increment => PhpElementType::Increment,
            crate::lexer::token_type::PhpTokenType::Decrement => PhpElementType::Decrement,
            crate::lexer::token_type::PhpTokenType::Arrow => PhpElementType::Arrow,
            crate::lexer::token_type::PhpTokenType::DoubleArrow => PhpElementType::DoubleArrow,
            crate::lexer::token_type::PhpTokenType::NullCoalesce => PhpElementType::NullCoalesce,
            crate::lexer::token_type::PhpTokenType::NullCoalesceAssign => PhpElementType::NullCoalesceAssign,
            crate::lexer::token_type::PhpTokenType::Ellipsis => PhpElementType::Ellipsis,

            // Punctuations
            crate::lexer::token_type::PhpTokenType::LeftParen => PhpElementType::LeftParen,
            crate::lexer::token_type::PhpTokenType::RightParen => PhpElementType::RightParen,
            crate::lexer::token_type::PhpTokenType::LeftBracket => PhpElementType::LeftBracket,
            crate::lexer::token_type::PhpTokenType::RightBracket => PhpElementType::RightBracket,
            crate::lexer::token_type::PhpTokenType::LeftBrace => PhpElementType::LeftBrace,
            crate::lexer::token_type::PhpTokenType::RightBrace => PhpElementType::RightBrace,
            crate::lexer::token_type::PhpTokenType::Semicolon => PhpElementType::Semicolon,
            crate::lexer::token_type::PhpTokenType::Comma => PhpElementType::Comma,
            crate::lexer::token_type::PhpTokenType::Dot => PhpElementType::Dot,
            crate::lexer::token_type::PhpTokenType::Question => PhpElementType::Question,
            crate::lexer::token_type::PhpTokenType::Colon => PhpElementType::Colon,
            crate::lexer::token_type::PhpTokenType::DoubleColon => PhpElementType::DoubleColon,
            crate::lexer::token_type::PhpTokenType::Backslash => PhpElementType::Backslash,
            crate::lexer::token_type::PhpTokenType::At => PhpElementType::At,
            crate::lexer::token_type::PhpTokenType::Dollar => PhpElementType::Dollar,

            // PHP special tags
            crate::lexer::token_type::PhpTokenType::OpenTag => PhpElementType::OpenTag,
            crate::lexer::token_type::PhpTokenType::CloseTag => PhpElementType::CloseTag,
            crate::lexer::token_type::PhpTokenType::EchoTag => PhpElementType::EchoTag,

            // Special
            crate::lexer::token_type::PhpTokenType::Eof => PhpElementType::Eof,
            crate::lexer::token_type::PhpTokenType::Error => PhpElementType::Error,

            // Element types
            crate::lexer::token_type::PhpTokenType::Root => PhpElementType::Root,
            crate::lexer::token_type::PhpTokenType::ClassDef => PhpElementType::ClassDef,
            crate::lexer::token_type::PhpTokenType::FunctionDef => PhpElementType::FunctionDef,
            crate::lexer::token_type::PhpTokenType::MethodDef => PhpElementType::MethodDef,
            crate::lexer::token_type::PhpTokenType::PropertyDef => PhpElementType::PropertyDef,
            crate::lexer::token_type::PhpTokenType::ConstDef => PhpElementType::ConstDef,
            crate::lexer::token_type::PhpTokenType::TraitDef => PhpElementType::TraitDef,
            crate::lexer::token_type::PhpTokenType::InterfaceDef => PhpElementType::InterfaceDef,
            crate::lexer::token_type::PhpTokenType::NamespaceDef => PhpElementType::NamespaceDef,
            crate::lexer::token_type::PhpTokenType::UseStatement => PhpElementType::UseStatement,
            crate::lexer::token_type::PhpTokenType::IfStatement => PhpElementType::IfStatement,
            crate::lexer::token_type::PhpTokenType::WhileStatement => PhpElementType::WhileStatement,
            crate::lexer::token_type::PhpTokenType::DoWhileStatement => PhpElementType::DoWhileStatement,
            crate::lexer::token_type::PhpTokenType::ForStatement => PhpElementType::ForStatement,
            crate::lexer::token_type::PhpTokenType::ForeachStatement => PhpElementType::ForeachStatement,
            crate::lexer::token_type::PhpTokenType::SwitchStatement => PhpElementType::SwitchStatement,
            crate::lexer::token_type::PhpTokenType::TryStatement => PhpElementType::TryStatement,
            crate::lexer::token_type::PhpTokenType::CatchBlock => PhpElementType::CatchBlock,
            crate::lexer::token_type::PhpTokenType::FinallyBlock => PhpElementType::FinallyBlock,
            crate::lexer::token_type::PhpTokenType::ExpressionStatement => PhpElementType::ExpressionStatement,
            crate::lexer::token_type::PhpTokenType::ReturnStatement => PhpElementType::ReturnStatement,
            crate::lexer::token_type::PhpTokenType::ThrowStatement => PhpElementType::ThrowStatement,
            crate::lexer::token_type::PhpTokenType::BreakStatement => PhpElementType::BreakStatement,
            crate::lexer::token_type::PhpTokenType::ContinueStatement => PhpElementType::ContinueStatement,
            crate::lexer::token_type::PhpTokenType::EchoStatement => PhpElementType::EchoStatement,
            crate::lexer::token_type::PhpTokenType::GlobalStatement => PhpElementType::GlobalStatement,
            crate::lexer::token_type::PhpTokenType::StaticStatement => PhpElementType::StaticStatement,
            crate::lexer::token_type::PhpTokenType::UnsetStatement => PhpElementType::UnsetStatement,
            crate::lexer::token_type::PhpTokenType::CompoundStatement => PhpElementType::CompoundStatement,

            // Expressions
            crate::lexer::token_type::PhpTokenType::Literal => PhpElementType::Literal,
            crate::lexer::token_type::PhpTokenType::ParenthesizedExpression => PhpElementType::ParenthesizedExpression,
            crate::lexer::token_type::PhpTokenType::CallExpression => PhpElementType::CallExpression,
            crate::lexer::token_type::PhpTokenType::ArrayAccessExpression => PhpElementType::ArrayAccessExpression,
            crate::lexer::token_type::PhpTokenType::MemberAccessExpression => PhpElementType::MemberAccessExpression,
            crate::lexer::token_type::PhpTokenType::BinaryExpression => PhpElementType::BinaryExpression,
        }
    }
}
