use crate::{Document, FormatContext, FormatResult};
use alloc::{boxed::Box, vec::Vec};
use oak_core::{
    errors::OakError,
    language::Language,
    tree::{RedLeaf, RedNode},
};

/// 格式化规则 trait
pub trait FormatRule<L: Language> {
    /// 规则名称
    fn name(&self) -> &str;

    /// 规则优先级（数字越大优先级越高）
    fn priority(&self) -> u8 {
        0
    }

    /// 检查规则是否适用于给定节点
    fn applies_to_node(&self, _node: &RedNode<L>) -> bool {
        false
    }

    /// 检查规则是否适用于给定 Token
    fn applies_to_token(&self, _token: &RedLeaf<L>) -> bool {
        false
    }

    /// 应用格式化规则到节点，返回可选的 Document
    fn apply_node<'a>(&self, node: &RedNode<L>, context: &FormatContext<L>, source: &'a str, format_children: &dyn Fn(&RedNode<L>) -> FormatResult<Document<'a>>) -> FormatResult<Option<Document<'a>>>;

    /// 应用格式化规则到 Token，返回可选的 Document
    fn apply_token<'a>(&self, token: &RedLeaf<L>, context: &FormatContext<L>, source: &'a str) -> FormatResult<Option<Document<'a>>>;

    /// 规则是否与其他规则冲突
    fn conflicts_with(&self, _other: &dyn FormatRule<L>) -> bool {
        false
    }
}

/// 规则集合
pub struct RuleSet<L: Language> {
    rules: Vec<Box<dyn FormatRule<L>>>,
}

impl<L: Language> Default for RuleSet<L> {
    fn default() -> Self {
        Self { rules: Vec::new() }
    }
}

impl<L: Language> RuleSet<L> {
    /// 创建新的规则集合
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加规则
    pub fn add_rule(&mut self, rule: Box<dyn FormatRule<L>>) -> FormatResult<()> {
        // 检查规则冲突
        for existing_rule in &self.rules {
            if rule.conflicts_with(existing_rule.as_ref()) || existing_rule.conflicts_with(rule.as_ref()) {
                return Err(OakError::format_error(format!("Rule conflict between '{}' and '{}'", existing_rule.name(), rule.name())));
            }
        }

        self.rules.push(rule);

        // 按优先级排序
        self.rules.sort_by(|a, b| b.priority().cmp(&a.priority()));

        Ok(())
    }

    /// 批量添加规则
    pub fn add_rules(&mut self, rules: Vec<Box<dyn FormatRule<L>>>) -> FormatResult<()> {
        for rule in rules {
            self.add_rule(rule)?;
        }
        Ok(())
    }

    /// 获取适用于节点的规则
    pub fn applicable_rules_for_node<'a>(&'a self, node: &'a RedNode<L>) -> impl Iterator<Item = &'a dyn FormatRule<L>> + 'a {
        self.rules.iter().filter(move |rule| rule.applies_to_node(node)).map(|rule| rule.as_ref())
    }

    /// 获取适用于 Token 的规则
    pub fn applicable_rules_for_token<'a>(&'a self, token: &'a RedLeaf<L>) -> impl Iterator<Item = &'a dyn FormatRule<L>> + 'a {
        self.rules.iter().filter(move |rule| rule.applies_to_token(token)).map(|rule| rule.as_ref())
    }

    /// 应用所有适用的节点规则，返回第一个成功的 Document
    pub fn apply_node_rules<'a>(&self, node: &RedNode<L>, context: &FormatContext<L>, source: &'a str, format_children: &dyn Fn(&RedNode<L>) -> FormatResult<Document<'a>>) -> FormatResult<Option<Document<'a>>> {
        for rule in self.applicable_rules_for_node(node) {
            if let Some(doc) = rule.apply_node(node, context, source, format_children)? {
                return Ok(Some(doc));
            }
        }
        Ok(None)
    }

    /// 应用所有适用的 Token 规则，返回第一个成功的 Document
    pub fn apply_token_rules<'a>(&self, token: &RedLeaf<L>, context: &FormatContext<L>, source: &'a str) -> FormatResult<Option<Document<'a>>> {
        for rule in self.applicable_rules_for_token(token) {
            if let Some(doc) = rule.apply_token(token, context, source)? {
                return Ok(Some(doc));
            }
        }
        Ok(None)
    }
}
