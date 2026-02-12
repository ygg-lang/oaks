#![doc = include_str!("readme.md")]
use core::range::Range;

/// Scala document root node.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScalaRoot {
    /// The top-level declarations in the Scala file.
    pub declarations: Vec<Declaration>,
    /// The span of the root node in the source file.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// A Scala declaration.
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
    /// An object declaration.
    Object {
        /// The name of the object.
        name: String,
        /// The members of the object.
        members: Vec<Declaration>,
        /// The span of the object declaration.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A trait declaration.
    Trait {
        /// The name of the trait.
        name: String,
        /// The members of the trait.
        members: Vec<Declaration>,
        /// The span of the trait declaration.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A function/method declaration.
    Def {
        /// The name of the function.
        name: String,
        /// The span of the function declaration.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A variable declaration.
    Val {
        /// The name of the variable.
        name: String,
        /// The span of the variable declaration.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}

impl ScalaRoot {
    /// Creates a new Scala root node with the given span.
    pub fn new(span: Range<usize>) -> Self {
        Self { declarations: vec![], span }
    }
}

impl Default for ScalaRoot {
    fn default() -> Self {
        Self { declarations: vec![], span: Range { start: 0, end: 0 } }
    }
}
