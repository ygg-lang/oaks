#![doc = "Svelte abstract syntax."]
use core::range::Range;

/// Svelte attribute or directive.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SvelteAttribute {
    /// Regular HTML attribute.
    Attribute(Attribute),
    /// Svelte directive or shorthand (e.g., on:click, bind:value, {value}).
    Directive(Directive),
}

/// Regular HTML attribute.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Attribute {
    /// The name of the attribute.
    pub name: String,
    /// The optional value of the attribute.
    pub value: Option<SvelteAttributeValue>,
    /// The source span of the attribute.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Svelte attribute value (can be literal or expression).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SvelteAttributeValue {
    /// Literal string value.
    Literal(String),
    /// Expression value `{value}`.
    Expression(String),
}

/// Svelte directive.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Directive {
    /// The type of directive (e.g., "on", "bind", "use", "class", "transition").
    pub kind: String,
    /// The name/argument of the directive (e.g., "click" in on:click).
    pub name: String,
    /// The optional expression value.
    pub expression: Option<String>,
    /// The source span of the directive.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents different types of nodes in a Svelte template.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SvelteNode {
    /// A Svelte/HTML element.
    Element(SvelteElement),
    /// Plain text content.
    Text(SvelteText),
    /// Svelte expression `{ expression }`.
    Expression(SvelteExpression),
    /// Svelte block (`{#if}`, `{#each}`, etc.).
    Block(SvelteBlock),
    /// An HTML comment.
    Comment(String),
}

/// A Svelte element (HTML tag or component).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SvelteElement {
    /// The name of the tag.
    pub tag_name: String,
    /// The attributes and directives of the element.
    pub attributes: Vec<SvelteAttribute>,
    /// The child nodes.
    pub children: Vec<SvelteNode>,
    /// The source span of the element.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Plain text content.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SvelteText {
    /// The text content.
    pub content: String,
    /// The source span of the text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Svelte expression `{ expression }`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SvelteExpression {
    /// The expression string.
    pub expression: String,
    /// The source span of the expression.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Svelte logic block (`{#if}`, `{#each}`, `{#await}`).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SvelteBlock {
    /// The type of block ("if", "each", "await", "key").
    pub kind: String,
    /// The expression of the block header.
    pub expression: String,
    /// The main children of the block.
    pub children: Vec<SvelteNode>,
    /// The optional else/then/catch branches.
    pub branches: Vec<SvelteBranch>,
    /// The source span of the entire block.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// A branch of a Svelte block (e.g., `{:else}`, `{:then}`).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SvelteBranch {
    /// The name of the branch ("else", "then", "catch").
    pub name: String,
    /// The optional expression for the branch (e.g., `{:else if ...}`).
    pub expression: Option<String>,
    /// The children of the branch.
    pub children: Vec<SvelteNode>,
    /// The source span of the branch.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// The root node of a Svelte component.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SvelteRoot {
    /// The top-level nodes in the Svelte file.
    pub nodes: Vec<SvelteNode>,
}
