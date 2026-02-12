use crate::{language::RbqLanguage, parser::element_type::RbqElementType};
use oak_core::{Range, tree::RedNode};

/// Represents an annotation in RBQ.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RbqAnnotation {
    /// The name of the annotation.
    pub name: String,
    /// List of arguments for the annotation.
    pub args: Vec<String>,
    /// The source range of the annotation.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl RbqAnnotation {
    /// Lowers a red node into an `RbqAnnotation` AST node.
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let span = red.span();
        let mut name = String::new();
        let mut args = Vec::new();

        for child in red.children() {
            match child.kind::<RbqElementType>() {
                RbqElementType::Ident if name.is_empty() => name = source[child.span()].trim().to_string(),
                RbqElementType::AnnotationArgs => {
                    if let Some(node) = child.as_node() {
                        for arg_child in node.children() {
                            match arg_child.kind::<RbqElementType>() {
                                RbqElementType::Literal | RbqElementType::MagicVar | RbqElementType::BinaryExpr | RbqElementType::MemberExpr | RbqElementType::CallExpr | RbqElementType::Ident => args.push(source[arg_child.span()].trim().to_string()),
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        Self { name, args, span }
    }
}
