use core::range::Range;

use oak_von::ast::{VonArray, VonField, VonObject, VonString, VonTuple, VonValue};

/// VOC root node.
#[derive(Debug, Clone, PartialEq)]
pub struct VocRoot;

/// VX document node.
#[derive(Debug, Clone, PartialEq)]
pub struct VxDocument {
    /// Template section.
    pub template: Option<TemplateNode>,
    /// Script section.
    pub script: Option<ScriptAst>,
    /// Style section.
    pub style: Option<StyleAst>,
}

/// Template node.
#[derive(Debug, Clone, PartialEq)]
pub enum TemplateNode {
    /// Text node.
    Text(VonString),
    /// Element node.
    Element {
        /// Tag name.
        tag: VonString,
        /// Attributes.
        attributes: Vec<Attribute>,
        /// Children nodes.
        children: Vec<TemplateNode>,
    },
}

impl TemplateNode {
    /// Creates a text node with the given value and a default span of `0..0`.
    pub fn text(value: impl Into<String>) -> Self {
        TemplateNode::Text(VonString { value: value.into(), span: Range::from(0..0) })
    }

    /// Creates an element node with the given tag, attributes, and children.
    ///
    /// All spans default to `0..0`.
    pub fn element(tag: impl Into<String>, attributes: Vec<(String, String)>, children: Vec<TemplateNode>) -> Self {
        TemplateNode::Element {
            tag: VonString { value: tag.into(), span: Range::from(0..0) },
            attributes: attributes.into_iter().map(|(name, value)| Attribute { name: VonString { value: name, span: Range::from(0..0) }, value: VonString { value, span: Range::from(0..0) } }).collect(),
            children,
        }
    }

    /// Returns the text content if this is a `Text` variant.
    pub fn text_value(&self) -> Option<&str> {
        match self {
            TemplateNode::Text(vs) => Some(&vs.value),
            _ => None,
        }
    }

    /// Returns the tag name if this is an `Element` variant.
    pub fn tag(&self) -> Option<&str> {
        match self {
            TemplateNode::Element { tag, .. } => Some(&tag.value),
            _ => None,
        }
    }

    /// Returns the attributes as key-value pairs if this is an `Element` variant.
    pub fn attributes(&self) -> Vec<(String, String)> {
        match self {
            TemplateNode::Element { attributes, .. } => attributes.iter().map(|attr| (attr.name.value.clone(), attr.value.value.clone())).collect(),
            _ => vec![],
        }
    }

    /// Returns a slice of child nodes if this is an `Element` variant.
    pub fn children(&self) -> &[TemplateNode] {
        match self {
            TemplateNode::Element { children, .. } => children,
            _ => &[],
        }
    }

    /// Converts this template node into a `VonValue` representation.
    pub fn von_value(&self) -> VonValue {
        match self {
            TemplateNode::Text(vs) => VonValue::String(vs.clone()),
            TemplateNode::Element { tag, attributes, children } => {
                let tag_field = VonField { name: "tag".to_string(), value: VonValue::String(tag.clone()), span: Range::from(0..0) };
                let attr_elements: Vec<VonValue> = attributes.iter().map(|attr| VonValue::Tuple(VonTuple { elements: vec![VonValue::String(attr.name.clone()), VonValue::String(attr.value.clone())], span: Range::from(0..0) })).collect();
                let attr_field = VonField { name: "attributes".to_string(), value: VonValue::Array(VonArray { elements: attr_elements, span: Range::from(0..0) }), span: Range::from(0..0) };
                let children_elements: Vec<VonValue> = children.iter().map(|c| c.von_value()).collect();
                let children_field = VonField { name: "children".to_string(), value: VonValue::Array(VonArray { elements: children_elements, span: Range::from(0..0) }), span: Range::from(0..0) };
                VonValue::Object(VonObject { fields: vec![tag_field, attr_field, children_field], span: Range::from(0..0) })
            }
        }
    }
}

/// An HTML attribute with a name and value.
#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    /// Attribute name.
    pub name: VonString,
    /// Attribute value.
    pub value: VonString,
}

/// Script AST node.
#[derive(Debug, Clone, PartialEq)]
pub struct ScriptAst {
    /// Raw source code.
    pub raw_source: String,
}

/// Style AST node.
#[derive(Debug, Clone, PartialEq)]
pub struct StyleAst {
    /// Style rules.
    pub rules: Vec<StyleRule>,
}

/// Style rule node.
#[derive(Debug, Clone, PartialEq)]
pub struct StyleRule {
    /// Selector.
    pub selector: String,
    /// Properties.
    pub properties: Vec<(String, String)>,
}

/// A parse error for VX documents.
#[derive(Debug, Clone, PartialEq)]
pub struct VxParseError {
    /// Error message.
    pub message: String,
    /// Line number where the error occurred.
    pub line: u32,
    /// Column number where the error occurred.
    pub column: u32,
}

impl std::fmt::Display for VxParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error: {} at {}:{}", self.message, self.line, self.column)
    }
}

impl std::error::Error for VxParseError {}
