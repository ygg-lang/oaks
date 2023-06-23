use crate::ast::Expression;
use core::range::Range;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JsxElement {
    pub opening_element: JsxOpeningElement,
    pub children: Vec<JsxChild>,
    pub closing_element: JsxClosingElement,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JsxOpeningElement {
    pub name: JsxTagName,
    pub attributes: Vec<JsxAttributeOrSpread>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JsxClosingElement {
    pub name: JsxTagName,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JsxSelfClosingElement {
    pub name: JsxTagName,
    pub attributes: Vec<JsxAttributeOrSpread>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JsxFragment {
    pub opening_fragment: JsxOpeningFragment,
    pub children: Vec<JsxChild>,
    pub closing_fragment: JsxClosingFragment,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JsxOpeningFragment {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JsxClosingFragment {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum JsxChild {
    JsxElement(Box<JsxElement>),
    JsxFragment(Box<JsxFragment>),
    JsxSelfClosingElement(Box<JsxSelfClosingElement>),
    JsxText(String),
    JsxExpressionContainer(Option<Expression>),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum JsxTagName {
    Identifier(String),
    MemberExpression { object: Box<JsxTagName>, property: String },
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum JsxAttributeOrSpread {
    Attribute(JsxAttribute),
    Spread(Expression),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JsxAttribute {
    pub name: String,
    pub value: Option<JsxAttributeValue>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum JsxAttributeValue {
    StringLiteral(String),
    ExpressionContainer(Option<Expression>),
    Element(Box<JsxElement>),
    Fragment(Box<JsxFragment>),
}
