#![doc = include_str!("readme.md")]
use core::range::Range;

use std::{boxed::Box, string::String, vec::Vec};

/// Strongly typed AST root for the Go language.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GoRoot {
    /// The package name declared in the source file.
    pub package: Option<String>,
    /// The list of import declarations.
    pub imports: Vec<Import>,
    /// The list of top-level declarations.
    pub declarations: Vec<Declaration>,
}

/// Represents an import declaration in Go source code.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Import {
    /// The import path as a string.
    pub path: String,
    /// An optional alias for the import.
    pub alias: Option<String>,
    /// The source span of the import declaration.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a top-level declaration in Go source code.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Declaration {
    /// A function declaration.
    Function(Function),
    /// A variable declaration.
    Variable(Variable),
    /// A type declaration.
    Type(TypeDecl),
    /// A constant declaration.
    Const(Const),
}

/// Represents a function declaration in Go source code.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Function {
    /// The function name.
    pub name: String,
    /// The optional receiver for methods.
    pub receiver: Option<Parameter>,
    /// The list of parameters.
    pub params: Vec<Parameter>,
    /// The list of return type names.
    pub return_types: Vec<String>,
    /// The function body block.
    pub body: Block,
    /// The source span of the function declaration.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a parameter in a function declaration.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Parameter {
    /// The parameter name.
    pub name: String,
    /// The parameter type name.
    pub param_type: String,
    /// The source span of the parameter.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a block of statements in Go source code.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Block {
    /// The list of statements in the block.
    pub statements: Vec<Statement>,
    /// The source span of the block.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a statement in Go source code.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Statement {
    /// An expression statement.
    Expression(Expression),
    /// An assignment statement.
    Assignment {
        /// The target variable names.
        targets: Vec<String>,
        /// The assigned values.
        values: Vec<Expression>,
        /// The source span of the assignment.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A return statement.
    Return {
        /// The values being returned.
        values: Vec<Expression>,
        /// The source span of the return statement.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// An if statement.
    If {
        /// The condition expression.
        condition: Expression,
        /// The then block.
        then_block: Block,
        /// The optional else block.
        else_block: Option<Block>,
        /// The source span of the if statement.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A for statement.
    For {
        /// The optional initialization statement.
        init: Option<Box<Statement>>,
        /// The optional condition expression.
        condition: Option<Expression>,
        /// The optional post iteration statement.
        post: Option<Box<Statement>>,
        /// The loop body block.
        body: Block,
        /// The source span of the for statement.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}

/// Represents an expression in Go source code.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Expression {
    /// An identifier expression.
    Identifier {
        /// The identifier name.
        name: String,
        /// The source span of the identifier.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A literal expression.
    Literal {
        /// The literal value as a string.
        value: String,
        /// The source span of the literal.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A binary expression.
    Binary {
        /// The left operand.
        left: Box<Expression>,
        /// The operator.
        op: String,
        /// The right operand.
        right: Box<Expression>,
        /// The source span of the binary expression.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A function call expression.
    Call {
        /// The function being called.
        func: Box<Expression>,
        /// The arguments passed to the function.
        args: Vec<Expression>,
        /// The source span of the call expression.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}

/// Represents a variable declaration in Go source code.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Variable {
    /// The variable name.
    pub name: String,
    /// The optional type annotation.
    pub var_type: Option<String>,
    /// The optional initial value.
    pub value: Option<Expression>,
    /// The source span of the variable declaration.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a type declaration in Go source code.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypeDecl {
    /// The type name.
    pub name: String,
    /// The type definition as a string.
    pub definition: String,
    /// The source span of the type declaration.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a constant declaration in Go source code.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Const {
    /// The constant name.
    pub name: String,
    /// The optional type annotation.
    pub const_type: Option<String>,
    /// The constant value.
    pub value: Expression,
    /// The source span of the constant declaration.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}
