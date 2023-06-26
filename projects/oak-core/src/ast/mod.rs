//! Unified AST processing interface for Oaks core framework.
//!
//! This module provides a unified interface for processing ASTs in Oaks,
//! including traversal, modification, and querying methods.

use crate::{Language, RedLeaf, RedNode, RedTree};

/// A unified interface for processing ASTs.
///
/// This trait provides a common set of methods for working with ASTs,
/// regardless of the specific language implementation.
pub trait AstProcessor<'a, L: Language> {
    /// Traverses the AST in pre-order, applying the given visitor to each node.
    fn traverse_pre_order(&self, root: RedTree<'a, L>, visitor: &mut impl Visitor<'a, L>);

    /// Traverses the AST in post-order, applying the given visitor to each node.
    fn traverse_post_order(&self, root: RedTree<'a, L>, visitor: &mut impl Visitor<'a, L>);

    /// Finds all nodes of the specified type.
    fn find_nodes_by_type(&self, root: RedTree<'a, L>, node_type: L::ElementType) -> Vec<RedNode<'a, L>>;

    /// Finds all tokens of the specified type.
    fn find_tokens_by_type(&self, root: RedTree<'a, L>, token_type: L::TokenType) -> Vec<RedLeaf<L>>;

    /// Finds the node at the specified offset.
    fn find_node_at_offset(&self, root: RedTree<'a, L>, offset: usize) -> Option<RedTree<'a, L>>;

    /// Replaces a node with a new one.
    ///
    /// Note: This operation may require rebuilding parts of the tree.
    fn replace_node(&self, old_node: RedNode<'a, L>, new_node: RedNode<'a, L>) -> Result<RedTree<'a, L>, AstError>;

    /// Removes a node from the tree.
    fn remove_node(&self, node: RedNode<'a, L>) -> Result<RedTree<'a, L>, AstError>;

    /// Adds a child node to the specified parent.
    fn add_child(&self, parent: RedNode<'a, L>, child: RedTree<'a, L>) -> Result<RedNode<'a, L>, AstError>;
}

/// A visitor for AST nodes.
///
/// This trait extends the existing Visitor trait with additional methods
/// for more flexible traversal and modification.
pub trait Visitor<'a, L: Language> {
    /// Visits a node before its children.
    fn visit_node_pre(&mut self, node: RedNode<'a, L>) -> VisitResult;

    /// Visits a node after its children.
    fn visit_node_post(&mut self, node: RedNode<'a, L>) -> VisitResult;

    /// Visits a token.
    fn visit_token(&mut self, token: RedLeaf<L>) -> VisitResult;

    /// Default implementation that just continues traversal.
    fn visit_node_pre_default(&mut self, _node: RedNode<'a, L>) -> VisitResult {
        VisitResult::Continue
    }

    /// Default implementation that just continues traversal.
    fn visit_node_post_default(&mut self, _node: RedNode<'a, L>) -> VisitResult {
        VisitResult::Continue
    }

    /// Default implementation that just continues traversal.
    fn visit_token_default(&mut self, _token: RedLeaf<L>) -> VisitResult {
        VisitResult::Continue
    }
}

/// Result of a visit operation.
pub enum VisitResult {
    /// Continue traversal.
    Continue,
    /// Skip the children of the current node.
    SkipChildren,
    /// Stop traversal entirely.
    Stop,
}

/// Error type for AST operations.
pub enum AstError {
    /// The operation failed because the node was not found.
    NodeNotFound,
    /// The operation failed because the node is not modifiable.
    NodeNotModifiable,
    /// The operation failed because of an invalid operation.
    InvalidOperation,
    /// The operation failed because of a language-specific error.
    LanguageSpecific(String),
}

impl std::fmt::Debug for AstError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AstError::NodeNotFound => write!(f, "Node not found"),
            AstError::NodeNotModifiable => write!(f, "Node not modifiable"),
            AstError::InvalidOperation => write!(f, "Invalid operation"),
            AstError::LanguageSpecific(msg) => write!(f, "Language-specific error: {}", msg),
        }
    }
}

impl std::fmt::Display for AstError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for AstError {}

/// Default implementation of AstProcessor.
pub struct DefaultAstProcessor;

impl<'a, L: Language> AstProcessor<'a, L> for DefaultAstProcessor {
    fn traverse_pre_order(&self, root: RedTree<'a, L>, visitor: &mut impl Visitor<'a, L>) {
        self.traverse_pre_order_impl(root, visitor)
    }

    fn traverse_post_order(&self, root: RedTree<'a, L>, visitor: &mut impl Visitor<'a, L>) {
        self.traverse_post_order_impl(root, visitor)
    }

    fn find_nodes_by_type(&self, root: RedTree<'a, L>, node_type: L::ElementType) -> Vec<RedNode<'a, L>> {
        let mut result = Vec::new();
        let mut visitor = TypeFinderVisitor { node_type, result: &mut result };
        self.traverse_pre_order(root, &mut visitor);
        result
    }

    fn find_tokens_by_type(&self, root: RedTree<'a, L>, token_type: L::TokenType) -> Vec<RedLeaf<L>> {
        let mut result = Vec::new();
        let mut visitor = TokenTypeFinderVisitor { token_type, result: &mut result };
        self.traverse_pre_order(root, &mut visitor);
        result
    }

    fn find_node_at_offset(&self, root: RedTree<'a, L>, offset: usize) -> Option<RedTree<'a, L>> {
        match root {
            RedTree::Node(node) => {
                if offset >= node.span().start && offset < node.span().end {
                    node.child_at_offset(offset)
                }
                else {
                    None
                }
            }
            RedTree::Leaf(leaf) => {
                if offset >= leaf.span.start && offset < leaf.span.end {
                    Some(RedTree::Leaf(leaf))
                }
                else {
                    None
                }
            }
        }
    }

    fn replace_node(&self, _old_node: RedNode<'a, L>, _new_node: RedNode<'a, L>) -> Result<RedTree<'a, L>, AstError> {
        // TODO: Implement node replacement
        Err(AstError::NodeNotModifiable)
    }

    fn remove_node(&self, _node: RedNode<'a, L>) -> Result<RedTree<'a, L>, AstError> {
        // TODO: Implement node removal
        Err(AstError::NodeNotModifiable)
    }

    fn add_child(&self, _parent: RedNode<'a, L>, _child: RedTree<'a, L>) -> Result<RedNode<'a, L>, AstError> {
        // TODO: Implement child addition
        Err(AstError::NodeNotModifiable)
    }
}

impl DefaultAstProcessor {
    /// Creates a new DefaultAstProcessor.
    pub fn new() -> Self {
        Self
    }

    /// Implementation of pre-order traversal.
    fn traverse_pre_order_impl<'a, L: Language>(&self, node: RedTree<'a, L>, visitor: &mut impl Visitor<'a, L>) {
        match node {
            RedTree::Node(n) => match visitor.visit_node_pre(n) {
                VisitResult::Continue => {
                    for child in n.children() {
                        self.traverse_pre_order_impl(child, visitor);
                    }
                    visitor.visit_node_post(n);
                }
                VisitResult::SkipChildren => {
                    visitor.visit_node_post(n);
                }
                VisitResult::Stop => return,
            },
            RedTree::Leaf(t) => {
                if let VisitResult::Stop = visitor.visit_token(t) {
                    return;
                }
            }
        }
    }

    /// Implementation of post-order traversal.
    fn traverse_post_order_impl<'a, L: Language>(&self, node: RedTree<'a, L>, visitor: &mut impl Visitor<'a, L>) {
        match node {
            RedTree::Node(n) => {
                if let VisitResult::Continue = visitor.visit_node_pre(n) {
                    for child in n.children() {
                        self.traverse_post_order_impl(child, visitor);
                    }
                }
                if let VisitResult::Stop = visitor.visit_node_post(n) {
                    return;
                }
            }
            RedTree::Leaf(t) => {
                if let VisitResult::Stop = visitor.visit_token(t) {
                    return;
                }
            }
        }
    }
}

/// A visitor that finds nodes of a specific type.
struct TypeFinderVisitor<'b, 'a, L: Language> {
    node_type: L::ElementType,
    result: &'b mut Vec<RedNode<'a, L>>,
}

impl<'b, 'a, L: Language> Visitor<'a, L> for TypeFinderVisitor<'b, 'a, L> {
    fn visit_node_pre(&mut self, node: RedNode<'a, L>) -> VisitResult {
        if node.element_type() == self.node_type {
            self.result.push(node);
        }
        VisitResult::Continue
    }

    fn visit_node_post(&mut self, _node: RedNode<'a, L>) -> VisitResult {
        VisitResult::Continue
    }

    fn visit_token(&mut self, _token: RedLeaf<L>) -> VisitResult {
        VisitResult::Continue
    }
}

/// A visitor that finds tokens of a specific type.
struct TokenTypeFinderVisitor<'b, L: Language> {
    token_type: L::TokenType,
    result: &'b mut Vec<RedLeaf<L>>,
}

impl<'b, 'a, L: Language> Visitor<'a, L> for TokenTypeFinderVisitor<'b, L> {
    fn visit_node_pre(&mut self, _node: RedNode<'a, L>) -> VisitResult {
        VisitResult::Continue
    }

    fn visit_node_post(&mut self, _node: RedNode<'a, L>) -> VisitResult {
        VisitResult::Continue
    }

    fn visit_token(&mut self, token: RedLeaf<L>) -> VisitResult {
        if token.kind == self.token_type {
            self.result.push(token);
        }
        VisitResult::Continue
    }
}

/// Extension methods for RedTree to make it easier to work with the AST processor.
pub trait RedTreeExt<'a, L: Language> {
    /// Traverses the tree in pre-order.
    fn traverse_pre_order(&self, visitor: &mut impl Visitor<'a, L>);

    /// Traverses the tree in post-order.
    fn traverse_post_order(&self, visitor: &mut impl Visitor<'a, L>);

    /// Finds all nodes of the specified type.
    fn find_nodes_by_type(&self, node_type: L::ElementType) -> Vec<RedNode<'a, L>>;

    /// Finds all tokens of the specified type.
    fn find_tokens_by_type(&self, token_type: L::TokenType) -> Vec<RedLeaf<L>>;

    /// Finds the node at the specified offset.
    fn find_node_at_offset(&self, offset: usize) -> Option<RedTree<'a, L>>;
}

impl<'a, L: Language> RedTreeExt<'a, L> for RedTree<'a, L> {
    fn traverse_pre_order(&self, visitor: &mut impl Visitor<'a, L>) {
        let processor = DefaultAstProcessor::new();
        processor.traverse_pre_order(*self, visitor);
    }

    fn traverse_post_order(&self, visitor: &mut impl Visitor<'a, L>) {
        let processor = DefaultAstProcessor::new();
        processor.traverse_post_order(*self, visitor);
    }

    fn find_nodes_by_type(&self, node_type: L::ElementType) -> Vec<RedNode<'a, L>> {
        let processor = DefaultAstProcessor::new();
        processor.find_nodes_by_type(*self, node_type)
    }

    fn find_tokens_by_type(&self, token_type: L::TokenType) -> Vec<RedLeaf<L>> {
        let processor = DefaultAstProcessor::new();
        processor.find_tokens_by_type(*self, token_type)
    }

    fn find_node_at_offset(&self, offset: usize) -> Option<RedTree<'a, L>> {
        let processor = DefaultAstProcessor::new();
        processor.find_node_at_offset(*self, offset)
    }
}
