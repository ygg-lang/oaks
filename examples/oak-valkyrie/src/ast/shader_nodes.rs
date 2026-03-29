use crate::ast::{Attribute, Identifier, Span, items_nodes::StatementNode};

/// A shader declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ShaderDeclaration {
    /// The shader name.
    pub name: Identifier,
    /// The shader kind (e.g., PBR, Unlit, Phong).
    pub kind: Identifier,
    /// Items declared within the shader (properties, uniforms, shader functions).
    pub items: Vec<StatementNode>,
    /// Annotations applied to the shader.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}
