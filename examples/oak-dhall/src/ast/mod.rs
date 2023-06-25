#![doc = include_str!("readme.md")]
use core::range::Range;

/// DHall AST root node.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct DHallRoot {
    /// List of expressions in the root.
    pub expressions: Vec<DHallExpr>,
}

/// DHall expression.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum DHallExpr {
    /// Identifier.
    Identifier {
        /// Name of the identifier.
        name: String,
        /// Source range of the identifier.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// Literal.
    Literal {
        /// Value of the literal.
        value: String,
        /// Source range of the literal.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// Function application.
    Application {
        /// Function being applied.
        func: Box<DHallExpr>,
        /// Argument being applied to the function.
        arg: Box<DHallExpr>,
        /// Source range of the application.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// Lambda expression.
    Lambda {
        /// Parameter name.
        param: String,
        /// Body of the lambda.
        body: Box<DHallExpr>,
        /// Source range of the lambda.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}
