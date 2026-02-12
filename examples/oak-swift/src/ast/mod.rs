#![doc = include_str!("readme.md")]
use core::range::Range;

/// Swift source file root node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwiftRoot {
    pub program: Program,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Swift program
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Program {
    pub statements: Vec<Statement>,
}

/// Statement
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Statement {
    /// Function definition
    FunctionDef { name: String, parameters: Vec<Parameter>, return_type: Option<Type>, body: Vec<Statement> },
    /// Variable declaration
    VariableDecl { is_mutable: bool, name: String, type_annotation: Option<Type>, value: Option<Expression> },
    /// Expression statement
    Expression(Expression),
    /// Return statement
    Return(Option<Expression>),
    /// Conditional statement
    If { test: Expression, body: Vec<Statement>, orelse: Option<Vec<Statement>> },
    /// While loop
    While { test: Expression, body: Vec<Statement> },
    /// For loop
    For { variable: String, iterable: Expression, body: Vec<Statement> },
    /// Block
    Block(Vec<Statement>),
}

/// Expression
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Expression {
    /// Binary operation
    Binary { left: Box<Expression>, operator: String, right: Box<Expression> },
    /// Unary operation
    Unary { operator: String, operand: Box<Expression> },
    /// Function call
    Call { callee: Box<Expression>, arguments: Vec<Expression> },
    /// Member access
    Member { object: Box<Expression>, member: String },
    /// Identifier
    Identifier(String),
    /// Literal
    Literal(Literal),
}

/// Literal
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Literal {
    Number(String),
    String(String),
    Boolean(bool),
    Nil,
}

/// Parameter
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Parameter {
    pub name: String,
    pub type_annotation: Type,
}

/// Type
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Type {
    pub name: String,
}
