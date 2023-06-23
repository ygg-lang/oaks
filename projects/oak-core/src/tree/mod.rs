//! Red-green tree implementation for efficient kind tree representation.
//!
//! This module provides the core red-green tree data structures that enable
//! efficient incremental parsing and kind tree manipulation.
//!
//! # Key Components
//!
//! - **Green Trees**: Immutable, position-agnostic kind tree nodes allocated in an Arena.
//! - **Red Trees**: Position-aware kind tree nodes computed from green trees.
//!
//! # Architecture
//!
//! The red-green tree design enables:
//! - **Incremental Parsing**: Only re-parse changed regions of source code
//! - **Memory Efficiency**: Arena-based allocation with minimal overhead
//! - **Performance**: Zero-copy node construction and fast traversal

mod cursor;
mod green_tree;
mod metadata;
pub mod red_tree;
mod typed;

pub use self::{
    cursor::Cursor,
    green_tree::{GreenLeaf, GreenNode, GreenTree},
    metadata::{ProvenancePart, TokenProvenance},
    red_tree::{RedChildren, RedLeaf, RedNode, RedTree},
    typed::TypedNode,
};

pub use triomphe::Arc;
