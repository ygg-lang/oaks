use crate::{DejavuLanguage, DejavuParser, ast::*, builder::text, lexer::token_type::DejavuSyntaxKind};
use oak_core::{OakError, RedNode, RedTree, source::SourceText};

impl<'config> DejavuParser<'config> {
    pub(crate) fn build_let(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<Statement, OakError> {
        let span = node.span();
        let mut children_iter = node
            .children()
            .filter(|c| match c {
                RedTree::Leaf(l) => !matches!(l.kind, DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment),
                RedTree::Node(n) => !matches!(n.green.kind, DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment),
            })
            .peekable();

        let mut annotations = Vec::new();
        while let Some(child) = children_iter.peek() {
            if let RedTree::Node(n) = child {
                if n.green.kind == DejavuSyntaxKind::Attribute {
                    annotations.push(self.build_attribute(n.clone(), source)?);
                    children_iter.next();
                    continue;
                }
            }
            break;
        }

        let let_keyword = children_iter.next().ok_or_else(|| source.syntax_error("Missing 'let' keyword", span.start))?;
        match let_keyword {
            RedTree::Leaf(t) if t.kind == DejavuSyntaxKind::Keyword(crate::lexer::DejavuKeywords::Let) => {}
            _ => {
                return Err(source.syntax_error("Expected 'let' keyword", let_keyword.span().start));
            }
        }

        let mut is_mutable = false;
        if let Some(RedTree::Leaf(t)) = children_iter.peek() {
            if t.kind == DejavuSyntaxKind::Keyword(crate::lexer::DejavuKeywords::Mut) {
                is_mutable = true;
                children_iter.next();
            }
        }

        let pattern_node = children_iter.next().ok_or_else(|| source.syntax_error("Missing pattern in let statement", span.start))?;
        let pattern = match pattern_node {
            RedTree::Node(n) => self.build_pattern(n, source)?,
            RedTree::Leaf(t) if t.kind == DejavuSyntaxKind::Identifier => {
                let t_text = text(source, t.span.clone().into());
                Pattern::Variable { name: Identifier { name: t_text, span: t.span.clone() }, span: t.span.clone() }
            }
            _ => {
                return Err(source.syntax_error("Expected pattern in let statement", pattern_node.span().start));
            }
        };

        let mut expr: Option<Expr> = None;

        if let Some(RedTree::Leaf(t)) = children_iter.peek() {
            if t.kind == DejavuSyntaxKind::Eq {
                children_iter.next();

                let expr_node = children_iter.next().ok_or_else(|| source.syntax_error("Missing expression after '=' in let statement", span.end))?;

                expr = Some(match expr_node {
                    RedTree::Node(n) => self.build_expr(n, source)?,
                    RedTree::Leaf(t) => {
                        return Err(source.syntax_error("Expected an expression, found a token after '=' in let statement", t.span.start));
                    }
                });
            }
        }

        while let Some(unexpected_child) = children_iter.next() {
            match unexpected_child {
                RedTree::Leaf(t) if t.kind == DejavuSyntaxKind::Semicolon => {}
                _ => {
                    let span = unexpected_child.span();
                    if span.start == span.end {
                        continue;
                    }
                    return Err(source.syntax_error("Unexpected token or expression after let statement", unexpected_child.span().start));
                }
            }
        }

        let expr = expr.ok_or_else(|| source.syntax_error("Missing expression in let statement", span.start))?;

        Ok(Statement::Let { annotations, is_mutable, pattern, expr, span })
    }

    pub(crate) fn build_expr_stmt(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<Statement, OakError> {
        let span = node.span();
        let mut children_iter = node
            .children()
            .filter(|c| match c {
                RedTree::Leaf(l) => !matches!(l.kind, DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment),
                RedTree::Node(n) => !matches!(n.green.kind, DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment),
            })
            .peekable();

        let mut annotations = Vec::new();
        while let Some(child) = children_iter.peek() {
            if let RedTree::Node(n) = child {
                if n.green.kind == DejavuSyntaxKind::Attribute {
                    annotations.push(self.build_attribute(n.clone(), source)?);
                    children_iter.next();
                    continue;
                }
            }
            break;
        }

        let expr_node = children_iter.next().ok_or_else(|| source.syntax_error("Missing expression in expression statement", span.start))?;

        let expr = match expr_node {
            RedTree::Node(n) => self.build_expr(n, source)?,
            RedTree::Leaf(t) => {
                return Err(source.syntax_error("Expected an expression, found a token", t.span.start));
            }
        };

        let mut semi = false;
        while let Some(RedTree::Leaf(t)) = children_iter.peek() {
            if t.kind == DejavuSyntaxKind::Semicolon {
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
            return Err(source.syntax_error("Unexpected token or expression after semicolon", unexpected_child.span().start));
        }

        Ok(Statement::ExprStmt { annotations, expr, semi, span })
    }

    pub(crate) fn build_using(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<Using, OakError> {
        let span = node.span();
        let mut path = NamePath { parts: Vec::new(), span: Default::default() };

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    _ => {}
                },
                RedTree::Node(n) => {
                    if n.green.kind == DejavuSyntaxKind::NamePath {
                        path = self.build_name_path(n, source)?;
                    }
                }
            }
        }
        Ok(Using { path, span })
    }

    pub(crate) fn build_effect(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<EffectDefinition, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut annotations = Vec::new();
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::Identifier => name = Identifier { name: text(source, t.span.clone().into()), span: t.span.clone() },
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuSyntaxKind::Attribute => annotations.push(self.build_attribute(n, source)?),
                    DejavuSyntaxKind::BlockExpression => {
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

    pub(crate) fn build_attribute(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<Attribute, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut args = Vec::new();
        let mut seen_name = false;

        for child in node.children() {
            match child {
                RedTree::Token(t) => match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::Identifier => {
                        if !seen_name {
                            name.name = text(source, t.span.clone().into());
                            name.span = t.span.clone();
                            seen_name = true;
                        }
                    }
                    DejavuSyntaxKind::At => continue,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::NamePath => {
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

        Ok(Attribute { name, args, span })
    }

    pub(crate) fn build_name_path(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<NamePath, OakError> {
        let span = node.span();
        let mut parts = Vec::new();

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::Identifier => parts.push(Identifier { name: text(source, t.span.clone().into()), span: t.span.clone() }),
                    _ => {}
                }
            }
        }
        Ok(NamePath { parts, span })
    }
}
