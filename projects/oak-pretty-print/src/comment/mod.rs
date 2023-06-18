/// 临时的位置类型
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

/// 临时的源码范围类型
#[derive(Debug, Clone, PartialEq)]
pub struct SourceSpan {
    pub start: Position,
    pub end: Position,
}

/// 注释类型
#[derive(Debug, Clone, PartialEq)]
pub enum CommentKind {
    /// 行注释 (如 // 或 #)
    Line,
    /// 块注释 (如 /* */ 或 """ """)
    Block,
    /// 文档注释 (如 /// 或 /** */)
    Doc,
}

/// 注释信息
#[derive(Debug, Clone, PartialEq)]
pub struct Comment {
    /// 注释类型
    pub kind: CommentKind,
    /// 注释内容（不包含注释标记）
    pub content: String,
    /// 注释在源码中的位置
    pub span: SourceSpan,
    /// 是否在行尾
    pub is_trailing: bool,
    /// 缩进级别
    pub indent_level: usize,
}

impl Comment {
    pub fn new(kind: CommentKind, content: String, span: SourceSpan) -> Self {
        Self { kind, content, span, is_trailing: false, indent_level: 0 }
    }

    pub fn line(content: String, span: SourceSpan) -> Self {
        Self::new(CommentKind::Line, content, span)
    }

    pub fn block(content: String, span: SourceSpan) -> Self {
        Self::new(CommentKind::Block, content, span)
    }

    pub fn doc(content: String, span: SourceSpan) -> Self {
        Self::new(CommentKind::Doc, content, span)
    }

    pub fn with_trailing(mut self, is_trailing: bool) -> Self {
        self.is_trailing = is_trailing;
        self
    }

    pub fn with_indent_level(mut self, level: usize) -> Self {
        self.indent_level = level;
        self
    }

    /// 获取格式化后的注释文本
    pub fn formatted_text(&self, indent: &str) -> String {
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
                        // 多行块注释
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
                        // 单行块注释
                        format!("/* {} */", self.content.trim())
                    }
                }
            }
        }
    }
}

/// 注释收集器，用于从源码中提取注释
#[derive(Debug, Clone)]
pub struct CommentCollector {
    comments: Vec<Comment>,
}

impl CommentCollector {
    pub fn new() -> Self {
        Self { comments: Vec::new() }
    }

    pub fn add_comment(&mut self, comment: Comment) {
        self.comments.push(comment);
    }

    pub fn comments(&self) -> &[Comment] {
        &self.comments
    }

    pub fn comments_mut(&mut self) -> &mut Vec<Comment> {
        &mut self.comments
    }

    /// 获取指定位置范围内的注释
    pub fn comments_in_range(&self, start: Position, end: Position) -> Vec<&Comment> {
        self.comments
            .iter()
            .filter(|comment| comment.span.start.offset >= start.offset && comment.span.end.offset <= end.offset)
            .collect()
    }

    /// 获取指定位置之前的注释
    pub fn comments_before(&self, position: Position) -> Vec<&Comment> {
        self.comments.iter().filter(|comment| comment.span.end.offset <= position.offset).collect()
    }

    /// 获取指定位置之后的注释
    pub fn comments_after(&self, position: Position) -> Vec<&Comment> {
        self.comments.iter().filter(|comment| comment.span.start.offset >= position.offset).collect()
    }

    /// 获取行尾注释
    pub fn trailing_comments(&self) -> Vec<&Comment> {
        self.comments.iter().filter(|comment| comment.is_trailing).collect()
    }

    /// 清空所有注释
    pub fn clear(&mut self) {
        self.comments.clear();
    }

    /// 按位置排序注释
    pub fn sort_by_position(&mut self) {
        self.comments.sort_by(|a, b| a.span.start.offset.cmp(&b.span.start.offset));
    }
}

impl Default for CommentCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// 注释处理器，负责在格式化过程中处理注释
#[derive(Debug)]
pub struct CommentProcessor {
    collector: CommentCollector,
    preserve_comments: bool,
    format_comments: bool,
}

impl CommentProcessor {
    pub fn new() -> Self {
        Self { collector: CommentCollector::new(), preserve_comments: true, format_comments: true }
    }

    pub fn with_preserve_comments(mut self, preserve: bool) -> Self {
        self.preserve_comments = preserve;
        self
    }

    pub fn with_format_comments(mut self, format: bool) -> Self {
        self.format_comments = format;
        self
    }

    pub fn collector(&self) -> &CommentCollector {
        &self.collector
    }

    pub fn collector_mut(&mut self) -> &mut CommentCollector {
        &mut self.collector
    }

    /// 处理注释，返回格式化后的注释文本
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
                    // 保持原始格式
                    comment.content.clone()
                }
            })
            .collect()
    }

    /// 在指定位置插入注释
    pub fn insert_comments_at_position(&self, output: &mut String, position: Position, indent: &str) {
        let comments = self.collector.comments_before(position);
        for comment in comments {
            if !comment.is_trailing {
                output.push_str(indent);
                output.push_str(&comment.formatted_text(indent));
                output.push('\n');
            }
        }
    }

    /// 插入行尾注释
    pub fn insert_trailing_comment(&self, output: &mut String, position: Position) {
        let comments = self.collector.comments_after(position);
        for comment in comments {
            if comment.is_trailing {
                output.push(' ');
                output.push_str(&comment.formatted_text(""));
                break; // 只处理第一个行尾注释
            }
        }
    }
}

impl Default for CommentProcessor {
    fn default() -> Self {
        Self::new()
    }
}
