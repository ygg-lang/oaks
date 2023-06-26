use std::boxed::Box;

use super::ExprNode;

/// Dimension specification.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Dimension {
    /// Explicit shape with lower and upper bounds.
    Explicit(Box<ExprNode>, Box<ExprNode>),
    /// Assumed shape with optional lower bound.
    Assumed(Option<Box<ExprNode>>),
    /// Deferred shape.
    Deferred,
    /// Assumed size with optional lower bound.
    AssumedSize(Option<Box<ExprNode>>),
    /// Assumed rank.
    AssumedRank,
}
