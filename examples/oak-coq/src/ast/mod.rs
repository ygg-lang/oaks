#![doc = include_str!("readme.md")]
use oak_core::Range;

#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Root node of the Coq AST (Abstract Syntax Tree).
///
/// This structure represents the top-level node of a Coq document,
/// containing a sequence of vernacular statements.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct CoqRoot {
    /// A vector of vernacular statements that make up the Coq document.
    pub vernaculars: Vec<Vernacular>,
}

/// Represents a vernacular statement in Coq.
///
/// Vernaculars are top-level commands in Coq, such as definitions,
/// theorems, inductive types, etc.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum Vernacular {
    /// A definition statement with a name, body, and source span.
    Definition {
        /// The name of the defined term.
        name: String,
        /// The body of the definition.
        body: String,
        /// The source span of the definition.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A theorem statement with a name, statement, proof, and source span.
    Theorem {
        /// The name of the theorem.
        name: String,
        /// The statement of the theorem.
        statement: String,
        /// The proof of the theorem.
        proof: String,
        /// The source span of the theorem.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// An inductive type definition with a name, constructors, and source span.
    Inductive {
        /// The name of the inductive type.
        name: String,
        /// The constructors of the inductive type.
        constructors: Vec<String>,
        /// The source span of the inductive type definition.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A fixpoint (recursive function) definition with a name, body, and source span.
    Fixpoint {
        /// The name of the fixpoint.
        name: String,
        /// The body of the fixpoint.
        body: String,
        /// The source span of the fixpoint definition.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A Check command with a term and source span.
    Check {
        /// The term to check.
        term: String,
        /// The source span of the Check command.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A Print command with a name and source span.
    Print {
        /// The name to print.
        name: String,
        /// The source span of the Print command.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}

impl CoqRoot {
    /// Creates a new empty CoqRoot.
    ///
    /// # Returns
    ///
    /// A new CoqRoot with no vernacular statements.
    pub fn new() -> Self {
        Self { vernaculars: Vec::new() }
    }

    /// Creates a new CoqRoot with the given vernacular statements.
    ///
    /// # Arguments
    ///
    /// * `vernaculars` - A vector of vernacular statements.
    ///
    /// # Returns
    ///
    /// A new CoqRoot containing the provided vernacular statements.
    pub fn with_vernaculars(vernaculars: Vec<Vernacular>) -> Self {
        Self { vernaculars }
    }
}

impl Default for CoqRoot {
    fn default() -> Self {
        Self::new()
    }
}
