#![doc = include_str!("readme.md")]
use core::range::Range;

use std::{boxed::Box, string::String, vec::Vec};

use crate::ElixirTokenType;

/// Represents an identifier in the Elixir AST.
///
/// An identifier is a name used to refer to variables, functions, modules, or other entities.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Identifier {
    /// The name of the identifier.
    pub name: String,
    /// The source code span where this identifier appears.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// The root node of the Elixir AST.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ElixirRoot {
    /// The collection of top-level items in the Elixir source.
    pub items: Vec<Item>,
}

/// Top-level item: Module, function, or statement
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Item {
    /// A module definition containing nested items.
    Module(Module),
    /// A function definition.
    Function(Function),
    /// A standalone statement.
    Statement(Statement),
}

/// Represents an Elixir module definition.
///
/// A module is a container for functions, types, and other items.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Module {
    /// The name of the module.
    pub name: Identifier,
    /// The items contained within the module.
    pub items: Vec<Item>,
    /// The source code span where this module appears.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents an Elixir function definition.
///
/// A function consists of a name, parameters, and a body.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Function {
    /// The name of the function.
    pub name: Identifier,
    /// The parameters accepted by the function.
    pub params: Vec<Param>,
    /// The body of the function.
    pub body: Block,
    /// The source code span where this function appears.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a function parameter.
///
/// A parameter has a name and an optional type specification.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Param {
    /// The name of the parameter.
    pub name: Identifier,
    /// The optional type annotation for the parameter.
    pub ty: Option<String>,
    /// The source code span where this parameter appears.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a block of statements.
///
/// A block is a sequence of statements that are executed in order.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Block {
    /// The statements contained in this block.
    pub statements: Vec<Statement>,
    /// The source code span where this block appears.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a statement in the Elixir AST.
///
/// A statement is an executable unit of code that does not produce a value.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Statement {
    /// A variable binding statement (let binding).
    Let {
        /// The name of the variable being bound.
        name: Identifier,
        /// The expression whose value is bound to the variable.
        expr: Expr,
        /// The source code span where this statement appears.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// An expression statement.
    ExprStmt {
        /// The expression being evaluated.
        expr: Expr,
        /// The source code span where this statement appears.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}

/// Represents an expression in the Elixir AST.
///
/// An expression is a unit of code that produces a value when evaluated.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Expr {
    /// An identifier expression.
    Ident(Identifier),
    /// An atom literal.
    Atom {
        /// The value of the atom.
        value: String,
        /// The source code span where this atom appears.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A numeric literal.
    Number {
        /// The value of the number as a string.
        value: String,
        /// The source code span where this number appears.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A string literal.
    String {
        /// The value of the string.
        value: String,
        /// The source code span where this string appears.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A boolean literal.
    Bool {
        /// The boolean value.
        value: bool,
        /// The source code span where this boolean appears.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A unary expression.
    Unary {
        /// The unary operator.
        op: ElixirTokenType,
        /// The operand expression.
        expr: Box<Expr>,
        /// The source code span where this expression appears.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A binary expression.
    Binary {
        /// The left operand.
        left: Box<Expr>,
        /// The binary operator.
        op: ElixirTokenType,
        /// The right operand.
        right: Box<Expr>,
        /// The source code span where this expression appears.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A pattern match expression.
    Match {
        /// The left-hand side pattern.
        left: Box<Expr>,
        /// The right-hand side expression.
        right: Box<Expr>,
        /// The source code span where this expression appears.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A function call expression.
    Call {
        /// The expression being called.
        callee: Box<Expr>,
        /// The arguments passed to the function.
        args: Vec<Expr>,
        /// The source code span where this expression appears.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A field access expression.
    Field {
        /// The receiver expression.
        receiver: Box<Expr>,
        /// The field being accessed.
        field: Identifier,
        /// The source code span where this expression appears.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A module attribute expression.
    Attribute {
        /// The name of the attribute.
        name: Identifier,
        /// The source code span where this attribute appears.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// An index access expression.
    Index {
        /// The receiver expression.
        receiver: Box<Expr>,
        /// The index expression.
        index: Box<Expr>,
        /// The source code span where this expression appears.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A parenthesized expression.
    Paren {
        /// The inner expression.
        expr: Box<Expr>,
        /// The source code span where this expression appears.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A list literal.
    List {
        /// The elements of the list.
        items: Vec<Expr>,
        /// The source code span where this list appears.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A tuple literal.
    Tuple {
        /// The elements of the tuple.
        items: Vec<Expr>,
        /// The source code span where this tuple appears.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A map literal.
    Map {
        /// The key-value pairs of the map.
        items: Vec<Expr>,
        /// The source code span where this map appears.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A block expression.
    Block(Block),
}
