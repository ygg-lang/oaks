#![doc = include_str!("readme.md")]
use core::range::Range;
use oak_core::tree::{GreenNode, RedNode, TypedNode};

/// CSS selector variant.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Selector {
    /// Element selector (e.g., `div`).
    Element(String),
    /// Class selector (e.g., `.class`).
    Class(String),
    /// ID selector (e.g., `#id`).
    Id(String),
    /// Attribute selector (e.g., `[attr]`).
    Attribute {
        /// The attribute name.
        name: String,
        /// Optional attribute value.
        value: Option<String>,
        /// Optional attribute operator.
        operator: Option<String>,
    },
    /// Pseudo-class selector (e.g., `:hover`).
    PseudoClass(String),
    /// Pseudo-element selector (e.g., `::before`).
    PseudoElement(String),
    /// Descendant selector (e.g., `div p`).
    Descendant(Box<Selector>, Box<Selector>),
    /// Child selector (e.g., `div > p`).
    Child(Box<Selector>, Box<Selector>),
    /// Adjacent sibling selector (e.g., `div + p`).
    AdjacentSibling(Box<Selector>, Box<Selector>),
    /// General sibling selector (e.g., `div ~ p`).
    GeneralSibling(Box<Selector>, Box<Selector>),
}

/// CSS value variant.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// Keyword value (e.g., `none`, `auto`).
    Keyword(String),
    /// Numeric value with unit (e.g., `10px`, `50%`).
    Numeric {
        /// The numeric value.
        value: f64,
        /// Optional unit.
        unit: Option<String>,
    },
    /// Color value (e.g., `#fff`, `rgb(255, 255, 255)`).
    Color(String),
    /// String value (e.g., `"text"`).
    String(String),
    /// URL value (e.g., `url("image.png")`).
    Url(String),
    /// Function value (e.g., `calc(100% - 20px)`).
    Function {
        /// The function name.
        name: String,
        /// The function arguments.
        arguments: Vec<Value>,
    },
    /// List of values (e.g., `1px 2px 3px`).
    List(Vec<Value>),
    /// Nested value (for complex expressions).
    Nested(Box<Value>),
}

/// CSS declaration consisting of a property name, value, and source span.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Declaration {
    /// The property name (e.g., `color`).
    pub property: String,
    /// The property value.
    pub value: Value,
    /// The range in the source code where this declaration is defined.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// CSS rule set consisting of selectors and declarations.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct RuleSet {
    /// The selectors for this rule set.
    pub selectors: Vec<Selector>,
    /// The declarations for this rule set.
    pub declarations: Vec<Declaration>,
    /// The range in the source code where this rule set is defined.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// CSS at-rule variant.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum AtRule {
    /// Media query rule (e.g., `@media screen and (max-width: 600px)`).
    Media {
        /// The media query condition.
        condition: String,
        /// The rule sets inside the media query.
        rules: Vec<RuleSet>,
        /// The range in the source code where this media rule is defined.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// Import rule (e.g., `@import "style.css"`).
    Import {
        /// The import path.
        path: String,
        /// Optional media query.
        media: Option<String>,
        /// The range in the source code where this import rule is defined.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// Font face rule (e.g., `@font-face`).
    FontFace {
        /// The font face declarations.
        declarations: Vec<Declaration>,
        /// The range in the source code where this font face rule is defined.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// Keyframes rule (e.g., `@keyframes animation`).
    Keyframes {
        /// The animation name.
        name: String,
        /// The keyframe rules.
        keyframes: Vec<(String, Vec<Declaration>)>,
        /// The range in the source code where this keyframes rule is defined.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// Other at-rules.
    Other {
        /// The at-rule name.
        name: String,
        /// The at-rule content.
        content: String,
        /// The range in the source code where this at-rule is defined.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}

/// CSS AST node variant.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum CssNode {
    /// A rule set.
    RuleSet(RuleSet),
    /// An at-rule.
    AtRule(AtRule),
    /// A comment.
    Comment(String),
}

/// Root node of the CSS AST.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct CssRoot {
    /// The top-level nodes in the CSS document.
    pub nodes: Vec<CssNode>,
}

impl<'a> TypedNode<'a> for CssRoot {
    type Language = crate::CssLanguage;

    fn cast(_node: RedNode<'a, Self::Language>) -> Option<Self> {
        // 这里需要根据实际情况实现从 RedNode 到 CssRoot 的转换
        // 暂时返回一个空的 CssRoot
        Some(Self { nodes: Vec::new() })
    }

    fn green(&self) -> &'a GreenNode<'a, Self::Language> {
        // 这里需要返回底层的 GreenNode
        // 暂时返回一个空的 GreenNode 引用
        // 注意：这是一个临时实现，需要根据实际情况修复
        panic!("CssRoot::green() not implemented")
    }
}
