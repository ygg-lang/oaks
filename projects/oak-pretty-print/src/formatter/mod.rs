use crate::{Comment, CommentProcessor, FormatConfig, FormatResult, FormatTraverser, RuleSet, create_builtin_rules};

// TODO: 这些类型在 oak-core 中不存在
// use oak_core::{AstNode, AstVisitor, Language, SourceFile, SourceManager};
use crate::rules::AstNode;

/// 格式化输出
#[derive(Debug, Clone)]
pub struct FormatOutput {
    /// 格式化后的代码
    pub content: String,
    /// 是否有变化
    pub changed: bool,
    /// 应用的规则数量
    pub rules_applied: usize,
}

impl FormatOutput {
    /// 创建新的格式化输出
    pub fn new(content: String, changed: bool, rules_applied: usize) -> Self {
        Self { content, changed, rules_applied }
    }
}

/// 格式化上下文，管理格式化过程中的状态
#[derive(Debug)]
pub struct FormatContext {
    /// 格式化配置
    pub config: FormatConfig,
    /// 当前缩进级别
    pub indent_level: usize,
    /// 输出缓冲区
    pub output: String,
    /// 当前是否在新行开始
    pub at_line_start: bool,
    /// 连续空行计数
    pub blank_line_count: usize,
    /// 已应用的规则
    pub rules_applied: Vec<String>,
    /// 注释处理器
    pub comment_processor: CommentProcessor,
}

impl FormatContext {
    /// 创建新的格式化上下文
    pub fn new(config: FormatConfig) -> Self {
        Self {
            config: config.clone(),
            indent_level: 0,
            output: String::new(),
            at_line_start: true,
            blank_line_count: 0,
            rules_applied: Vec::new(),
            comment_processor: CommentProcessor::new()
                .with_preserve_comments(config.format_comments)
                .with_format_comments(config.format_comments),
        }
    }

    /// 增加缩进级别
    pub fn increase_indent(&mut self) {
        self.indent_level += 1;
    }

    /// 减少缩进级别
    pub fn decrease_indent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }

    /// 写入文本到输出缓冲区
    pub fn write(&mut self, text: &str) {
        if self.at_line_start && !text.trim().is_empty() {
            // 在行开始时添加缩进
            self.write_indent();
            self.at_line_start = false;
        }
        self.output.push_str(text);
        self.blank_line_count = 0;
    }

    /// 写入缩进
    pub fn write_indent(&mut self) {
        let indent = self.config.indent_string();
        for _ in 0..self.indent_level {
            self.output.push_str(&indent);
        }
    }

    /// 写入换行符
    pub fn write_newline(&mut self) {
        let line_ending = self.config.line_ending_string();
        self.output.push_str(line_ending);
        self.at_line_start = true;

        // 检查是否是空行
        if self.output.trim_end().ends_with(line_ending) {
            self.blank_line_count += 1;
        }
        else {
            self.blank_line_count = 0;
        }
    }

    /// 检查是否应该限制空行数量
    pub fn should_limit_blank_lines(&self) -> bool {
        self.blank_line_count >= self.config.max_blank_lines
    }

    /// 添加已应用的规则
    pub fn add_applied_rule(&mut self, rule_name: String) {
        self.rules_applied.push(rule_name);
    }

    /// 添加注释
    pub fn add_comment(&mut self, comment: Comment) {
        self.comment_processor.collector_mut().add_comment(comment);
    }

    /// 在当前位置插入注释
    pub fn insert_comments(&mut self) {
        // TODO: 使用临时的 Position 定义
        let position = crate::comment::Position { line: 0, column: self.output.len(), offset: self.output.len() };
        let indent = self.config.indent_string();
        let current_indent = indent.repeat(self.indent_level);

        self.comment_processor.insert_comments_at_position(&mut self.output, position, &current_indent);
    }

    /// 写入空行（如果允许）
    pub fn write_blank_line(&mut self) {
        if self.config.preserve_blank_lines && self.blank_line_count < self.config.max_blank_lines {
            self.write_newline();
        }
    }

    /// 确保换行
    pub fn ensure_newline(&mut self) {
        if !self.at_line_start {
            self.write_newline();
        }
    }

    /// 修剪行尾空白
    pub fn trim_trailing_whitespace(&mut self) {
        if self.config.trim_trailing_whitespace {
            let lines: Vec<&str> = self.output.lines().collect();
            let mut new_output = String::new();

            for (i, line) in lines.iter().enumerate() {
                new_output.push_str(line.trim_end());
                if i < lines.len() - 1 {
                    new_output.push_str(self.config.line_ending_string());
                }
            }

            self.output = new_output;
        }
    }

    /// 确保文件末尾有换行符
    pub fn ensure_final_newline(&mut self) {
        if self.config.insert_final_newline && !self.output.ends_with('\n') && !self.output.ends_with("\r\n") {
            self.write_newline();
        }
    }

    /// 完成格式化
    pub fn finalize(&mut self) {
        self.trim_trailing_whitespace();
        self.ensure_final_newline();
    }
}

/// 通用格式化器
pub struct Formatter {
    /// 格式化配置
    config: FormatConfig,
    /// 格式化规则集合
    rules: RuleSet,
}

impl Formatter {
    /// 创建新的格式化器
    pub fn new(config: FormatConfig) -> Self {
        let mut formatter = Self { config, rules: RuleSet::new() };

        // 添加内置规则
        for rule in create_builtin_rules() {
            let _ = formatter.add_rule(rule); // 忽略规则冲突错误
        }

        formatter
    }

    /// 添加格式化规则
    pub fn add_rule(&mut self, rule: Box<dyn crate::FormatRule>) -> FormatResult<()> {
        self.rules.add_rule(rule)
    }

    /// 格式化 AST 节点
    pub fn format_ast(&self, root: &dyn AstNode) -> FormatResult<FormatOutput> {
        let mut context = FormatContext::new(self.config.clone());
        let original_content = context.output.clone();

        // 应用格式化规则
        self.format_node(root, &mut context)?;

        // 完成格式化
        context.finalize();

        let changed = context.output != original_content;

        Ok(FormatOutput::new(context.output, changed, context.rules_applied.len()))
    }

    /// TODO: 格式化源代码字符串（需要先解析为 AST）
    // pub fn format_source<L: Language>(&self, source: &str, language: L) -> FormatResult<FormatOutput>
    // where
    //     L::SyntaxKind: crate::SyntaxKind,
    // {
    // 这里需要使用 oak-core 的解析功能
    // 由于 Language trait 的限制，这里提供一个基础实现框架

    // 1. 词法分析
    // let lexer = L::lexer();
    // let tokens = lexer
    //     .tokenize(source)
    //     .map_err(|e| FormatError::ParseError { message: format!("Lexer error: {:?}", e), position: 0 })?;

    // 2. 语法分析
    // let parser = L::parser();
    // let ast = parser
    //     .parse(&tokens)
    //     .map_err(|e| FormatError::ParseError { message: format!("Parser error: {:?}", e), position: 0 })?;

    // 3. 格式化 AST
    // 注意：这里需要 L::Node 实现 AstNode trait
    // 在实际使用中，需要为具体的语言节点类型实现 AstNode

    // 暂时返回一个占位符实现
    // Ok(FormatOutput::new(source.to_string(), false, 0))
    // }

    /// 递归格式化节点
    fn format_node(&self, node: &dyn AstNode, context: &mut FormatContext) -> FormatResult<()> {
        // 在节点前插入注释
        context.insert_comments();

        // 使用遍历器进行格式化
        let traverser = FormatTraverser::new();
        traverser.traverse(node, context)?;

        // 应用适用的格式化规则
        self.rules.apply_rules(node, context)?;
        context.add_applied_rule("format_node".to_string());

        // 在节点后插入注释
        context.insert_comments();

        Ok(())
    }
}

impl Default for Formatter {
    fn default() -> Self {
        Self::new(FormatConfig::default())
    }
}
