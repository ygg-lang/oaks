use crate::{FormatContext, FormatError, FormatResult};
use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};
// TODO: 这些类型在 oak-core 中不存在，需要实现
// use oak_core::{AstNode, AstVisitor};

/// 临时的 AST 节点 trait，用于编译
pub trait AstNode {
    fn as_any(&self) -> &dyn core::any::Any;
}

/// 格式化规则 trait
pub trait FormatRule {
    /// 规则名称
    fn name(&self) -> &str;

    /// 规则优先级（数字越大优先级越高）
    fn priority(&self) -> u8 {
        0
    }

    /// 检查规则是否适用于给定节点
    fn applies_to(&self, node: &dyn AstNode) -> bool;

    /// 应用格式化规则
    fn apply(&self, node: &dyn AstNode, context: &mut FormatContext) -> FormatResult<()>;

    /// 规则是否与其他规则冲突
    fn conflicts_with(&self, _other: &dyn FormatRule) -> bool {
        false
    }
}

/// 规则集合
#[derive(Default)]
pub struct RuleSet {
    rules: Vec<Box<dyn FormatRule>>,
}

impl RuleSet {
    /// 创建新的规则集合
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// 添加规则
    pub fn add_rule(&mut self, rule: Box<dyn FormatRule>) -> FormatResult<()> {
        // 检查规则冲突
        for existing_rule in &self.rules {
            if rule.conflicts_with(existing_rule.as_ref()) || existing_rule.conflicts_with(rule.as_ref()) {
                return Err(FormatError::RuleConflict {
                    rule1: existing_rule.name().to_string(),
                    rule2: rule.name().to_string(),
                });
            }
        }

        self.rules.push(rule);

        // 按优先级排序
        self.rules.sort_by(|a, b| b.priority().cmp(&a.priority()));

        Ok(())
    }

    /// 移除规则
    pub fn remove_rule(&mut self, name: &str) -> bool {
        if let Some(pos) = self.rules.iter().position(|r| r.name() == name) {
            self.rules.remove(pos);
            true
        }
        else {
            false
        }
    }

    /// 获取适用于节点的规则
    pub fn applicable_rules(&self, node: &dyn AstNode) -> Vec<&dyn FormatRule> {
        self.rules.iter().filter(|rule| rule.applies_to(node)).map(|rule| rule.as_ref()).collect()
    }

    /// 应用所有适用的规则
    pub fn apply_rules(&self, node: &dyn AstNode, context: &mut FormatContext) -> FormatResult<()> {
        let applicable_rules = self.applicable_rules(node);

        for rule in applicable_rules {
            rule.apply(node, context)?;
        }

        Ok(())
    }

    /// 获取规则数量
    pub fn len(&self) -> usize {
        self.rules.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}

/// 基础格式化规则实现
pub struct BasicFormatRule {
    name: String,
    priority: u8,
    apply_fn: Box<dyn Fn(&dyn AstNode, &mut FormatContext) -> FormatResult<()>>,
    applies_fn: Box<dyn Fn(&dyn AstNode) -> bool>,
}

impl BasicFormatRule {
    /// 创建新的基础格式化规则
    pub fn new<F, A>(name: String, apply_fn: F, applies_fn: A) -> Self
    where
        F: Fn(&dyn AstNode, &mut FormatContext) -> FormatResult<()> + 'static,
        A: Fn(&dyn AstNode) -> bool + 'static,
    {
        Self { name, priority: 0, apply_fn: Box::new(apply_fn), applies_fn: Box::new(applies_fn) }
    }

    /// 设置优先级
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }
}

impl FormatRule for BasicFormatRule {
    fn name(&self) -> &str {
        &self.name
    }

    fn priority(&self) -> u8 {
        self.priority
    }

    fn applies_to(&self, node: &dyn AstNode) -> bool {
        (self.applies_fn)(node)
    }

    fn apply(&self, node: &dyn AstNode, context: &mut FormatContext) -> FormatResult<()> {
        (self.apply_fn)(node, context)
    }
}
