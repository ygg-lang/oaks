/// Jinja AST module
///
/// This module defines the abstract syntax tree (AST) for Jinja templates.
use core::range::Range;

/// The root node of a Jinja template AST
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JinjaRoot {
    /// Child elements in the template
    pub elements: Vec<JinjaElement>,
    /// Source code span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl JinjaRoot {
    /// Creates a new Jinja root with the given elements and span
    pub fn new(elements: Vec<JinjaElement>, span: Range<usize>) -> Self {
        Self { elements, span }
    }
}

/// Jinja template element
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum JinjaElement {
    /// Text content outside Jinja delimiters
    Text {
        /// Raw text content
        content: String,
        /// Source code span
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// Variable expression `{{ variable }}`
    Variable {
        /// Variable expression text
        expression: String,
        /// Source code span
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// Block statement `{% block name %}...{% endblock %}`
    Block {
        /// Block name
        name: String,
        /// Block body elements
        body: Vec<JinjaElement>,
        /// Source code span
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// If statement `{% if condition %}...{% endif %}`
    IfStatement {
        /// Condition expression text
        condition: String,
        /// Body elements when condition is true
        body: Vec<JinjaElement>,
        /// Else body elements
        else_body: Option<Vec<JinjaElement>>,
        /// Source code span
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// For loop `{% for x in y %}...{% endfor %}`
    ForStatement {
        /// Loop variable name
        variable: String,
        /// Iterable expression text
        iterable: String,
        /// Loop body elements
        body: Vec<JinjaElement>,
        /// Else body elements
        else_body: Option<Vec<JinjaElement>>,
        /// Source code span
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// Macro definition `{% macro name(args) %}...{% endmacro %}`
    MacroDefinition {
        /// Macro name
        name: String,
        /// Macro parameter names
        params: Vec<String>,
        /// Macro body elements
        body: Vec<JinjaElement>,
        /// Source code span
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// Extends statement `{% extends "template" %}`
    Extends {
        /// Parent template name
        template: String,
        /// Source code span
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// Include statement `{% include "template" %}`
    Include {
        /// Included template name
        template: String,
        /// Source code span
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// Set statement `{% set x = expr %}`
    Set {
        /// Variable name
        name: String,
        /// Value expression text
        value: String,
        /// Source code span
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// Import statement `{% import "module" as name %}`
    Import {
        /// Import expression text
        expression: String,
        /// Source code span
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// From-import statement `{% from "module" import name %}`
    FromImport {
        /// Import expression text
        expression: String,
        /// Source code span
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// Comment `{# comment #}`
    Comment {
        /// Comment content
        content: String,
        /// Source code span
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// Generic tag `{% tag ... %}`
    Tag {
        /// Tag name
        name: String,
        /// Tag content text
        content: String,
        /// Source code span
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}
