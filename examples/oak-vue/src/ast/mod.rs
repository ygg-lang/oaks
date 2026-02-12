#![doc = include_str!("readme.md")]
use core::range::Range;

use std::hash::Hash;

/// Vue attribute or directive.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum VueAttribute {
    /// Regular HTML attribute.
    Attribute(Attribute),
    /// Vue directive (e.g., v-if, v-for, :bind, @click).
    Directive(Directive),
}

/// Attribute value.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AttributeValue {
    /// The source span of the value.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Regular HTML attribute.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Attribute {
    /// The name of the attribute.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub name: Range<usize>,
    /// The optional value of the attribute.
    pub value: Option<AttributeValue>,
    /// The source span of the attribute.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Vue directive modifier.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Modifier {
    /// The name of the modifier.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub name: Range<usize>,
    /// The source span of the modifier.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl Modifier {
    pub fn new(name: Range<usize>, span: Range<usize>) -> Self {
        Self { name, span }
    }
}

/// Directive argument.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DirectiveArgument {
    /// The source span of the argument.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Directive expression.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DirectiveExpression {
    /// The source span of the expression.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Vue directive.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Directive {
    /// The name of the directive (e.g., "if", "for", "bind", "on").
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub name: Range<usize>,
    /// The optional argument (e.g., "id" in v-bind:id).
    pub arg: Option<DirectiveArgument>,
    /// The modifiers (e.g., "prevent" in @click.prevent).
    pub modifiers: Vec<Modifier>,
    /// The optional value (expression).
    pub value: Option<DirectiveExpression>,
    /// The source span of the directive.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents different types of nodes in a Vue template.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum VueNode {
    /// A Vue element or component.
    Element(VueElement),
    /// Plain text content.
    Text(VueText),
    /// Vue interpolation `{{ expression }}`.
    Interpolation(VueInterpolation),
    /// An HTML/Vue comment.
    Comment(String),
}

/// A Vue element (HTML tag or component).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VueElement {
    /// The name of the tag.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub tag_name: Range<usize>,
    /// The attributes and directives of the element.
    pub attributes: Vec<VueAttribute>,
    /// The child nodes.
    pub children: Vec<VueNode>,
    /// The source span of the element.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Plain text content.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VueText {
    /// The source span of the text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Vue interpolation `{{ expression }}`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VueInterpolation {
    /// The expression inside the interpolation.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub expression: Range<usize>,
    /// The source span of the interpolation.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// A Vue SFC (Single File Component) block (template, script, style).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VueBlock {
    /// The name of the block (e.g., "template", "script", "style").
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub name: Range<usize>,
    /// The attributes of the block (e.g., lang="ts", setup).
    pub attributes: Vec<VueAttribute>,
    /// The children nodes (if it's a template).
    pub children: Vec<VueNode>,
    /// The source span of the block.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// The root node of a Vue SFC.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VueRoot {
    /// The blocks (template, script, style) in the SFC.
    pub blocks: Vec<VueBlock>,
}
