//! Red-green tree implementation for efficient kind tree representation.
//!
//! This module provides the core red-green tree data structures that enable
//! efficient incremental parsing and kind tree manipulation. The red-green
//! tree architecture separates immutable structure (green) from position-aware
//! representation (red), enabling optimal sharing and incremental updates.
//!
//! # Key Components
//!
//! - **Green Trees**: Immutable, position-agnostic kind tree nodes that can be
//!   cached and shared across different parse trees
//! - **Red Trees**: Position-aware kind tree nodes computed from green trees
//!   with absolute offsets for error reporting and diagnostics
//! - **Green Builder**: Utility for constructing green trees incrementally
//!
//! # Architecture
//!
//! The red-green tree design enables:
//! - **Incremental Parsing**: Only re-parse changed regions of source code
//! - **Memory Efficiency**: Share immutable green nodes across parse trees
//! - **Position Tracking**: Provide accurate source locations for diagnostics
//! - **Performance**: Minimize allocations through reference counting and caching

mod green_tree;
pub(crate) mod incremental;
mod red_tree;

pub use self::{
    green_tree::{GreenLeaf, GreenNode, GreenTree},
    incremental::GreenBuilder,
    red_tree::{RedChildren, RedLeaf, RedNode, RedTree},
};
