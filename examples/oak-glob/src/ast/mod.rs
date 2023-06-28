use oak_core::tree::GreenNode;

/// Root node of the glob pattern AST.
pub struct GlobRoot {
    /// The underlying green node.
    pub node: oak_core::tree::GreenNode<'static, super::language::GlobLanguage>,
}

impl GlobRoot {
    /// Creates a new GlobRoot from a GreenNode.
    pub fn new(node: &oak_core::tree::GreenNode<'static, super::language::GlobLanguage>) -> Self {
        Self { node: node.clone() }
    }
}

/// Comment node in the glob pattern AST.
pub struct GlobComment<'a> {
    /// The underlying green node.
    pub node: GreenNode<'a, super::language::GlobLanguage>,
}

impl<'a> GlobComment<'a> {
    /// Creates a new GlobComment from a GreenNode.
    pub fn new(node: &'a GreenNode<'a, super::language::GlobLanguage>) -> Self {
        Self { node: node.clone() }
    }
}

/// Rule node in the glob pattern AST.
pub struct GlobRule<'a> {
    /// The underlying green node.
    pub node: GreenNode<'a, super::language::GlobLanguage>,
}

impl<'a> GlobRule<'a> {
    /// Creates a new GlobRule from a GreenNode.
    pub fn new(node: &'a GreenNode<'a, super::language::GlobLanguage>) -> Self {
        Self { node: node.clone() }
    }
}
