#![doc = include_str!("readme.md")]
use std::range::Range;

/// Root node of the OCaml AST.
#[derive(Debug, PartialEq, Clone)]
pub struct OCamlRoot {
    /// Items in the OCaml AST.
    pub items: Vec<OCamlItem>,
}

/// A top-level item in the OCaml AST.
#[derive(Debug, PartialEq, Clone)]
pub enum OCamlItem {
    /// An expression.
    Expression(OCamlExpr),
}

/// An OCaml expression.
#[derive(Debug, PartialEq, Clone)]
pub enum OCamlExpr {
    /// An identifier.
    Identifier {
        /// The name of the identifier.
        name: String,
        /// The span of the identifier in the source code.
        span: Range<usize>,
    },
    /// A literal value.
    Literal {
        /// The value of the literal.
        value: String,
        /// The span of the literal in the source code.
        span: Range<usize>,
    },
}
