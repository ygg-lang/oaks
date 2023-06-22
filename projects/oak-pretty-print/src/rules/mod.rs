use crate::{Doc, FormatContext, FormatResult};
use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};
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
    fn apply_node(&self, node: &RedNode<L>, context: &FormatContext<L>, format_children: &dyn Fn(&RedNode<L>) -> FormatResult<Doc>) -> FormatResult<Option<Doc>>;

    /// 应用格式化规则到 Token，返回可选的 Document
    fn apply_token(&self, token: &RedLeaf<L>, context: &FormatContext<L>) -> FormatResult<Option<Doc>>;

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
    pub fn applicable_rules_for_node(&self, node: &RedNode<L>) -> Vec<&dyn FormatRule<L>> {
        self.rules.iter().filter(|rule| rule.applies_to_node(node)).map(|rule| rule.as_ref()).collect()
    }

    /// 获取适用于 Token 的规则
    pub fn applicable_rules_for_token(&self, token: &RedLeaf<L>) -> Vec<&dyn FormatRule<L>> {
        self.rules.iter().filter(|rule| rule.applies_to_token(token)).map(|rule| rule.as_ref()).collect()
    }

    /// 应用所有适用的节点规则，返回第一个成功的 Document
    pub fn apply_node_rules(&self, node: &RedNode<L>, context: &FormatContext<L>, format_children: &dyn Fn(&RedNode<L>) -> FormatResult<Doc>) -> FormatResult<Option<Doc>> {
        let rules = self.applicable_rules_for_node(node);
        for rule in rules {
            if let Some(doc) = rule.apply_node(node, context, format_children)? {
                return Ok(Some(doc));
            }
        }
        Ok(None)
    }

    /// 应用所有适用的 Token 规则，返回第一个成功的 Document
    pub fn apply_token_rules(&self, token: &RedLeaf<L>, context: &FormatContext<L>) -> FormatResult<Option<Doc>> {
        let rules = self.applicable_rules_for_token(token);
        for rule in rules {
            if let Some(doc) = rule.apply_token(token, context)? {
                return Ok(Some(doc));
            }
        }
        Ok(None)
    }
}

/// 批量定义格式化规则的宏
#[macro_export]
macro_rules! define_rules {
    ($($name:ident {
        priority: $prio:expr,
        $(node($node_arg:ident, $ctx_arg:ident, $children_arg:ident) if $node_cond:expr => $node_body:expr,)?
        $(token($token_arg:ident, $ctx_arg_token:ident) if $token_cond:expr => $token_body:expr,)?
    })*) => {
        {
            let mut rules: Vec<Box<dyn $crate::FormatRule<L>>> = Vec::new();
            $(
                let mut rule = $crate::rules::CustomRule::new(stringify!($name).to_string())
                    .with_priority($prio);

                $(
                    rule = rule.with_applies_to_node(
                        #[allow(unused_variables)]
                        |$node_arg: &oak_core::tree::RedNode<L>| $node_cond
                    ).with_apply_node(
                        #[allow(unused_variables)]
                        |$node_arg: &oak_core::tree::RedNode<L>, $ctx_arg: &$crate::FormatContext<L>, $children_arg: &dyn Fn(&oak_core::tree::RedNode<L>) -> $crate::FormatResult<$crate::Doc>| {
                            $node_body
                        }
                    );
                )?

                $(
                    rule = rule.with_applies_to_token(
                        #[allow(unused_variables)]
                        |$token_arg: &oak_core::tree::RedLeaf<L>| $token_cond
                    ).with_apply_token(
                        #[allow(unused_variables)]
                        |$token_arg: &oak_core::tree::RedLeaf<L>, $ctx_arg_token: &$crate::FormatContext<L>| {
                            $token_body
                        }
                    );
                )?

                rules.push(Box::new(rule));
            )*
            rules
        }
    };
}

/// 自定义格式化规则实现
pub struct CustomRule<L: Language> {
    name: String,
    priority: u8,
    applies_to_node_fn: Option<Box<dyn Fn(&RedNode<L>) -> bool + Send + Sync>>,
    applies_to_token_fn: Option<Box<dyn Fn(&RedLeaf<L>) -> bool + Send + Sync>>,
    apply_node_fn: Option<Box<dyn Fn(&RedNode<L>, &FormatContext<L>, &dyn Fn(&RedNode<L>) -> FormatResult<Doc>) -> FormatResult<Option<Doc>> + Send + Sync>>,
    apply_token_fn: Option<Box<dyn Fn(&RedLeaf<L>, &FormatContext<L>) -> FormatResult<Option<Doc>> + Send + Sync>>,
}

impl<L: Language> CustomRule<L> {
    /// 创建新的自定义规则
    pub fn new(name: impl ToString) -> Self {
        Self { name: name.to_string(), priority: 0, applies_to_node_fn: None, applies_to_token_fn: None, apply_node_fn: None, apply_token_fn: None }
    }

    /// 设置优先级
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    /// 设置节点适用条件
    pub fn with_applies_to_node(mut self, f: impl Fn(&RedNode<L>) -> bool + Send + Sync + 'static) -> Self {
        self.applies_to_node_fn = Some(Box::new(f));
        self
    }

    /// 设置 Token 适用条件
    pub fn with_applies_to_token(mut self, f: impl Fn(&RedLeaf<L>) -> bool + Send + Sync + 'static) -> Self {
        self.applies_to_token_fn = Some(Box::new(f));
        self
    }

    /// 设置节点格式化逻辑
    pub fn with_apply_node(mut self, f: impl Fn(&RedNode<L>, &FormatContext<L>, &dyn Fn(&RedNode<L>) -> FormatResult<Doc>) -> FormatResult<Option<Doc>> + Send + Sync + 'static) -> Self {
        self.apply_node_fn = Some(Box::new(f));
        self
    }

    /// 设置 Token 格式化逻辑
    pub fn with_apply_token(mut self, f: impl Fn(&RedLeaf<L>, &FormatContext<L>) -> FormatResult<Option<Doc>> + Send + Sync + 'static) -> Self {
        self.apply_token_fn = Some(Box::new(f));
        self
    }
}

impl<L: Language> FormatRule<L> for CustomRule<L> {
    fn name(&self) -> &str {
        &self.name
    }

    fn priority(&self) -> u8 {
        self.priority
    }

    fn applies_to_node(&self, node: &RedNode<L>) -> bool {
        self.applies_to_node_fn.as_ref().map_or(false, |f| f(node))
    }

    fn applies_to_token(&self, token: &RedLeaf<L>) -> bool {
        self.applies_to_token_fn.as_ref().map_or(false, |f| f(token))
    }

    fn apply_node(&self, node: &RedNode<L>, context: &FormatContext<L>, format_children: &dyn Fn(&RedNode<L>) -> FormatResult<Doc>) -> FormatResult<Option<Doc>> {
        if let Some(f) = &self.apply_node_fn { f(node, context, format_children) } else { Ok(None) }
    }

    fn apply_token(&self, token: &RedLeaf<L>, context: &FormatContext<L>) -> FormatResult<Option<Doc>> {
        if let Some(f) = &self.apply_token_fn { f(token, context) } else { Ok(None) }
    }
}
