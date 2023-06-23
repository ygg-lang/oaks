use crate::{Document, FormatContext, FormatResult};
use alloc::{boxed::Box, vec::Vec};
use oak_core::{
    errors::OakError,
    language::Language,
    tree::{RedLeaf, RedNode},
};

/// Trait for defining formatting rules
pub trait FormatRule<L: Language> {
    /// The name of the rule
    fn name(&self) -> &str;

    /// The priority of the rule (higher numbers indicate higher priority)
    fn priority(&self) -> u8 {
        0
    }

    /// Checks if the rule applies to the given node
    fn applies_to_node(&self, _node: &RedNode<L>) -> bool {
        false
    }

    /// Checks if the rule applies to the given token
    fn applies_to_token(&self, _token: &RedLeaf<L>) -> bool {
        false
    }

    /// Applies the formatting rule to a node, returning an optional Document
    fn apply_node<'a>(&self, node: &RedNode<L>, context: &FormatContext<L>, source: &'a str, format_children: &dyn Fn(&RedNode<L>) -> FormatResult<Document<'a>>) -> FormatResult<Option<Document<'a>>>;

    /// Applies the formatting rule to a token, returning an optional Document
    fn apply_token<'a>(&self, token: &RedLeaf<L>, context: &FormatContext<L>, source: &'a str) -> FormatResult<Option<Document<'a>>>;

    /// Checks if the rule conflicts with another rule
    fn conflicts_with(&self, _other: &dyn FormatRule<L>) -> bool {
        false
    }
}

/// A collection of formatting rules
pub struct RuleSet<L: Language> {
    rules: Vec<Box<dyn FormatRule<L>>>,
}

impl<L: Language> Default for RuleSet<L> {
    fn default() -> Self {
        Self { rules: Vec::new() }
    }
}

impl<L: Language> RuleSet<L> {
    /// Creates a new rule set
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a rule to the set
    pub fn add_rule(&mut self, rule: Box<dyn FormatRule<L>>) -> FormatResult<()> {
        // Check for rule conflicts
        for existing_rule in &self.rules {
            if rule.conflicts_with(existing_rule.as_ref()) || existing_rule.conflicts_with(rule.as_ref()) {
                return Err(OakError::format_error(format!("Rule conflict between '{}' and '{}'", existing_rule.name(), rule.name())));
            }
        }

        self.rules.push(rule);

        // Sort by priority
        self.rules.sort_by(|a, b| b.priority().cmp(&a.priority()));

        Ok(())
    }

    /// Adds multiple rules to the set
    pub fn add_rules(&mut self, rules: Vec<Box<dyn FormatRule<L>>>) -> FormatResult<()> {
        for rule in rules {
            self.add_rule(rule)?
        }
        Ok(())
    }

    /// Gets rules applicable to a specific node
    pub fn applicable_rules_for_node<'a>(&'a self, node: &'a RedNode<L>) -> impl Iterator<Item = &'a dyn FormatRule<L>> + 'a {
        self.rules.iter().filter(move |rule| rule.applies_to_node(node)).map(|rule| rule.as_ref())
    }

    /// Gets rules applicable to a specific token
    pub fn applicable_rules_for_token<'a>(&'a self, token: &'a RedLeaf<L>) -> impl Iterator<Item = &'a dyn FormatRule<L>> + 'a {
        self.rules.iter().filter(move |rule| rule.applies_to_token(token)).map(|rule| rule.as_ref())
    }

    /// Applies all applicable node rules and returns the first successful Document
    pub fn apply_node_rules<'a>(&self, node: &RedNode<L>, context: &FormatContext<L>, source: &'a str, format_children: &dyn Fn(&RedNode<L>) -> FormatResult<Document<'a>>) -> FormatResult<Option<Document<'a>>> {
        for rule in self.applicable_rules_for_node(node) {
            if let Some(doc) = rule.apply_node(node, context, source, format_children)? {
                return Ok(Some(doc));
            }
        }
        Ok(None)
    }

    /// Applies all applicable token rules and returns the first successful Document
    pub fn apply_token_rules<'a>(&self, token: &RedLeaf<L>, context: &FormatContext<L>, source: &'a str) -> FormatResult<Option<Document<'a>>> {
        for rule in self.applicable_rules_for_token(token) {
            if let Some(doc) = rule.apply_token(token, context, source)? {
                return Ok(Some(doc));
            }
        }
        Ok(None)
    }
}
