/// EJS AST module
///
/// This module defines the abstract syntax tree (AST) for EJS templates.
/// EJS (Embedded JavaScript) is a simple templating language that lets you
/// generate HTML markup with plain JavaScript.
use core::range::Range;

/// The root node of an EJS template AST
///
/// This structure represents the root of the abstract syntax tree for an EJS template.
/// It contains the span information that covers the entire template content.
///
/// # Example
///
/// ```ignore
/// let root = EjsRoot {
///     span: 0..template_length,
/// };
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EjsRoot {
    /// The span of the root node
    ///
    /// This field represents the byte range in the source text that this root node covers.
    /// Typically, this spans from the beginning (index 0) to the end of the template content.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl EjsRoot {
    /// Creates a new EJS root node with the given span
    ///
    /// # Arguments
    ///
    /// * `span` - The byte range in the source text that this root node covers
    ///
    /// # Returns
    ///
    /// A new `EjsRoot` instance with the specified span
    ///
    /// # Example
    ///
    /// ```ignore
    /// let root = EjsRoot::new(0..100);
    /// ```
    pub fn new(span: Range<usize>) -> Self {
        Self { span }
    }
}
