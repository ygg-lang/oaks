#![doc = include_str!("readme.md")]
use oak_core::Range;

/// Kotlin root
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KotlinRoot {
    /// The span of the root node.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
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
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A data class declaration.
    DataClass {
        /// The name of the data class.
        name: String,
        /// The parameters of the data class.
        params: Vec<Parameter>,
        /// The members of the data class.
        members: Vec<Declaration>,
        /// The span of the data class declaration.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A sealed class declaration.
    SealedClass {
        /// The name of the sealed class.
        name: String,
        /// The members of the sealed class.
        members: Vec<Declaration>,
        /// The span of the sealed class declaration.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
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
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// An extension function declaration.
    ExtensionFunction {
        /// The receiver type of the extension function.
        receiver_type: String,
        /// The name of the extension function.
        name: String,
        /// The parameters of the extension function.
        params: Vec<Parameter>,
        /// The body of the extension function.
        body: Vec<Statement>,
        /// The span of the extension function declaration.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A variable declaration.
    Variable {
        /// The name of the variable.
        name: String,
        /// Whether the variable is a `val` (true) or `var` (false).
        is_val: bool,
        /// The span of the variable declaration.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
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
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
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
