use core::range::Range;

/// Stylus document root node
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StylusRoot {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    /// The source span of the root node.
    pub span: Range<usize>,
    /// The list of top-level items in the document.
    pub items: Vec<StylusItem>,
}

/// Stylus top-level item
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StylusItem {
    /// CSS rule item.
    Rule(StylusRule),
    /// Comment item.
    Comment(StylusComment),
    /// Mixin definition item.
    Mixin(StylusMixin),
    /// Variable definition item.
    Variable(StylusVariable),
    /// Import statement item.
    Import(StylusImport),
    /// Function definition item.
    Function(StylusFunction),
    /// If statement item.
    If(StylusIf),
    /// For loop item.
    For(StylusFor),
    /// While loop item.
    While(StylusWhile),
}

/// Stylus rule
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StylusRule {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    /// The source span of the rule.
    pub span: Range<usize>,
    /// Selector of the rule.
    pub selector: String,
    /// Properties of the rule.
    pub properties: Vec<StylusProperty>,
}

/// Stylus comment
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StylusComment {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    /// The source span of the comment.
    pub span: Range<usize>,
    /// Text of the comment.
    pub text: String,
}

/// Stylus property
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StylusProperty {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    /// The source span of the property.
    pub span: Range<usize>,
    /// Name of the property.
    pub name: String,
    /// Value of the property.
    pub value: String,
}

/// Stylus mixin definition
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StylusMixin {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    /// The source span of the mixin.
    pub span: Range<usize>,
    /// Name of the mixin.
    pub name: String,
    /// Parameters of the mixin.
    pub params: Vec<StylusParam>,
    /// Body of the mixin.
    pub body: Vec<StylusItem>,
}

/// Stylus variable definition
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StylusVariable {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    /// The source span of the variable.
    pub span: Range<usize>,
    /// Name of the variable.
    pub name: String,
    /// Value of the variable.
    pub value: String,
}

/// Stylus import statement
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StylusImport {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    /// The source span of the import.
    pub span: Range<usize>,
    /// Path to the imported file.
    pub path: String,
}

/// Stylus function definition
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StylusFunction {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    /// The source span of the function.
    pub span: Range<usize>,
    /// Name of the function.
    pub name: String,
    /// Parameters of the function.
    pub params: Vec<StylusParam>,
    /// Body of the function.
    pub body: Vec<StylusItem>,
    /// Return value of the function.
    pub return_value: Option<String>,
}

/// Stylus if statement
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StylusIf {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    /// The source span of the if statement.
    pub span: Range<usize>,
    /// Condition of the if statement.
    pub condition: String,
    /// Body of the if statement.
    pub body: Vec<StylusItem>,
    /// Else clause of the if statement.
    pub else_clause: Option<Vec<StylusItem>>,
}

/// Stylus for loop
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StylusFor {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    /// The source span of the for loop.
    pub span: Range<usize>,
    /// Variable name of the for loop.
    pub variable: String,
    /// Range expression of the for loop.
    pub range: String,
    /// Body of the for loop.
    pub body: Vec<StylusItem>,
}

/// Stylus while loop
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StylusWhile {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    /// The source span of the while loop.
    pub span: Range<usize>,
    /// Condition of the while loop.
    pub condition: String,
    /// Body of the while loop.
    pub body: Vec<StylusItem>,
}

/// Stylus parameter
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StylusParam {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    /// The source span of the parameter.
    pub span: Range<usize>,
    /// Name of the parameter.
    pub name: String,
    /// Default value of the parameter.
    pub default: Option<String>,
}
