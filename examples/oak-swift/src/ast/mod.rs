#![doc = include_str!("readme.md")]
use core::range::Range;

/// Swift source file root node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwiftRoot {
    /// The program node containing all statements.
    pub program: Program,
    /// The source span of the root node.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Swift program
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Program {
    /// The list of statements in the program.
    pub statements: Vec<Statement>,
}

/// Statement
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Statement {
    /// Function definition
    FunctionDef {
        /// The function name.
        name: String,
        /// The function parameters.
        parameters: Vec<Parameter>,
        /// The return type of the function.
        return_type: Option<Type>,
        /// The function body statements.
        body: Vec<Statement>,
    },
    /// Variable declaration
    VariableDecl {
        /// Whether the variable is mutable.
        is_mutable: bool,
        /// The variable name.
        name: String,
        /// The type annotation of the variable.
        type_annotation: Option<Type>,
        /// The initial value of the variable.
        value: Option<Expression>,
    },
    /// Expression statement
    Expression(Expression),
    /// Return statement
    Return(Option<Expression>),
    /// Conditional statement
    If {
        /// The condition expression.
        test: Expression,
        /// The body statements when condition is true.
        body: Vec<Statement>,
        /// The else clause statements.
        orelse: Option<Vec<Statement>>,
    },
    /// While loop
    While {
        /// The loop condition expression.
        test: Expression,
        /// The loop body statements.
        body: Vec<Statement>,
    },
    /// For loop
    For {
        /// The loop variable name.
        variable: String,
        /// The iterable expression.
        iterable: Expression,
        /// The loop body statements.
        body: Vec<Statement>,
    },
    /// Block
    Block(Vec<Statement>),
}

/// Expression
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Expression {
    /// Binary operation
    Binary {
        /// The left operand.
        left: Box<Expression>,
        /// The operator.
        operator: String,
        /// The right operand.
        right: Box<Expression>,
    },
    /// Unary operation
    Unary {
        /// The operator.
        operator: String,
        /// The operand.
        operand: Box<Expression>,
    },
    /// Function call
    Call {
        /// The callee expression.
        callee: Box<Expression>,
        /// The call arguments.
        arguments: Vec<Expression>,
    },
    /// Member access
    Member {
        /// The object expression.
        object: Box<Expression>,
        /// The member name.
        member: String,
    },
    /// Identifier
    Identifier(String),
    /// Literal
    Literal(Literal),
}

/// Literal
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Literal {
    /// Numeric literal value.
    Number(String),
    /// String literal value.
    String(String),
    /// Boolean literal value.
    Boolean(bool),
    /// Nil literal value.
    Nil,
}

/// Parameter
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Parameter {
    /// The parameter name.
    pub name: String,
    /// The type annotation of the parameter.
    pub type_annotation: Type,
}

/// Type
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Type {
    /// The type name.
    pub name: String,
}
