/// Fluent abstract syntax tree module.

/// Fluent root node.
#[derive(Debug, Clone)]
pub struct FluentRoot {
    /// Fluent messages.
    pub messages: Vec<FluentMessage>,
}

/// Fluent message.
#[derive(Debug, Clone)]
pub struct FluentMessage {
    /// Message identifier.
    pub id: String,
    /// Message value.
    pub value: Option<FluentPattern>,
    /// Message attributes.
    pub attributes: Vec<FluentAttribute>,
    /// Message variants.
    pub variants: Vec<FluentVariant>,
}

/// Fluent attribute.
#[derive(Debug, Clone)]
pub struct FluentAttribute {
    /// Attribute identifier.
    pub id: String,
    /// Attribute value.
    pub value: FluentPattern,
}

/// Fluent variant.
#[derive(Debug, Clone)]
pub struct FluentVariant {
    /// Variant key.
    pub key: FluentVariantKey,
    /// Variant value.
    pub value: FluentPattern,
}

/// Fluent variant key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FluentVariantKey {
    /// Number key.
    Number(i64),
    /// String key.
    String(String),
}

/// Fluent pattern.
#[derive(Debug, Clone)]
pub struct FluentPattern {
    /// Pattern elements.
    pub elements: Vec<FluentPatternElement>,
}

/// Fluent pattern element.
#[derive(Debug, Clone)]
pub enum FluentPatternElement {
    /// Text element.
    Text(String),
    /// Variable reference.
    VariableReference(String),
    /// Message reference.
    MessageReference(String, Option<String>),
    /// Select expression.
    SelectExpression(Box<FluentExpression>, Vec<FluentVariant>),
}

/// Fluent expression.
#[derive(Debug, Clone)]
pub enum FluentExpression {
    /// Variable reference.
    VariableReference(String),
    /// Message reference.
    MessageReference(String, Option<String>),
    /// Number literal.
    NumberLiteral(i64),
}
