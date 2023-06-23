use crate::{ast::*, builder::TypeScriptBuilder, language::TypeScriptLanguage, parser::element_type::TypeScriptElementType};
use oak_core::{OakError, RedNode, RedTree, Source, SourceText};

impl<'config> TypeScriptBuilder<'config> {
    pub(crate) fn build_type_annotation(&self, node: &RedNode<TypeScriptLanguage>, source: &SourceText) -> Result<Option<TypeAnnotation>, OakError> {
        for child in node.children() {
            if let RedTree::Node(child_node) = child {
                let kind = child_node.green.kind;
                match kind {
                    TypeScriptElementType::PredefinedType => {
                        return Ok(Some(TypeAnnotation::Predefined(source.get_text_in(child_node.span().into()).to_string())));
                    }
                    TypeScriptElementType::TypeReference => {
                        let mut name = String::new();
                        let mut args = Vec::new();
                        for sub_child in child_node.children() {
                            if let RedTree::Node(sub_node) = sub_child {
                                match sub_node.green.kind {
                                    TypeScriptElementType::IdentifierName => {
                                        name = source.get_text_in(sub_node.span().into()).to_string();
                                    }
                                    TypeScriptElementType::TypeAnnotation => {
                                        if let Some(t) = self.build_type_annotation(&sub_node, source)? {
                                            args.push(t);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        return Ok(Some(TypeAnnotation::Reference { name, args }));
                    }
                    TypeScriptElementType::UnionType => {
                        let mut types = Vec::new();
                        for sub_child in child_node.children() {
                            if let RedTree::Node(sub_node) = sub_child {
                                if let Some(t) = self.build_type_annotation(&sub_node, source)? {
                                    types.push(t);
                                }
                            }
                        }
                        return Ok(Some(TypeAnnotation::Union(types)));
                    }
                    TypeScriptElementType::IntersectionType => {
                        let mut types = Vec::new();
                        for sub_child in child_node.children() {
                            if let RedTree::Node(sub_node) = sub_child {
                                if let Some(t) = self.build_type_annotation(&sub_node, source)? {
                                    types.push(t);
                                }
                            }
                        }
                        return Ok(Some(TypeAnnotation::Intersection(types)));
                    }
                    TypeScriptElementType::ArrayType => {
                        for sub_child in child_node.children() {
                            if let RedTree::Node(sub_node) = sub_child {
                                if let Some(t) = self.build_type_annotation(&sub_node, source)? {
                                    return Ok(Some(TypeAnnotation::Array(Box::new(t))));
                                }
                            }
                        }
                    }
                    TypeScriptElementType::TupleType => {
                        let mut types = Vec::new();
                        for sub_child in child_node.children() {
                            if let RedTree::Node(sub_node) = sub_child {
                                if let Some(t) = self.build_type_annotation(&sub_node, source)? {
                                    types.push(t);
                                }
                            }
                        }
                        return Ok(Some(TypeAnnotation::Tuple(types)));
                    }
                    TypeScriptElementType::LiteralType => {
                        for sub_child in child_node.children() {
                            if let RedTree::Node(sub_node) = sub_child {
                                match sub_node.green.kind {
                                    TypeScriptElementType::StringLiteral => {
                                        let text = source.get_text_in(sub_node.span().into());
                                        return Ok(Some(TypeAnnotation::Literal(LiteralType::String(text[1..text.len() - 1].to_string()))));
                                    }
                                    TypeScriptElementType::NumericLiteral => {
                                        let text = source.get_text_in(sub_node.span().into());
                                        if let Ok(n) = text.parse::<f64>() {
                                            return Ok(Some(TypeAnnotation::Literal(LiteralType::Number(n))));
                                        }
                                    }
                                    TypeScriptElementType::BooleanLiteral | TypeScriptElementType::True | TypeScriptElementType::False => {
                                        let text = source.get_text_in(sub_node.span().into());
                                        return Ok(Some(TypeAnnotation::Literal(LiteralType::Boolean(text == "true"))));
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(None)
    }

    pub(crate) fn build_type_parameter(&self, node: &RedNode<TypeScriptLanguage>, source: &SourceText) -> Result<Option<TypeParameter>, OakError> {
        let mut name = String::new();
        let mut constraint = None;
        let mut default = None;
        let span = node.span();

        for child in node.children() {
            if let RedTree::Node(child_node) = child {
                match child_node.green.kind {
                    TypeScriptElementType::IdentifierName => name = source.get_text_in(child_node.span().into()).to_string(),
                    TypeScriptElementType::TypeAnnotation => {
                        if constraint.is_none() {
                            constraint = self.build_type_annotation(&child_node, source)?
                        }
                        else {
                            default = self.build_type_annotation(&child_node, source)?
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(Some(TypeParameter { name, constraint, default, span: span.into() }))
    }
}
