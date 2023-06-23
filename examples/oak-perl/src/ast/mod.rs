#![doc = include_str!("readme.md")]
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Root node of a Perl program.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PerlRoot {
    /// Top-level items in the program.
    pub items: Vec<PerlItem>,
}

/// Top-level items in a Perl program.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PerlItem {
    /// Package declaration.
    Package(PerlPackage),
    /// Use statement.
    Use(PerlUse),
    /// Subroutine definition.
    Subroutine(PerlSubroutine),
    /// Variable declaration.
    Variable(PerlVariable),
    /// Expression statement.
    Expression(PerlExpression),
    /// Comment.
    Comment(String),
}

/// Package declaration.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PerlPackage {
    /// Package name.
    pub name: String,
}

/// Use statement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PerlUse {
    /// Module name.
    pub module: String,
    /// Optional imports.
    pub imports: Option<Vec<String>>,
}

/// Subroutine definition.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PerlSubroutine {
    /// Subroutine name.
    pub name: String,
    /// Parameter names.
    pub parameters: Vec<String>,
    /// Subroutine body statements.
    pub body: Vec<PerlStatement>,
}

/// Variable declaration.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PerlVariable {
    /// Variable scope.
    pub scope: PerlScope,
    /// Variable name.
    pub name: String,
    /// Optional initial value.
    pub value: Option<PerlExpression>,
}

/// Variable scope.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PerlScope {
    /// Lexical scope (`my`).
    My,
    /// Package scope (`our`).
    Our,
    /// Dynamic scope (`local`).
    Local,
}

/// Perl statement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PerlStatement {
    /// Expression statement.
    Expression(PerlExpression),
    /// If statement.    If(PerlIf),
    /// Loop statement.
    Loop(PerlLoop),
    /// Return statement.
    Return(Option<PerlExpression>),
    /// Control flow statement.
    Control(PerlControl),
}

/// If statement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PerlIf {
    /// Condition expression.
    pub condition: PerlExpression,
    /// `then` block statements.
    pub then_block: Vec<PerlStatement>,
    /// `elsif` blocks (condition and statements).
    pub elsif_blocks: Vec<(PerlExpression, Vec<PerlStatement>)>,
    /// `else` block statements.
    pub else_block: Option<Vec<PerlStatement>>,
}

/// Loop statement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PerlLoop {
    /// `while` loop.
    While {
        /// Loop condition.
        condition: PerlExpression,
        /// Loop body.
        body: Vec<PerlStatement>,
    },
    /// `until` loop.
    Until {
        /// Loop condition.
        condition: PerlExpression,
        /// Loop body.
        body: Vec<PerlStatement>,
    },
    /// `for` loop.
    For {
        /// Initializer.
        init: Option<PerlExpression>,
        /// Condition.
        condition: Option<PerlExpression>,
        /// Update expression.
        update: Option<PerlExpression>,
        /// Loop body.
        body: Vec<PerlStatement>,
    },
    /// `foreach` loop.
    Foreach {
        /// Iterator variable name.
        variable: String,
        /// Iterable expression.
        iterable: PerlExpression,
        /// Loop body.
        body: Vec<PerlStatement>,
    },
}

/// Control flow statement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PerlControl {
    /// Exit the innermost loop.
    Last,
    /// Start the next iteration of the loop.
    Next,
    /// Restart the current iteration of the loop.
    Redo,
}

/// Perl expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PerlExpression {
    /// Literal value.
    Literal(PerlLiteral),
    /// Variable reference.
    Variable(PerlVariableRef),
    /// Binary operation.
    Binary {
        /// Left operand.
        left: Box<PerlExpression>,
        /// Binary operator.
        operator: PerlBinaryOp,
        /// Right operand.
        right: Box<PerlExpression>,
    },
    /// Unary operation.
    Unary {
        /// Unary operator.
        operator: PerlUnaryOp,
        /// Operand.
        operand: Box<PerlExpression>,
    },
    /// Function or subroutine call.
    Call {
        /// Function name.
        function: String,
        /// Arguments.
        arguments: Vec<PerlExpression>,
    },
    /// Array access.
    ArrayAccess {
        /// Array expression.
        array: Box<PerlExpression>,
        /// Index expression.
        index: Box<PerlExpression>,
    },
    /// Hash access.
    HashAccess {
        /// Hash expression.
        hash: Box<PerlExpression>,
        /// Key expression.
        key: Box<PerlExpression>,
    },
}

/// Literal value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PerlLiteral {
    /// String literal.
    String(String),
    /// Numeric literal.
    Number(String),
    /// Array literal.
    Array(Vec<PerlExpression>),
    /// Hash literal.
    Hash(Vec<(PerlExpression, PerlExpression)>),
}

/// Variable reference.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PerlVariableRef {
    /// Variable sigil.
    pub sigil: PerlSigil,
    /// Variable name.
    pub name: String,
}

/// Variable sigil.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PerlSigil {
    /// Scalar sigil (`$`).
    Scalar,
    /// Array sigil (`@`).
    Array,
    /// Hash sigil (`%`).
    Hash,
    /// Code sigil (`&`).
    Code,
    /// Glob sigil (`*`).
    Glob,
}

/// Binary operator.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PerlBinaryOp {
    // Arithmetic operators
    /// Addition.
    Add,
    /// Subtraction.
    Subtract,
    /// Multiplication.
    Multiply,
    /// Division.
    Divide,
    /// Modulo.
    Modulo,
    /// Power.
    Power,

    // String operators
    /// Concatenation.
    Concat,
    /// Repetition.
    Repeat,

    // Comparison operators
    /// Equality.
    Equal,
    /// Inequality.
    NotEqual,
    /// Less than.
    LessThan,
    /// Less than or equal to.
    LessEqual,
    /// Greater than.
    GreaterThan,
    /// Greater than or equal to.
    GreaterEqual,
    /// Spaceship operator.
    Spaceship,

    // Logical operators
    /// Logical AND.
    LogicalAnd,
    /// Logical OR.
    LogicalOr,

    // Bitwise operators
    /// Bitwise AND.
    BitwiseAnd,
    /// Bitwise OR.
    BitwiseOr,
    /// Bitwise XOR.
    BitwiseXor,
    /// Left shift.
    LeftShift,
    /// Right shift.
    RightShift,

    // 赋值操作符
    Assign,

    // 模式匹配
    Match,
    NotMatch,
}

/// 一元操作符
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PerlUnaryOp {
    Plus,
    Minus,
    LogicalNot,
    BitwiseNot,
    Increment,
    Decrement,
    Reference,
    Dereference,
}
