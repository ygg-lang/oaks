#![doc = include_str!("readme.md")]

use oak_core::Range;

use serde::{Deserialize, Serialize};

/// The root node of a Coq AST (Abstract Syntax Tree).
///
/// This structure represents the top-level node of a Coq document,
/// containing a collection of vernacular statements.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CoqRoot {
    /// A vector of vernacular statements that make up the Coq document.
    pub vernaculars: Vec<Vernacular>,
}

/// Represents a vernacular statement in Coq.
///
/// A vernacular is a top-level command in Coq, such as definitions,
/// theorems, inductive types, etc.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Vernacular {
    /// A definition statement with a name, body, and source span.
    Definition {
        /// The name of the defined term.
        name: String,
        /// The body of the definition.
        body: String,
        /// The source span of the definition.
        #[serde(with = "oak_core::serde_range")]
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
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// An inductive type definition with a name, constructors, and source span.
    Inductive {
        /// The name of the inductive type.
        name: String,
        /// The constructors of the inductive type.
        constructors: Vec<String>,
        /// The source span of the inductive type.
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A fixpoint (recursive function) definition with a name, body, and source span.
    Fixpoint {
        /// The name of the fixpoint.
        name: String,
        /// The body of the fixpoint.
        body: String,
        /// The source span of the fixpoint.
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A Check command with a term and source span.
    Check {
        /// The term to check.
        term: String,
        /// The source span of the check command.
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A Print command with a name and source span.
    Print {
        /// The name to print.
        name: String,
        /// The source span of the print command.
        #[serde(with = "oak_core::serde_range")]
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
    /// * `vernaculars` - A vector of vernacular statements to include in the root.
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
