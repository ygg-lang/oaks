/// Temporary position type
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    /// Line number (0-indexed)
    pub line: usize,
    /// Column number (0-indexed)
    pub column: usize,
    /// Byte offset in the source string
    pub offset: usize,
}

/// Temporary source span type
#[derive(Debug, Clone, PartialEq)]
pub struct SourceSpan {
    /// Start position
    pub start: Position,
    /// End position
    pub end: Position,
}

/// Comment type
#[derive(Debug, Clone, PartialEq)]
pub enum CommentKind {
    /// Line comment (e.g., // or #)
    Line,
    /// Block comment (e.g., /* */ or """ """)
    Block,
    /// Documentation comment (e.g., /// or /** */)
    Doc,
}

/// Comment information
#[derive(Debug, Clone, PartialEq)]
pub struct Comment {
    /// Comment type
    pub kind: CommentKind,
    /// Comment content (excluding markers)
    pub content: String,
    /// Comment position in the source code
    pub span: SourceSpan,
    /// Whether it is at the end of a line
    pub is_trailing: bool,
    /// Indentation level
    pub indent_level: usize,
    /// Whether the comment is attached to a node
    pub is_attached: bool,
    /// The node type the comment is attached to
    pub attached_node_type: Option<String>,
    /// Whether the comment is a leading comment
    pub is_leading: bool,
    /// Whether the comment is a block comment that should be preserved as-is
    pub preserve_format: bool,
}

impl Comment {
    /// Creates a new comment
    pub fn new(kind: CommentKind, content: String, span: SourceSpan) -> Self {
        Self { kind, content, span, is_trailing: false, indent_level: 0, is_attached: false, attached_node_type: None, is_leading: false, preserve_format: false }
    }

    /// Creates a new line comment
    pub fn line(content: String, span: SourceSpan) -> Self {
        Self::new(CommentKind::Line, content, span)
    }

    /// Creates a new block comment
    pub fn block(content: String, span: SourceSpan) -> Self {
        Self::new(CommentKind::Block, content, span)
    }

    /// Creates a new doc comment
    pub fn doc(content: String, span: SourceSpan) -> Self {
        Self::new(CommentKind::Doc, content, span)
    }

    /// Sets whether the comment is trailing
    pub fn with_trailing(mut self, is_trailing: bool) -> Self {
        self.is_trailing = is_trailing;
        self
    }

    /// Sets the indentation level of the comment
    pub fn with_indent_level(mut self, level: usize) -> Self {
        self.indent_level = level;
        self
    }

    /// Sets whether the comment is attached to a node
    pub fn with_attached(mut self, is_attached: bool) -> Self {
        self.is_attached = is_attached;
        self
    }

    /// Sets the node type the comment is attached to
    pub fn with_attached_node_type(mut self, node_type: Option<String>) -> Self {
        self.attached_node_type = node_type;
        self
    }

    /// Sets whether the comment is a leading comment
    pub fn with_leading(mut self, is_leading: bool) -> Self {
        self.is_leading = is_leading;
        self
    }

    /// Sets whether the comment's format should be preserved
    pub fn with_preserve_format(mut self, preserve: bool) -> Self {
        self.preserve_format = preserve;
        self
    }

    /// Gets the formatted comment text
    pub fn formatted_text(&self, indent: &str) -> String {
        // If preserve_format is true, return the original content
        if self.preserve_format {
            return self.content.clone();
        }

        let prefix = match self.kind {
            CommentKind::Line => "//",
            CommentKind::Block => "/*",
            CommentKind::Doc => "///",
        };

        let suffix = match self.kind {
            CommentKind::Block => " */",
            _ => "",
        };

        if self.content.trim().is_empty() {
            format!("{}{}{}", prefix, self.content, suffix)
        }
        else {
            match self.kind {
                CommentKind::Line | CommentKind::Doc => {
                    format!("{} {}", prefix, self.content.trim())
                }
                CommentKind::Block => {
                    if self.content.contains('\n') {
                        // Multi-line block comment
                        let lines: Vec<&str> = self.content.lines().collect();
                        let mut result = String::new();
                        result.push_str("/*\n");
                        for line in lines {
                            result.push_str(indent);
                            result.push_str(" * ");
                            result.push_str(line.trim());
                            result.push('\n');
                        }
                        result.push_str(indent);
                        result.push_str(" */");
                        result
                    }
                    else {
                        // Single-line block comment
                        format!("/* {} */", self.content.trim())
                    }
                }
            }
        }
    }
}

/// Comment collector, used to extract comments from source code
#[derive(Debug, Clone, PartialEq)]
pub struct CommentCollector {
    comments: Vec<Comment>,
}

impl CommentCollector {
    /// Creates a new comment collector
    pub fn new() -> Self {
        Self { comments: Vec::new() }
    }

    /// Adds a comment to the collector
    pub fn add_comment(&mut self, comment: Comment) {
        self.comments.push(comment)
    }

    /// Gets the collected comments
    pub fn comments(&self) -> &[Comment] {
        &self.comments
    }

    /// Gets the collected comments mutably
    pub fn comments_mut(&mut self) -> &mut Vec<Comment> {
        &mut self.comments
    }

    /// Gets comments within a specified position range
    pub fn comments_in_range(&self, start: Position, end: Position) -> Vec<&Comment> {
        self.comments.iter().filter(|comment| comment.span.start.offset >= start.offset && comment.span.end.offset <= end.offset).collect()
    }

    /// Gets comments before a specified position
    pub fn comments_before(&self, position: Position) -> Vec<&Comment> {
        self.comments.iter().filter(|comment| comment.span.end.offset <= position.offset).collect()
    }

    /// Gets comments after a specified position
    pub fn comments_after(&self, position: Position) -> Vec<&Comment> {
        self.comments.iter().filter(|comment| comment.span.start.offset >= position.offset).collect()
    }

    /// Gets trailing comments
    pub fn trailing_comments(&self) -> Vec<&Comment> {
        self.comments.iter().filter(|comment| comment.is_trailing).collect()
    }

    /// Gets leading comments
    pub fn leading_comments(&self) -> Vec<&Comment> {
        self.comments.iter().filter(|comment| comment.is_leading).collect()
    }

    /// Gets attached comments
    pub fn attached_comments(&self) -> Vec<&Comment> {
        self.comments.iter().filter(|comment| comment.is_attached).collect()
    }

    /// Gets comments by kind
    pub fn comments_by_kind(&self, kind: CommentKind) -> Vec<&Comment> {
        self.comments.iter().filter(|comment| comment.kind == kind).collect()
    }

    /// Gets comments attached to a specific node type
    pub fn comments_attached_to(&self, node_type: &str) -> Vec<&Comment> {
        self.comments.iter().filter(|comment| comment.attached_node_type.as_ref().map(|s| s.as_str()) == Some(node_type)).collect()
    }

    /// Clears all comments
    pub fn clear(&mut self) {
        self.comments.clear()
    }

    /// Sorts comments by position
    pub fn sort_by_position(&mut self) {
        self.comments.sort_by(|a, b| a.span.start.offset.cmp(&b.span.start.offset))
    }

    /// Sorts comments by line and column
    pub fn sort_by_line_column(&mut self) {
        self.comments.sort_by(|a, b| if a.span.start.line == b.span.start.line { a.span.start.column.cmp(&b.span.start.column) } else { a.span.start.line.cmp(&b.span.start.line) })
    }

    /// Groups comments by line
    pub fn group_by_line(&self) -> std::collections::HashMap<usize, Vec<&Comment>> {
        let mut groups = std::collections::HashMap::new();
        for comment in &self.comments {
            groups.entry(comment.span.start.line).or_insert_with(Vec::new).push(comment);
        }
        groups
    }
}

impl Default for CommentCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Comment processor, responsible for handling comments during formatting
#[derive(Debug, Clone)]
pub struct CommentProcessor {
    collector: CommentCollector,
    preserve_comments: bool,
    format_comments: bool,
}

impl CommentProcessor {
    /// Creates a new comment processor
    pub fn new() -> Self {
        Self { collector: CommentCollector::new(), preserve_comments: true, format_comments: true }
    }

    /// Sets whether to preserve comments
    pub fn with_preserve_comments(mut self, preserve: bool) -> Self {
        self.preserve_comments = preserve;
        self
    }

    /// Sets whether to format comments
    pub fn with_format_comments(mut self, format: bool) -> Self {
        self.format_comments = format;
        self
    }

    /// Gets the comment collector
    pub fn collector(&self) -> &CommentCollector {
        &self.collector
    }

    /// Gets the comment collector mutably
    pub fn collector_mut(&mut self) -> &mut CommentCollector {
        &mut self.collector
    }

    /// Processes comments and returns the formatted comment text
    pub fn process_comments(&self, comments: &[Comment], indent: &str) -> Vec<String> {
        if !self.preserve_comments {
            return Vec::new();
        }

        comments
            .iter()
            .map(|comment| {
                if self.format_comments {
                    comment.formatted_text(indent)
                }
                else {
                    // Preserve original format
                    comment.content.clone()
                }
            })
            .collect()
    }

    /// Inserts comments at the specified position
    pub fn insert_comments_at_position(&self, output: &mut String, position: Position, indent: &str) {
        let comments = self.collector.comments_before(position);
        for comment in comments {
            if !comment.is_trailing {
                output.push_str(indent);
                output.push_str(&comment.formatted_text(indent));
                output.push('\n')
            }
        }
    }

    /// Inserts a trailing comment
    pub fn insert_trailing_comment(&self, output: &mut String, position: Position) {
        let comments = self.collector.comments_after(position);
        for comment in comments {
            if comment.is_trailing {
                output.push(' ');
                output.push_str(&comment.formatted_text(""));
                break; // Only process the first trailing comment
            }
        }
    }

    /// Inserts leading comments for a node
    pub fn insert_leading_comments(&self, output: &mut String, node_type: &str, indent: &str) {
        let comments = self.collector.comments_attached_to(node_type);
        for comment in comments {
            if comment.is_leading {
                output.push_str(indent);
                output.push_str(&comment.formatted_text(indent));
                output.push('\n');
            }
        }
    }

    /// Inserts attached comments for a node
    pub fn insert_attached_comments(&self, output: &mut String, node_type: &str, indent: &str) {
        let comments = self.collector.comments_attached_to(node_type);
        for comment in comments {
            if comment.is_attached && !comment.is_leading && !comment.is_trailing {
                output.push_str(indent);
                output.push_str(&comment.formatted_text(indent));
                output.push('\n');
            }
        }
    }

    /// Groups comments by their position relative to nodes
    pub fn group_comments_by_position(&mut self) {
        // Sort comments by position
        self.collector.sort_by_position();

        // TODO: Implement grouping logic based on node positions
        // This would require access to the AST nodes
    }

    /// Detects and marks comment types based on context
    pub fn detect_comment_types(&mut self) {
        // Sort comments by line and column
        self.collector.sort_by_line_column();

        // Group comments by line
        let line_groups = self.collector.group_by_line();

        // Process each line's comments
        for (line, comments) in line_groups {
            // If there's only one comment on the line, check if it's trailing
            if comments.len() == 1 {
                let comment = &comments[0];
                // TODO: Check if the comment is at the end of a line with code
                // This would require access to the source code
            }
        }
    }
}

impl Default for CommentProcessor {
    fn default() -> Self {
        Self::new()
    }
}
