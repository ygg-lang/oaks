#![doc = include_str!("readme.md")]
use oak_core::Range;

/// Kotlin root
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KotlinRoot {
    /// The span of the root node.
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    /// The declarations in the root node.
    pub declarations: Vec<Declaration>,
}

/// A Kotlin declaration.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Declaration {
    /// A class declaration.
    Class {
        /// The name of the class.
        name: String,
        /// The members of the class.
        members: Vec<Declaration>,
        /// The span of the class declaration.
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A function declaration.
    Function {
        /// The name of the function.
        name: String,
        /// The parameters of the function.
        params: Vec<Parameter>,
        /// The body of the function.
        body: Vec<Statement>,
        /// The span of the function declaration.
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A variable declaration.
    Variable {
        /// The name of the variable.
        name: String,
        /// Whether the variable is a `val` (true) or `var` (false).
        is_val: bool,
        /// The span of the variable declaration.
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
}

/// A Kotlin parameter.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Parameter {
    /// The name of the parameter.
    pub name: String,
    /// The type of the parameter.
    pub type_name: Option<String>,
    /// The span of the parameter.
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// A Kotlin statement.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Statement {
    /// A return statement.
    Return(Option<String>),
    /// An expression statement.
    Expression(String),
    /// A variable declaration statement.
    Variable {
        /// The name of the variable.
        name: String,
        /// Whether the variable is a `val` (true) or `var` (false).
        is_val: bool,
    },
    /// An assignment statement.
    Assignment {
        /// The target of the assignment.
        target: String,
        /// The value being assigned.
        value: String,
    },
}
