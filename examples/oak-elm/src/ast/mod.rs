#![doc = include_str!("readme.md")]
use core::range::Range;
use std::{boxed::Box, string::String, vec::Vec};

use crate::ElmTokenType;

/// An identifier in the Elm source code.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Identifier {
    /// The name of the identifier.
    pub name: String,
    /// The source range of the identifier.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Root node of the Elm AST.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ElmRoot {
    /// Top-level items in the module.
    pub items: Vec<Item>,
}

/// A top-level item in an Elm module.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Item {
    /// A module declaration.
    Module(Module),
    /// A function declaration.
    Function(Function),
    /// A statement.
    Statement(Statement),
}

/// An Elm module declaration.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Module {
    /// The name of the module.
    pub name: Identifier,
    /// The items contained in the module.
    pub items: Vec<Item>,
    /// The source range of the module.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// An Elm function declaration.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Function {
    /// The name of the function.
    pub name: Identifier,
    /// The parameters of the function.
    pub params: Vec<Param>,
    /// The body of the function.
    pub body: Block,
    /// The source range of the function.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// An Elm function parameter.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Param {
    /// The name of the parameter.
    pub name: Identifier,
    /// The type of the parameter, if specified.
    pub ty: Option<String>,
    /// The source range of the parameter.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// An Elm code block.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Block {
    /// The statements in the block.
    pub statements: Vec<Statement>,
    /// The source range of the block.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// An Elm statement.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Statement {
    /// A `let` statement.
    Let {
        /// The name of the binding.
        name: Identifier,
        /// The expression to bind.
        expr: Expr,
        /// The source range of the statement.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// An expression statement.
    ExprStmt {
        /// The expression in the statement.
        expr: Expr,
        /// The source range of the statement.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}

/// An Elm expression.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Expr {
    /// An identifier expression.
    Ident(Identifier),
    /// An atomic expression.
    Atom {
        /// The value of the atom.
        value: String,
        /// The source range of the expression.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A number literal expression.
    Number {
        /// The value of the number.
        value: String,
        /// The source range of the expression.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A string literal expression.
    String {
        /// The value of the string.
        value: String,
        /// The source range of the expression.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A boolean literal expression.
    Bool {
        /// The value of the boolean.
        value: bool,
        /// The source range of the expression.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A binary operation expression.
    Binary {
        /// The left-hand side of the operation.
        lhs: Box<Expr>,
        /// The operator token.
        op: ElmTokenType,
        /// The right-hand side of the operation.
        rhs: Box<Expr>,
        /// The source range of the expression.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A unary operation expression.
    Unary {
        /// The operator token.
        op: ElmTokenType,
        /// The expression to operate on.
        expr: Box<Expr>,
        /// The source range of the expression.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A function call expression.
    Call {
        /// The function being called.
        callee: Box<Expr>,
        /// The arguments to the function.
        args: Vec<Expr>,
        /// The source range of the expression.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A field access expression.
    Field {
        /// The receiver of the field access.
        receiver: Box<Expr>,
        /// The name of the field.
        field: Identifier,
        /// The source range of the expression.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// An index access expression.
    Index {
        /// The receiver of the index access.
        receiver: Box<Expr>,
        /// The index expression.
        index: Box<Expr>,
        /// The source range of the expression.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A parenthesized expression.
    Paren {
        /// The inner expression.
        expr: Box<Expr>,
        /// The source range of the expression.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A block expression.
    Block(Block),
    /// An `if` expression.
    If {
        /// The condition.
        cond: Box<Expr>,
        /// The `then` branch.
        then: Box<Expr>,
        /// The `else` branch.
        els: Box<Expr>,
        /// The source range of the expression.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}
