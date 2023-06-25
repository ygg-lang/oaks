use crate::ast::Expression;
use core::range::Range;

/// Represents a JSX element.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JsxElement {
    /// Opening element tag.
    pub opening_element: JsxOpeningElement,
    /// Children of the JSX element.
    pub children: Vec<JsxChild>,
    /// Closing element tag.
    pub closing_element: JsxClosingElement,
    /// Source span of the JSX element.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a JSX opening element tag.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JsxOpeningElement {
    /// Name of the tag.
    pub name: JsxTagName,
    /// Attributes of the element.
    pub attributes: Vec<JsxAttributeOrSpread>,
    /// Source span of the opening element.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a JSX closing element tag.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JsxClosingElement {
    /// Name of the tag.
    pub name: JsxTagName,
    /// Source span of the closing element.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a self-closing JSX element.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JsxSelfClosingElement {
    /// Name of the tag.
    pub name: JsxTagName,
    /// Attributes of the element.
    pub attributes: Vec<JsxAttributeOrSpread>,
    /// Source span of the self-closing element.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a JSX fragment.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JsxFragment {
    /// Opening fragment tag (`<>`).
    pub opening_fragment: JsxOpeningFragment,
    /// Children of the fragment.
    pub children: Vec<JsxChild>,
    /// Closing fragment tag (`</>`).
    pub closing_fragment: JsxClosingFragment,
    /// Source span of the fragment.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a JSX opening fragment tag.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JsxOpeningFragment {
    /// Source span of the opening fragment.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a JSX closing fragment tag.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JsxClosingFragment {
    /// Source span of the closing fragment.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a child of a JSX element or fragment.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum JsxChild {
    /// A JSX element child.
    JsxElement(Box<JsxElement>),
    /// A JSX fragment child.
    JsxFragment(Box<JsxFragment>),
    /// A self-closing JSX element child.
    JsxSelfClosingElement(Box<JsxSelfClosingElement>),
    /// Literal text within JSX.
    JsxText(String),
    /// A JSX expression container (`{expression}`).
    JsxExpressionContainer(Option<Expression>),
}

/// Represents the name of a JSX tag.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum JsxTagName {
    /// A simple identifier name.
    Identifier(String),
    /// A member expression name (e.g., `MyComp.SubComp`).
    MemberExpression {
        /// Object part of the member expression.
        object: Box<JsxTagName>,
        /// Property part of the member expression.
        property: String,
    },
}

/// Represents either a JSX attribute or a spread attribute.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum JsxAttributeOrSpread {
    /// A regular attribute.
    Attribute(JsxAttribute),
    /// A spread attribute (`{...props}`).
    Spread(Expression),
}

/// Represents a JSX attribute.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JsxAttribute {
    /// Name of the attribute.
    pub name: String,
    /// Value of the attribute, if any.
    pub value: Option<JsxAttributeValue>,
    /// Source span of the attribute.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents the value of a JSX attribute.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum JsxAttributeValue {
    /// A string literal value.
    StringLiteral(String),
    /// An expression container value (`{expression}`).
    ExpressionContainer(Option<Expression>),
    /// A JSX element value.
    Element(Box<JsxElement>),
    /// A JSX fragment value.
    Fragment(Box<JsxFragment>),
}
