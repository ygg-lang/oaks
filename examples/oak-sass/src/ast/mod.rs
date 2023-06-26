#![doc = include_str!("readme.md")]
use core::range::Range;

/// Root node of the Sass AST.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SassRoot {
    /// Statements in the Sass file.
    pub statements: Vec<SassStatement>,
    /// Source code span of the entire Sass file
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a Sass statement.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SassStatement {
    /// Variable definition: `$name: value;`
    VariableDefinition(VariableDefinition),
    /// Rule set: `selector { properties and nested rules }`
    RuleSet(RuleSet),
    /// At-rule: `@import`, `@media`, etc.
    AtRule(AtRule),
    /// CSS property within a ruleset or at-rule
    Property(Property),
    /// Error node for fault tolerance
    Error(ErrorNode),
}

/// Error node for fault tolerance
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ErrorNode {
    /// The error message describing what went wrong during parsing.
    pub message: String,
    /// Source code span where the error occurred.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Variable definition: `$name: value;`
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VariableDefinition {
    /// Source code span of the variable name (without the `$` prefix).
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub name: Range<usize>,
    /// Source code span of the variable value expression.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub value: Range<usize>,
    /// Source code span of the entire variable definition.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Rule set: `selector { properties and nested rules }`
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RuleSet {
    /// Source code span of the CSS selector.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub selector: Range<usize>,
    /// Nested child statements within this rule set.
    pub children: Vec<SassStatement>,
    /// CSS properties defined directly in this rule set.
    pub properties: Vec<Property>,
    /// Source code span of the entire rule set including selector and braces.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// CSS property: `name: value;`
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Property {
    /// Source code span of the property name.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub name: Range<usize>,
    /// Source code span of the property value.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub value: Range<usize>,
    /// Source code span of the entire property declaration including the semicolon.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// At-rule: `@name ...`
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AtRule {
    /// Source code span of the at-rule name (without the `@` prefix).
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub name: Range<usize>,
    /// Optional source code span of the at-rule parameters.
    #[cfg_attr(feature = "serde", serde(with = "serde_range_opt"))]
    pub params: Option<Range<usize>>,
    /// Optional body containing nested statements for at-rules with braces.
    pub body: Option<Vec<SassStatement>>,
    /// Source code span of the entire at-rule.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[cfg(feature = "serde")]
mod serde_range_opt {
    use super::*;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &Option<Range<usize>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(range) => oak_core::serde_range::serialize(range, serializer),
            None => serializer.serialize_none(),
        }
    }

    #[derive(Deserialize)]
    struct RangeDef {
        start: usize,
        end: usize,
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Range<usize>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<RangeDef> = Option::deserialize(deserializer)?;
        Ok(opt.map(|def| Range { start: def.start, end: def.end }))
    }
}
