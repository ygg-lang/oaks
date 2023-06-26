#![doc = include_str!("readme.md")]
use core::range::Range;

use crate::lexer::token_type::RTokenType;

/// An identifier in the R language.
///
/// Represents a named entity such as a variable, function, or parameter.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Identifier {
    /// The name of the identifier.
    pub name: String,
    /// The source code span of the identifier.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// The root node of the R language AST.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RRoot {
    /// The list of top-level statements in the R program.
    pub statements: Vec<Statement>,
}

/// R statement
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Statement {
    /// An assignment statement, e.g., `x <- 1`.
    Assignment {
        /// The name being assigned to.
        name: Identifier,
        /// The expression being assigned.
        expr: Expr,
        /// The source code span of the assignment statement.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// An expression statement (an expression evaluated for its side effects).
    ExprStmt {
        /// The expression being evaluated.
        expr: Expr,
        /// The source code span of the expression statement.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A function definition statement.
    FunctionDef {
        /// The name of the function.
        name: Identifier,
        /// The parameter list of the function.
        params: Vec<Identifier>,
        /// The body of the function (list of statements).
        body: Vec<Statement>,
        /// The source code span of the function definition.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}

/// R expression
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Expr {
    /// An identifier expression.
    Ident(Identifier),
    /// A literal expression, such as a string or number.
    Literal {
        /// The literal value as a string.
        value: String,
        /// The source code span of the literal.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A boolean literal expression (`TRUE` or `FALSE`).
    Bool {
        /// The boolean value.
        value: bool,
        /// The source code span of the boolean literal.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A null literal expression (`NULL`).
    Null {
        /// The source code span of the null literal.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A function call expression.
    Call {
        /// The expression being called (typically an identifier).
        callee: Box<Expr>,
        /// The list of argument expressions.
        args: Vec<Expr>,
        /// The source code span of the call expression.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A binary expression, e.g., `a + b`.
    Binary {
        /// The left-hand side expression.
        left: Box<Expr>,
        /// The operator token type.
        op: RTokenType,
        /// The right-hand side expression.
        right: Box<Expr>,
        /// The source code span of the binary expression.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A unary expression, e.g., `-x` or `!x`.
    Unary {
        /// The operator token type.
        op: RTokenType,
        /// The operand expression.
        expr: Box<Expr>,
        /// The source code span of the unary expression.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}
