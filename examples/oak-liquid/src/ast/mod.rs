/// Liquid AST module
///
/// This module defines the abstract syntax tree (AST) for Liquid templates.
use core::range::Range;

/// The root node of a Liquid template AST.
///
/// This is the top-level structure produced by the [`crate::LiquidBuilder`],
/// containing all child nodes of the parsed template.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidRoot {
    /// The byte span of the root node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// The top-level child nodes of the template.
    pub children: Vec<LiquidNode>,
}

/// A node in the Liquid template AST.
///
/// Each variant corresponds to a distinct syntactic construct in the
/// Liquid template language.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LiquidNode {
    /// Plain text content outside of any Liquid tag or expression.
    Text(LiquidText),
    /// Variable output expression `{{ ... }}`.
    Variable(LiquidVariable),
    /// Generic tag statement `{% ... %}`.
    Tag(LiquidTag),
    /// Comment block `{# ... #}`.
    Comment(LiquidComment),
    /// If / elsif / else conditional block.
    If(LiquidIf),
    /// For loop iteration block.
    For(LiquidFor),
    /// Block placeholder statement.
    Block(LiquidBlock),
    /// Variable assignment statement `{% assign ... %}`.
    Assign(LiquidAssign),
    /// Capture block statement `{% capture ... %}`.
    Capture(LiquidCapture),
    /// Case / when conditional block.
    Case(LiquidCase),
    /// Include statement for template inclusion.
    Include(LiquidInclude),
    /// Render statement for rendering a snippet (Liquid 5).
    Render(LiquidRender),
    /// Unless negated conditional block.
    Unless(LiquidUnless),
    /// Raw block for unprocessed content.
    Raw(LiquidRaw),
    /// Break statement for loop control.
    Break(LiquidBreak),
    /// Continue statement for loop control.
    Continue(LiquidContinue),
    /// Tablerow iteration statement.
    Tablerow(LiquidTablerow),
    /// Cycle statement for cycling through values.
    Cycle(LiquidCycle),
    /// Macro definition block.
    Macro(LiquidMacro),
    /// Error node for malformed constructs.
    Error(LiquidError),
}

/// Plain text content node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidText {
    /// The text content.
    pub text: String,
    /// The byte span in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Variable output expression node `{{ ... }}`.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidVariable {
    /// Child nodes within the variable expression.
    pub children: Vec<LiquidNode>,
    /// The byte span in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Generic tag statement node `{% ... %}`.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidTag {
    /// Child nodes within the tag statement.
    pub children: Vec<LiquidNode>,
    /// The byte span in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Comment block node `{# ... #}`.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidComment {
    /// The comment text content.
    pub text: String,
    /// The byte span in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// If / elsif / else conditional block node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidIf {
    /// Child nodes in the conditional body.
    pub children: Vec<LiquidNode>,
    /// The byte span in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// For loop iteration block node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidFor {
    /// Child nodes in the loop body.
    pub children: Vec<LiquidNode>,
    /// The byte span in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Block placeholder statement node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidBlock {
    /// Child nodes in the block body.
    pub children: Vec<LiquidNode>,
    /// The byte span in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Variable assignment statement node `{% assign ... %}`.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidAssign {
    /// Child nodes within the assign statement.
    pub children: Vec<LiquidNode>,
    /// The byte span in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Capture block statement node `{% capture ... %}`.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidCapture {
    /// Child nodes in the capture body.
    pub children: Vec<LiquidNode>,
    /// The byte span in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Case / when conditional block node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidCase {
    /// Child nodes in the case body.
    pub children: Vec<LiquidNode>,
    /// The byte span in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Include statement node for template inclusion.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidInclude {
    /// Child nodes within the include statement.
    pub children: Vec<LiquidNode>,
    /// The byte span in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Render statement node for rendering a snippet (Liquid 5).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidRender {
    /// Child nodes within the render statement.
    pub children: Vec<LiquidNode>,
    /// The byte span in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Unless negated conditional block node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidUnless {
    /// Child nodes in the unless body.
    pub children: Vec<LiquidNode>,
    /// The byte span in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Raw block node for unprocessed content.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidRaw {
    /// The raw text content.
    pub text: String,
    /// The byte span in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Break statement node for loop control.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidBreak {
    /// The byte span in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Continue statement node for loop control.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidContinue {
    /// The byte span in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Tablerow iteration statement node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidTablerow {
    /// Child nodes in the tablerow body.
    pub children: Vec<LiquidNode>,
    /// The byte span in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Cycle statement node for cycling through values.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidCycle {
    /// Child nodes within the cycle statement.
    pub children: Vec<LiquidNode>,
    /// The byte span in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Macro definition block node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidMacro {
    /// Child nodes in the macro body.
    pub children: Vec<LiquidNode>,
    /// The byte span in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Error node for malformed constructs.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidError {
    /// The byte span in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}
