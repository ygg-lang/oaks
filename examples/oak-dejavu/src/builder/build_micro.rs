use crate::{DejavuLanguage, DejavuParser, ast::*, builder::text, lexer::token_type::DejavuSyntaxKind};
use oak_core::{OakError, RedNode, RedTree, source::SourceText};

impl<'config> DejavuParser<'config> {
    pub(crate) fn build_mezzo(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<TypeFunction, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut annotations = Vec::new();
        let mut params = Vec::new();
        let mut body = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::Identifier => {
                        name.name = text(source, t.span.clone().into());
                        name.span = t.span.clone();
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuSyntaxKind::Attribute => {
                        annotations.push(self.build_attribute(n, source)?);
                    }
                    DejavuSyntaxKind::ParameterList => {
                        params = self.build_params(n, source)?;
                    }
                    DejavuSyntaxKind::BlockExpression => {
                        body = Some(self.build_block(n, source)?);
                    }
                    _ => {
                        // For now, ignore other nodes or add more cases as needed
                    }
                },
            }
        }

        let body = body.ok_or_else(|| source.syntax_error(format!("Missing mezzo body at {:?}", span), span.start))?;

        Ok(TypeFunction { name, annotations, params, body, span })
    }

    pub(crate) fn build_micro(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<MicroDefinition, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut annotations = Vec::new();
        let mut params = Vec::new();
        let mut return_type = None;
        let mut body = None;

        for child in node.children() {
            match child {
                RedTree::Token(t) => match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::Identifier => {
                        if name.name.is_empty() {
                            name.name = text(source, t.span.clone().into());
                            name.span = t.span.clone();
                        }
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::Attribute => {
                        annotations.push(self.build_attribute(n, source)?);
                    }
                    DejavuSyntaxKind::ParameterList => {
                        params = self.build_params(n, source)?;
                    }
                    DejavuSyntaxKind::Type => {
                        return_type = Some(text(source, n.span().into()).trim().to_string());
                    }
                    DejavuSyntaxKind::BlockExpression => {
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

    pub(crate) fn build_lambda_expr(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<Lambda, OakError> {
        let span = node.span();
        let mut params = Vec::new();
        let mut return_type = None;
        let mut body = None;

        for child in node.children() {
            match child {
                RedTree::Token(t) => match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::ParameterList => params = self.build_params(n, source)?,
                    DejavuSyntaxKind::Type => return_type = Some(text(source, n.span().into()).trim().to_string()),
                    DejavuSyntaxKind::BlockExpression => body = Some(self.build_block(n, source)?),
                    _ => {}
                },
            }
        }

        let body = body.ok_or_else(|| source.syntax_error(format!("Missing lambda body at {:?}", span), span.start))?;

        Ok(Lambda { params, return_type, body, span })
    }

    pub(crate) fn build_params(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<Vec<Param>, OakError> {
        let mut params = Vec::new();
        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::Parameter => params.push(self.build_param(n, source)?),
                    _ => {}
                },
            }
        }
        Ok(params)
    }

    pub(crate) fn build_param(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<Param, OakError> {
        let span = node.span();
        let mut annotations = Vec::new();
        let mut name: Option<Identifier> = None;
        let mut ty = None;
        for child in node.children() {
            match child {
                RedTree::Token(t) => match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::Identifier => {
                        if name.is_none() {
                            name = Some(Identifier { name: text(source, t.span.clone().into()), span: t.span.clone() });
                        }
                    }
                    DejavuSyntaxKind::Colon => continue,
                    _ => return Err(source.syntax_error(format!("Unexpected token in parameter definition: {:?}", t.kind), t.span.start)),
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::Attribute => annotations.push(self.build_attribute(n, source)?),
                    DejavuSyntaxKind::Type => ty = Some(text(source, n.span().into()).trim().to_string()),
                    _ => return Err(source.syntax_error(format!("Unexpected node in parameter definition: {:?}", n.green.kind), n.span().start)),
                },
            }
        }
        if let Some(name) = name { Ok(Param { annotations, name, ty, span }) } else { Err(source.syntax_error(format!("Missing name in parameter at {:?}", span), span.start)) }
    }
}
