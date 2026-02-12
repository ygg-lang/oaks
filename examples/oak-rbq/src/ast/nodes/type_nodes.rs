use super::data_def_nodes::RbqField;
use crate::{language::RbqLanguage, parser::element_type::RbqElementType};
use oak_core::{Range, tree::RedNode};

/// Represents a type reference in RBQ.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RbqType {
    /// A named type (e.g., `i32`, `Vec<T>`).
    Named {
        /// The path to the type.
        path: String,
        /// Generic arguments for the type.
        generic_args: Vec<RbqType>,
        /// Whether the type is a physical pointer.
        is_physical_ptr: bool,
        /// Whether the type is optional.
        is_optional: bool,
        /// The source range of the type reference.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// An inline struct definition used as a type.
    InlineStruct(Vec<RbqField>, #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))] Range<usize>),
    /// A physical reference to another type.
    PhysicalRef(Box<RbqType>, #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))] Range<usize>),
    /// An optional type.
    Optional(Box<RbqType>, #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))] Range<usize>),
    /// A literal type (used for numeric or string literals in generics).
    Literal(String, #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))] Range<usize>),
}

impl RbqType {
    /// Lowers a red node into an `RbqType` AST node.
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let span = red.span();
        let mut is_optional = false;
        let mut is_ref = false;
        let mut path = String::new();
        let mut generic_args = Vec::new();
        let mut inline_fields = Vec::new();
        let mut literal = None;

        for child in red.children() {
            match child.kind::<RbqElementType>() {
                RbqElementType::Question => is_optional = true,
                RbqElementType::Ampersand => is_ref = true,
                RbqElementType::Ident | RbqElementType::Utf8Kw => {
                    if path.is_empty() {
                        path = source[child.span()].trim().to_string();
                    }
                    else {
                        path.push('.');
                        path.push_str(source[child.span()].trim());
                    }
                }
                RbqElementType::Dot => {}
                RbqElementType::GenericArgs => {
                    if let Some(node) = child.as_node() {
                        for arg_child in node.children() {
                            if let Some(arg_node) = arg_child.as_node() {
                                generic_args.push(RbqType::lower(arg_node, source));
                            }
                            else if arg_child.kind::<RbqElementType>() == RbqElementType::NumberLiteral || arg_child.kind::<RbqElementType>() == RbqElementType::StringLiteral {
                                generic_args.push(RbqType::Literal(source[arg_child.span()].trim().to_string(), arg_child.span()));
                            }
                        }
                    }
                }
                RbqElementType::FieldDef => {
                    if let Some(node) = child.as_node() {
                        inline_fields.push(RbqField::lower(node, source));
                    }
                }
                RbqElementType::Literal => {
                    literal = Some((source[child.span()].trim().to_string(), child.span()));
                }
                _ => {}
            }
        }

        if !inline_fields.is_empty() {
            return RbqType::InlineStruct(inline_fields, span);
        }

        if let Some((lit, lit_span)) = literal {
            return RbqType::Literal(lit, lit_span);
        }

        let mut t = RbqType::Named { path, generic_args, is_physical_ptr: is_ref, is_optional, span: span.clone() };
        if is_ref {
            t = RbqType::PhysicalRef(Box::new(t), span.clone());
        }
        if is_optional {
            t = RbqType::Optional(Box::new(t), span);
        }
        t
    }
}
