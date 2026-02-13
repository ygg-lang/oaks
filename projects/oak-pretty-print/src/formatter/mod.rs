use crate::{CommentProcessor, Document, FormatConfig, FormatResult, RuleSet, create_builtin_rules};
use alloc::{boxed::Box, string::String, sync::Arc, vec::Vec};
use oak_core::{
    language::Language,
    tree::{RedLeaf, RedNode, RedTree},
};

/// Formatted output
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatOutput {
    /// The formatted code string
    pub content: String,
    /// Indicates if the content was changed during formatting
    pub changed: bool,
}

impl FormatOutput {
    /// Creates a new format output
    pub fn new(content: String, changed: bool) -> Self {
        Self { content, changed }
    }
}

/// A path node for efficiently recording the formatting path
#[derive(Debug)]
pub struct PathNode<L: Language> {
    /// The element type of the node
    pub kind: L::ElementType,
    /// The parent path node
    pub parent: Option<Arc<PathNode<L>>>,
}

/// Formatting context for managing state during the formatting process
#[derive(Debug, Clone)]
pub struct FormatContext<L: Language> {
    /// Formatting configuration
    pub config: Arc<FormatConfig>,
    /// Comment processor for handling comments during formatting
    pub comment_processor: Arc<CommentProcessor>,
    /// Source code content
    pub source: Option<Arc<str>>,
    /// Current nesting depth
    pub depth: usize,
    /// Path of parent node types
    pub path: Option<Arc<PathNode<L>>>,
}

impl<L: Language> FormatContext<L> {
    /// Creates a new formatting context
    pub fn new(config: FormatConfig) -> Self {
        let config = Arc::new(config);
        let comment_processor = Arc::new(CommentProcessor::new().with_preserve_comments(config.format_comments).with_format_comments(config.format_comments));
        Self { config, comment_processor, source: None, depth: 0, path: None }
    }

    /// Enters a child node, increasing depth and recording the path
    pub fn enter(&self, kind: L::ElementType) -> Self {
        let path = Some(Arc::new(PathNode { kind, parent: self.path.clone() }));
        Self { config: self.config.clone(), comment_processor: self.comment_processor.clone(), source: self.source.clone(), depth: self.depth + 1, path }
    }

    /// Checks if the formatter is currently inside a node of a specific type
    pub fn is_inside(&self, kind: L::ElementType) -> bool {
        let mut current = self.path.as_ref();
        while let Some(node) = current {
            if node.kind == kind {
                return true;
            }
            current = node.parent.as_ref();
        }
        false
    }

    /// Gets the type of the parent node
    pub fn parent_kind(&self) -> Option<L::ElementType> {
        self.path.as_ref().and_then(|n| n.parent.as_ref()).map(|n| n.kind.clone())
    }
}

/// A generic formatter
pub struct Formatter<L: Language + 'static> {
    /// Set of formatting rules
    rules: RuleSet<L>,
    /// Initial formatting context
    pub context: FormatContext<L>,
}

impl<L: Language + 'static> Formatter<L> {
    /// Creates a new formatter
    pub fn new(config: FormatConfig) -> Self {
        let mut formatter = Self { rules: RuleSet::new(), context: FormatContext::new(config) };

        // Add built-in rules
        for rule in create_builtin_rules::<L>() {
            let _ = formatter.rules.add_rule(rule);
        }

        formatter
    }

    /// Adds a formatting rule
    pub fn add_rule(&mut self, rule: Box<dyn crate::FormatRule<L>>) -> FormatResult<()> {
        self.rules.add_rule(rule)
    }

    /// Formats an AST node
    pub fn format<'a>(&mut self, root: &RedNode<L>, source: &'a str) -> FormatResult<FormatOutput> {
        self.context.source = Some(Arc::from(source));
        let doc = self.format_node(root, &self.context, source)?;
        let content = doc.render((*self.context.config).clone());
        let changed = content != source;
        Ok(FormatOutput::new(content, changed))
    }

    /// Recursively formats a node and generates a Document
    fn format_node<'a>(&self, node: &RedNode<L>, context: &FormatContext<L>, source: &'a str) -> FormatResult<Document<'a>> {
        // Create a new context, recording current path and depth
        let new_context = context.enter(node.green.kind.clone());

        // Create a closure for formatting child nodes
        let format_children = |n: &RedNode<L>| {
            let mut children_docs = Vec::new();
            for child in n.children() {
                match child {
                    RedTree::Node(child_node) => children_docs.push(self.format_node(&child_node, &new_context, source)?),
                    RedTree::Leaf(child_token) => children_docs.push(self.format_token(&child_token, &new_context, source)?),
                }
            }
            Ok(Document::Concat(children_docs))
        };

        // Apply node rules
        if let Some(doc) = self.rules.apply_node_rules(node, &new_context, source, &format_children)? {
            return Ok(doc);
        }

        // Default logic: format all child nodes and concatenate
        format_children(node)
    }

    /// Recursively formats a Token and generates a Document
    fn format_token<'a>(&self, token: &RedLeaf<L>, context: &FormatContext<L>, source: &'a str) -> FormatResult<Document<'a>> {
        // Apply Token rules
        if let Some(doc) = self.rules.apply_token_rules(token, context, source)? {
            return Ok(doc);
        }

        // Default logic: output as is
        let text = &source[token.span.start..token.span.end];
        Ok(Document::Text(text.into()))
    }
}
