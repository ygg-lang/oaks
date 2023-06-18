use crate::{
    Language, Token,
    tree::green_tree::{GreenLeaf, GreenNode, GreenTree},
};
use triomphe::Arc;

/// Incremental cache for preserving old green trees and optional lexical caches.
///
/// This struct maintains state from previous parsing operations to enable
/// incremental reparsing when source code changes. It stores the last
/// lexed tokens and the last parsed green tree, allowing the parser to
/// reuse unchanged portions of the kind tree.
#[derive(Debug)]
pub struct IncrementalCache<'pool, L: Language> {
    /// The green builder for constructing new nodes
    pub pool: &'pool mut GreenBuilder<L>,
    /// The tokens from the previous lexical analysis
    pub last_lex: Option<Vec<Token<L::SyntaxKind>>>,
    /// The green tree from the previous parsing operation
    pub last_parse: Option<Arc<GreenNode<L::SyntaxKind>>>,
}

/// Green tree builder for constructing nodes from child elements.
///
/// This builder provides a fluent API for constructing green nodes in an
/// incremental parsing system.
#[derive(Debug, Clone)]
pub struct GreenBuilder<L: Language> {
    /// Collection of child elements (tokens and nodes) that form this green node
    children: Vec<GreenTree<L::SyntaxKind>>,
}

impl<'pool, L: Language> IncrementalCache<'pool, L> {
    /// Creates a new incremental cache with the provided green builder.
    ///
    /// # Arguments
    ///
    /// * `pool` - The green builder to use for constructing new nodes
    ///
    /// # Returns
    ///
    /// A new `IncrementalCache` with no previous tokens or parse tree
    pub fn new(pool: &'pool mut GreenBuilder<L>) -> Self {
        Self { pool, last_parse: None, last_lex: None }
    }

    /// Sets the previously lexed tokens in the cache.
    ///
    /// # Arguments
    ///
    /// * `tokens` - The tokens from a previous lexical analysis
    ///
    /// # Returns
    ///
    /// Self with the lexed tokens set
    pub fn with_lexed(mut self, tokens: Vec<Token<L::SyntaxKind>>) -> Self {
        self.last_lex = Some(tokens);
        self
    }

    /// Sets the previously parsed green tree in the cache.
    ///
    /// # Arguments
    ///
    /// * `green` - The green tree from a previous parsing operation
    ///
    /// # Returns
    ///
    /// Self with the parsed tree set
    pub fn with_parsed(mut self, green: GreenNode<L::SyntaxKind>) -> Self {
        self.last_parse = Some(Arc::new(green));
        self
    }

    /// Get a token from the cached lexical analysis by index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the token to retrieve
    ///
    /// # Returns
    ///
    /// An optional reference to the token at the specified index,
    /// or `None` if no tokens are cached or the index is out of bounds
    pub fn get_token(&self, index: usize) -> Option<&Token<L::SyntaxKind>> {
        match self.last_lex.as_ref() {
            Some(s) => s.get(index),
            None => None,
        }
    }

    /// Get the total number of tokens in the cached lexical analysis.
    ///
    /// # Returns
    ///
    /// The number of tokens in the cache, or 0 if no tokens are cached
    pub fn count_tokens(&self) -> usize {
        match self.last_lex.as_ref() {
            Some(s) => s.len(),
            None => 0,
        }
    }
}

impl<L: Language> GreenBuilder<L> {
    /// Creates a new empty green builder.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = GreenBuilder::<K>::new();
    /// ```
    pub fn new(capacity: usize) -> Self {
        Self { children: Vec::with_capacity(capacity) }
    }

    /// Adds a tokens leaf to the builder.
    ///
    /// # Arguments
    ///
    /// * `kind` - The tokens kind/type
    /// * `len` - The tokens length in bytes
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = GreenBuilder::new().tokens(K::Identifier, 5);
    /// ```
    pub fn token(mut self, kind: L::SyntaxKind, len: usize) -> Self {
        self.children.push(GreenTree::Leaf(GreenLeaf::new(kind, len)));
        self
    }

    /// Adds an existing green tree element to the builder.
    ///
    /// # Arguments
    ///
    /// * `elem` - The green tree element to add
    ///
    /// # Examples
    ///
    /// ```
    /// let child_node = some_green_tree;
    /// let builder = GreenBuilder::new().push(child_node);
    /// ```
    pub fn push(mut self, elem: GreenTree<L::SyntaxKind>) -> Self {
        self.children.push(elem);
        self
    }

    /// Finishes building and creates a new green node with the specified kind.
    ///
    /// # Arguments
    ///
    /// * `kind` - The node kind/type for the finished green node
    ///
    /// # Returns
    ///
    /// A `GreenNode<K>` containing all the accumulated children.
    ///
    /// # Examples
    ///
    /// ```
    /// let green_node = GreenBuilder::new().token(K::Number, 3).finish(K::Expression);
    /// ```
    pub fn finish(self, kind: L::SyntaxKind) -> Arc<GreenNode<L::SyntaxKind>> {
        GreenNode::new(kind, self.children)
    }
}
