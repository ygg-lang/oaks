use crate::{
    DejavuLanguage,
    ast::{IdentifierNode, MicroDefinition, ParameterNode, TypeFunctionDefinition},
    builder::{DejavuBuilder, text},
    lexer::token_type::DejavuTokenType,
    parser::element_type::DejavuElementType,
};
use oak_core::{OakError, RedNode, RedTree, Source};

impl<'config> DejavuBuilder<'config> {
    pub(crate) fn build_mezzo<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<TypeFunctionDefinition, OakError> {
        let span = node.span();
        let mut name = IdentifierNode { name: String::new(), span: Default::default() };
        let mut annotations = Vec::new();
        let mut params = Vec::new();
        let mut body = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                    DejavuTokenType::Identifier => {
                        name.name = text(source, t.span.clone().into());
                        name.span = t.span.clone();
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuElementType::Whitespace | DejavuElementType::LineComment | DejavuElementType::BlockComment => continue,
                    DejavuElementType::Attribute => {
                        annotations.push(self.build_attribute(n, source)?);
                    }
                    DejavuElementType::ParameterList => {
                        params = self.build_params(n, source)?;
                    }
                    DejavuElementType::BlockExpression => {
                        body = Some(self.build_block(n, source)?);
                    }
                    _ => {
                        // For now, ignore other nodes or add more cases as needed
                    }
                },
            }
        }

        let body = body.ok_or_else(|| source.syntax_error(format!("Missing mezzo body at {:?}", span), span.start))?;

        Ok(TypeFunctionDefinition { name, annotations, params, return_type: None, body, span })
    }

    pub(crate) fn build_micro<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<MicroDefinition, OakError> {
        let span = node.span();
        let mut name = IdentifierNode { name: String::new(), span: Default::default() };
        let mut annotations = Vec::new();
        let mut params = Vec::new();
        let mut return_type = None;
        let mut body = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                    DejavuTokenType::Identifier => {
                        if name.name.is_empty() {
                            name.name = text(source, t.span.clone().into());
                            name.span = t.span.clone();
                        }
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuElementType::Whitespace | DejavuElementType::LineComment | DejavuElementType::BlockComment => continue,
                    DejavuElementType::Attribute => {
                        annotations.push(self.build_attribute(n, source)?);
                    }
                    DejavuElementType::ParameterList => {
                        params = self.build_params(n, source)?;
                    }
                    DejavuElementType::Type => {
                        return_type = Some(text(source, n.span().into()).trim().to_string());
                    }
                    DejavuElementType::BlockExpression => {
                        body = Some(self.build_block(n, source)?);
                    }
                    _ => {
                        return Err(source.syntax_error(format!("Unexpected item in micro definition: {:?}", n.green.kind), n.span().start));
                    }
                },
            }
        }

        let body = body.ok_or_else(|| source.syntax_error(format!("Missing micro body at {:?}", span), span.start))?;

        Ok(MicroDefinition { name, annotations, params, return_type, body, span })
    }

    pub(crate) fn build_params<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<Vec<ParameterNode>, OakError> {
        let mut params = Vec::new();
        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuElementType::Whitespace | DejavuElementType::LineComment | DejavuElementType::BlockComment => continue,
                    DejavuElementType::Parameter => params.push(self.build_param(n, source)?),
                    _ => {}
                },
            }
        }
        Ok(params)
    }

    pub(crate) fn build_param<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<ParameterNode, OakError> {
        let span = node.span();
        let mut annotations = Vec::new();
        let mut name: Option<IdentifierNode> = None;
        let mut ty = None;
        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                    DejavuTokenType::Identifier => {
                        if name.is_none() {
                            name = Some(IdentifierNode { name: text(source, t.span.clone().into()), span: t.span.clone() });
                        }
                    }
                    DejavuTokenType::Colon => continue,
                    _ => return Err(source.syntax_error(format!("Unexpected token in parameter definition: {:?}", t.kind), t.span.start)),
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuElementType::Whitespace | DejavuElementType::LineComment | DejavuElementType::BlockComment => continue,
                    DejavuElementType::Attribute => annotations.push(self.build_attribute(n, source)?),
                    DejavuElementType::Type => ty = Some(text(source, n.span().into()).trim().to_string()),
                    _ => return Err(source.syntax_error(format!("Unexpected node in parameter definition: {:?}", n.green.kind), n.span().start)),
                },
            }
        }
        if let Some(name) = name { Ok(ParameterNode { annotations, name, ty, span }) } else { Err(source.syntax_error(format!("Missing name in parameter at {:?}", span), span.start)) }
    }
}
