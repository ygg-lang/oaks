#![doc = include_str!("readme.md")]
use core::range::Range;

/// MATLAB Abstract Syntax Tree Root Node
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MatlabRoot {
    /// List of items in script or function
    pub items: Vec<Item>,
}

/// Top-level items in MATLAB
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Item {
    /// Function definition
    Function(Function),
    /// Class definition
    Class(Class),
    /// Statement
    Statement(Statement),
}

/// Function definition
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Function {
    /// Function name
    pub name: String,
    /// Input parameters
    pub inputs: Vec<String>,
    /// Output parameters
    pub outputs: Vec<String>,
    /// Function body
    pub body: Vec<Statement>,
    /// Source code span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Class definition
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Class {
    /// Class name
    pub name: String,
    /// Base classes
    pub superclasses: Vec<String>,
    /// Property block
    pub properties: Vec<Property>,
    /// Method block
    pub methods: Vec<Function>,
    /// Source code span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Property
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Property {
    /// Property name
    pub name: String,
    /// Default value
    pub default_value: Option<String>,
    /// Source code span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Statement
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Statement {
    /// Assignment statement
    Assignment {
        /// Target of the assignment.
        target: String,
        /// Value assigned.
        value: String,
        /// Source range.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// Expression statement
    Expression {
        /// Expression value.
        value: String,
        /// Source range.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// If statement
    If {
        /// Condition.
        condition: String,
        /// Body of the if branch.
        body: Vec<Statement>,
        /// Else-if branches.
        else_ifs: Vec<(String, Vec<Statement>)>,
        /// Else branch body.
        else_body: Option<Vec<Statement>>,
        /// Source range.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// For loop
    For {
        /// Loop variable.
        variable: String,
        /// Range of the loop.
        range: String,
        /// Body of the loop.
        body: Vec<Statement>,
        /// Source range.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// While loop
    While {
        /// Loop condition.
        condition: String,
        /// Body of the loop.
        body: Vec<Statement>,
        /// Source range.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}

/// Matlab script.
pub struct MatlabScript {
    /// Items in the script.
    pub items: Vec<Item>,
}

impl MatlabScript {
    /// Creates a new `MatlabScript` with the given items.
    pub fn new(items: Vec<Item>) -> Self {
        Self { items }
    }
}
