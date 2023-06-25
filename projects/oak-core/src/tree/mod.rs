#![doc = include_str!("readme.md")]

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
