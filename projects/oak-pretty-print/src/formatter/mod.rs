use crate::{CommentProcessor, Doc, FormatConfig, FormatResult, RuleSet, create_builtin_rules};
use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};
use oak_core::{
    language::Language,
    tree::{RedLeaf, RedNode, RedTree},
};

/// 格式化输出
#[derive(Debug, Clone)]
pub struct FormatOutput {
    /// 格式化后的代码
    pub content: String,
    /// 是否有变化
    pub changed: bool,
}

impl FormatOutput {
    /// 创建新的格式化输出
    pub fn new(content: String, changed: bool) -> Self {
        Self { content, changed }
    }
}

/// 格式化上下文，管理格式化过程中的状态
#[derive(Debug, Clone)]
pub struct FormatContext<L: Language> {
    /// 格式化配置
    pub config: FormatConfig,
    /// 注释处理器
    pub comment_processor: CommentProcessor,
    /// 源码内容
    pub source: Option<String>,
    /// 当前嵌套深度
    pub depth: usize,
    /// 父节点类型路径
    pub path: Vec<L::ElementType>,
}

impl<L: Language> FormatContext<L> {
    /// 创建新的格式化上下文
    pub fn new(config: FormatConfig) -> Self {
        Self { config: config.clone(), comment_processor: CommentProcessor::new().with_preserve_comments(config.format_comments).with_format_comments(config.format_comments), source: None, depth: 0, path: Vec::new() }
    }

    /// 进入子节点，增加深度并记录路径
    pub fn enter(&self, kind: L::ElementType) -> Self {
        let mut path = self.path.clone();
        path.push(kind);
        Self { config: self.config.clone(), comment_processor: self.comment_processor.clone(), source: self.source.clone(), depth: self.depth + 1, path }
    }

    /// 检查是否处于特定类型的节点内部
    pub fn is_inside(&self, kind: L::ElementType) -> bool {
        self.path.contains(&kind)
    }

    /// 获取父节点类型
    pub fn parent_kind(&self) -> Option<&L::ElementType> {
        self.path.last()
    }
}

/// 通用格式化器
pub struct Formatter<L: Language + 'static> {
    /// 格式化配置
    config: FormatConfig,
    /// 格式化规则集合
    rules: RuleSet<L>,
    /// 初始格式化上下文
    pub context: FormatContext<L>,
}

impl<L: Language + 'static> Formatter<L>
where
    L::ElementType: oak_core::language::TokenType,
{
    /// 创建新的格式化器
    pub fn new(config: FormatConfig) -> Self {
        let mut formatter = Self { config: config.clone(), rules: RuleSet::new(), context: FormatContext::new(config) };

        // 添加内置规则
        for rule in create_builtin_rules::<L>() {
            let _ = formatter.rules.add_rule(rule);
        }

        formatter
    }

    /// 添加格式化规则
    pub fn add_rule(&mut self, rule: Box<dyn crate::FormatRule<L>>) -> FormatResult<()> {
        self.rules.add_rule(rule)
    }

    /// 格式化 AST 节点
    pub fn format(&mut self, root: &RedNode<L>, source: &str) -> FormatResult<FormatOutput> {
        self.context.source = Some(source.to_string());
        let doc = self.format_node(root, &self.context)?;
        let content = doc.render(self.config.clone());
        Ok(FormatOutput::new(content, true))
    }

    /// 递归格式化节点并生成 Document
    fn format_node(&self, node: &RedNode<L>, context: &FormatContext<L>) -> FormatResult<Doc> {
        // 创建一个新的上下文，记录当前路径和深度
        let new_context = context.enter(node.green.kind.clone());

        // 创建一个用于格式化子节点的闭包
        let format_children = |n: &RedNode<L>| {
            let mut children_docs = Vec::new();
            for child in n.children() {
                match child {
                    RedTree::Node(child_node) => children_docs.push(self.format_node(&child_node, &new_context)?),
                    RedTree::Leaf(child_token) => children_docs.push(self.format_token(&child_token, &new_context)?),
                }
            }
            Ok(Doc::Concat(children_docs))
        };

        // 应用节点规则
        if let Some(doc) = self.rules.apply_node_rules(node, &new_context, &format_children)? {
            return Ok(doc);
        }

        // 默认逻辑：格式化所有子节点并连接
        format_children(node)
    }

    /// 递归格式化 Token 并生成 Document
    fn format_token(&self, token: &RedLeaf<L>, context: &FormatContext<L>) -> FormatResult<Doc> {
        // 应用 Token 规则
        if let Some(doc) = self.rules.apply_token_rules(token, context)? {
            return Ok(doc);
        }

        // 默认逻辑：原样输出
        if let Some(source) = &context.source {
            let text = &source[token.span.start..token.span.end];
            Ok(Doc::Text(text.to_string()))
        }
        else {
            Ok(Doc::Nil)
        }
    }
}
