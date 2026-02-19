use crate::{
    DejavuBuilder, DejavuLanguage,
    ast::{AttributeNode, EffectDefinition, ExpressionNode, ExpressionStatement, IdentifierNode, LetStatement, NamePathNode, PatternNode, StatementNode, UsingStatement, VariablePatternNode},
    builder::text,
    lexer::token_type::DejavuTokenType,
    parser::element_type::DejavuElementType,
};
use oak_core::{OakError, RedNode, RedTree, Source};

impl<'config> DejavuBuilder<'config> {
    pub(crate) fn build_statement<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<StatementNode, OakError> {
        match node.green.kind {
            DejavuElementType::LetStatement => self.build_let(node, source),
            DejavuElementType::ExprStatement => self.build_expr_stmt(node, source),
            _ => Err(source.syntax_error(format!("Unknown statement type: {:?}", node.green.kind), node.span().start)),
        }
    }

    pub(crate) fn build_let<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<StatementNode, OakError> {
        let span = node.span();
        let mut children_iter = node
            .children()
            .filter(|c| match c {
                RedTree::Leaf(l) => !matches!(l.kind, DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment),
                RedTree::Node(n) => !matches!(n.green.kind, DejavuElementType::Whitespace | DejavuElementType::LineComment | DejavuElementType::BlockComment),
            })
            .peekable();

        let mut annotations = Vec::new();
        while let Some(child) = children_iter.peek() {
            if let RedTree::Node(n) = child {
                if n.green.kind == DejavuElementType::Attribute {
                    annotations.push(self.build_attribute(n.clone(), source)?);
                    children_iter.next();
                    continue;
                }
            }
            break;
        }

        let let_keyword = children_iter.next().ok_or_else(|| source.syntax_error("Missing 'let' keyword".to_string(), span.start))?;
        match let_keyword {
            RedTree::Leaf(t) if t.kind == DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::Let) => {}
            _ => {
                return Err(source.syntax_error("Expected 'let' keyword".to_string(), let_keyword.span().start));
            }
        }

        let mut is_mutable = false;
        if let Some(RedTree::Leaf(t)) = children_iter.peek() {
            if t.kind == DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::Mut) {
                is_mutable = true;
                children_iter.next();
            }
        }

        let pattern_node = children_iter.next().ok_or_else(|| source.syntax_error("Missing pattern in let statement".to_string(), span.start))?;
        let pattern = match pattern_node {
            RedTree::Node(n) => self.build_pattern(n, source)?,
            RedTree::Leaf(t) if t.kind == DejavuTokenType::Identifier => {
                let t_text = text(source, t.span.clone().into());
                PatternNode::Variable(VariablePatternNode { name: IdentifierNode { name: t_text, span: t.span.clone() }, span: t.span.clone() })
            }
            _ => {
                return Err(source.syntax_error("Expected pattern in let statement".to_string(), pattern_node.span().start));
            }
        };

        let mut expr: Option<ExpressionNode> = None;

        if let Some(RedTree::Leaf(t)) = children_iter.peek() {
            if t.kind == DejavuTokenType::Eq {
                children_iter.next();

                let expr_node = children_iter.next().ok_or_else(|| source.syntax_error("Missing expression after '=' in let statement".to_string(), span.end))?;

                expr = Some(match expr_node {
                    RedTree::Node(n) => self.build_expr(n, source)?,
                    RedTree::Leaf(t) => {
                        return Err(source.syntax_error("Expected an expression, found a token after '=' in let statement".to_string(), t.span.start));
                    }
                });
            }
        }

        while let Some(unexpected_child) = children_iter.next() {
            match unexpected_child {
                RedTree::Leaf(t) if t.kind == DejavuTokenType::Semicolon => {}
                _ => {
                    let span = unexpected_child.span();
                    if span.start == span.end {
                        continue;
                    }
                    return Err(source.syntax_error("Unexpected token or expression after let statement".to_string(), unexpected_child.span().start));
                }
            }
        }

        let expr = expr.ok_or_else(|| source.syntax_error("Missing expression in let statement".to_string(), span.start))?;

        Ok(StatementNode::Let(LetStatement { annotations, is_mutable, pattern, expr, span }))
    }

    pub(crate) fn build_expr_stmt<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<StatementNode, OakError> {
        let span = node.span();
        let mut children_iter = node
            .children()
            .filter(|c| match c {
                RedTree::Leaf(l) => !matches!(l.kind, DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment),
                RedTree::Node(n) => !matches!(n.green.kind, DejavuElementType::Whitespace | DejavuElementType::LineComment | DejavuElementType::BlockComment),
            })
            .peekable();

        let mut annotations = Vec::new();
        while let Some(child) = children_iter.peek() {
            if let RedTree::Node(n) = child {
                if n.green.kind == DejavuElementType::Attribute {
                    annotations.push(self.build_attribute(n.clone(), source)?);
                    children_iter.next();
                    continue;
                }
            }
            break;
        }

        let expr_node = children_iter.next().ok_or_else(|| source.syntax_error("Missing expression in expression statement".to_string(), span.start))?;

        let expr = match expr_node {
            RedTree::Node(n) => self.build_expr(n, source)?,
            RedTree::Leaf(t) => {
                return Err(source.syntax_error("Expected an expression, found a token".to_string(), t.span.start));
            }
        };

        let mut semi = false;
        while let Some(RedTree::Leaf(t)) = children_iter.peek() {
            if t.kind == DejavuTokenType::Semicolon {
                semi = true;
                children_iter.next();
                continue;
            }
            break;
        }

        while let Some(unexpected_child) = children_iter.next() {
            let span = unexpected_child.span();
            if span.start == span.end {
                continue;
            }
            return Err(source.syntax_error("Unexpected token or expression after semicolon".to_string(), unexpected_child.span().start));
        }

        Ok(StatementNode::Expr(ExpressionStatement { annotations, expr, semi, span }))
    }

    pub(crate) fn build_using<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<UsingStatement, OakError> {
        let span = node.span();
        let mut path = NamePathNode { parts: Vec::new(), span: Default::default() };

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                    _ => {}
                },
                RedTree::Node(n) => {
                    if n.green.kind == DejavuElementType::NamePath {
                        path = self.build_name_path(n, source)?;
                    }
                }
            }
        }
        Ok(UsingStatement { path, span })
    }

    pub(crate) fn build_effect<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<EffectDefinition, OakError> {
        let span = node.span();
        let mut name = IdentifierNode { name: String::new(), span: Default::default() };
        let mut annotations = Vec::new();
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                    DejavuTokenType::Identifier => name = IdentifierNode { name: text(source, t.span.clone().into()), span: t.span.clone() },
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuElementType::Attribute => annotations.push(self.build_attribute(n, source)?),
                    DejavuElementType::BlockExpression => {
                        for inner_child in n.children() {
                            if let RedTree::Node(inner_n) = inner_child {
                                if let Ok(item) = self.build_item(inner_n, source) {
                                    items.push(item);
                                }
                            }
                        }
                    }
                    _ => {}
                },
            }
        }
        Ok(EffectDefinition { name, annotations, items, span })
    }

    pub(crate) fn build_attribute<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<AttributeNode, OakError> {
        let span = node.span();
        let mut name = IdentifierNode { name: String::new(), span: Default::default() };
        let mut args = Vec::new();
        let mut seen_name = false;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                    DejavuTokenType::Identifier => {
                        if !seen_name {
                            name.name = text(source, t.span.clone().into());
                            name.span = t.span.clone();
                            seen_name = true;
                        }
                    }
                    DejavuTokenType::At => continue,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuElementType::Whitespace | DejavuElementType::LineComment | DejavuElementType::BlockComment => continue,
                    DejavuElementType::NamePath => {
                        if !seen_name {
                            let path = self.build_name_path(n, source)?;
                            name.name = path.parts.iter().map(|p| p.name.as_str()).collect::<Vec<_>>().join("::");
                            name.span = path.span;
                            seen_name = true;
                            continue;
                        }
                        args.push(self.build_expr(n, source)?);
                    }
                    _ => args.push(self.build_expr(n, source)?),
                },
            }
        }

        Ok(AttributeNode { name, args, span })
    }

    pub(crate) fn build_name_path<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<NamePathNode, OakError> {
        let span = node.span();
        let mut parts = Vec::new();

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                match t.kind {
                    DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                    DejavuTokenType::Identifier => parts.push(IdentifierNode { name: text(source, t.span.clone().into()), span: t.span.clone() }),
                    _ => {}
                }
            }
        }
        Ok(NamePathNode { parts, span })
    }
}
