use crate::{DejavuLanguage, DejavuParser, ast::*, builder::text, lexer::token_type::DejavuSyntaxKind};
use oak_core::{OakError, RedNode, RedTree, source::SourceText};

impl<'config> DejavuParser<'config> {
    pub(crate) fn build_expr(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<Expr, OakError> {
        let node_kind = node.green.kind;
        let node_span = node.span();
        if node_kind == DejavuSyntaxKind::Error {
            return Err(source.syntax_error("Syntax error in expression".to_string(), node_span.start));
        }
        match node_kind {
            DejavuSyntaxKind::IdentifierExpression => {
                let span = node.span();
                let mut ident: Option<Identifier> = None;
                for child in node.children() {
                    match child {
                        RedTree::Token(t) => match t.kind {
                            DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                            DejavuSyntaxKind::Identifier => {
                                let t_text = text(source, t.span.clone().into());
                                ident = Some(Identifier { name: t_text, span: t.span.clone() });
                            }
                            DejavuSyntaxKind::At | DejavuSyntaxKind::Bolt => {
                                continue;
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
                if let Some(id) = ident {
                    return Ok(Expr::Ident(id));
                }
                Err(source.syntax_error(format!("Missing identifier in identifier expression at {:?}", span), span.start))
            }
            DejavuSyntaxKind::PathExpression | DejavuSyntaxKind::NamePath => {
                let mut path = NamePath { parts: Vec::new(), span: Default::default() };
                if node_kind == DejavuSyntaxKind::NamePath {
                    path = self.build_name_path(node, source)?;
                }
                else {
                    for child in node.children() {
                        match child {
                            RedTree::Token(t) => match t.kind {
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
                }
                Ok(Expr::Path(path))
            }
            DejavuSyntaxKind::LiteralExpression => {
                let span = node.span();
                for child in node.children() {
                    match child {
                        RedTree::Token(t) => match t.kind {
                            DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                            DejavuSyntaxKind::Keyword(crate::lexer::DejavuKeywords::True) => return Ok(Expr::Bool { value: true, span }),
                            DejavuSyntaxKind::Keyword(crate::lexer::DejavuKeywords::False) => return Ok(Expr::Bool { value: false, span }),
                            _ => return Ok(Expr::Literal { value: text(source, t.span.into()), span }),
                        },
                        RedTree::Node(_) => {}
                    }
                }
                Err(source.syntax_error(format!("Missing literal in literal expression at {:?}", span), span.start))
            }
            DejavuSyntaxKind::BooleanLiteral => {
                let span = node.span();
                for child in node.children() {
                    match child {
                        RedTree::Token(t) => match t.kind {
                            DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                            _ => return Ok(Expr::Bool { value: t.kind == DejavuSyntaxKind::Keyword(crate::lexer::DejavuKeywords::True), span }),
                        },
                        RedTree::Node(_) => {}
                    }
                }
                Err(source.syntax_error(format!("Missing boolean literal in boolean literal expression at {:?}", span), span.start))
            }
            DejavuSyntaxKind::ParenthesizedExpression => {
                let span = node.span();
                for child in node.children() {
                    match child {
                        RedTree::Token(t) => match t.kind {
                            DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment | DejavuSyntaxKind::LeftParen | DejavuSyntaxKind::RightParen => continue,
                            _ => {}
                        },
                        RedTree::Node(n) => return Ok(Expr::Paren { expr: Box::new(self.build_expr(n, source)?), span }),
                    }
                }
                Err(source.syntax_error(format!("Missing expression in parenthesized expression at {:?}", span), span.start))
            }
            DejavuSyntaxKind::UnaryExpression => {
                let span = node.span();
                let mut op: Option<DejavuSyntaxKind> = None;
                let mut expr: Option<Expr> = None;
                for child in node.children() {
                    match child {
                        RedTree::Node(n) => expr = Some(self.build_expr(n, source)?),
                        RedTree::Leaf(t) => match t.kind {
                            DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                            _ => {
                                if let oak_core::UniversalTokenRole::Operator = oak_core::TokenType::role(&t.kind) {
                                    op = Some(t.kind);
                                }
                            }
                        },
                    }
                }
                if let (Some(op_kind), Some(expr_val)) = (op, expr) { Ok(Expr::Unary { op: op_kind, expr: Box::new(expr_val), span }) } else { Err(source.syntax_error(format!("Missing operand in unary expression at {:?}", span), span.start)) }
            }
            DejavuSyntaxKind::BinaryExpression => {
                let span = node.span();
                let mut left: Option<Expr> = None;
                let mut op: Option<DejavuSyntaxKind> = None;
                let mut right: Option<Expr> = None;
                for child in node.children() {
                    match child {
                        RedTree::Node(n) => {
                            if left.is_none() {
                                left = Some(self.build_expr(n, source)?);
                            }
                            else {
                                right = Some(self.build_expr(n, source)?);
                            }
                        }
                        RedTree::Leaf(t) => match t.kind {
                            DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                            _ => {
                                if let oak_core::UniversalTokenRole::Operator = oak_core::TokenType::role(&t.kind) {
                                    op = Some(t.kind);
                                }
                            }
                        },
                    }
                }
                if let (Some(left_expr), Some(op_kind), Some(right_expr)) = (left, op, right) {
                    Ok(Expr::Binary { left: Box::new(left_expr), op: op_kind, right: Box::new(right_expr), span })
                }
                else {
                    Err(source.syntax_error(format!("Missing operands in binary expression at {:?}", span), span.start))
                }
            }
            DejavuSyntaxKind::CallExpression => {
                let span = node.span();
                let mut callee: Option<Expr> = None;
                let mut args: Vec<Expr> = Vec::new();
                let mut seen_paren = false;
                for child in node.children() {
                    match child {
                        RedTree::Node(n) => {
                            if !seen_paren && callee.is_none() {
                                callee = Some(self.build_expr(n, source)?);
                            }
                            else {
                                args.push(self.build_expr(n, source)?);
                            }
                        }
                        RedTree::Leaf(t) => match t.kind {
                            DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment | DejavuSyntaxKind::Comma | DejavuSyntaxKind::RightParen => continue,
                            DejavuSyntaxKind::LeftParen => {
                                seen_paren = true;
                            }
                            _ => {}
                        },
                    }
                }
                if let Some(callee_expr) = callee { Ok(Expr::Call { callee: Box::new(callee_expr), args, span }) } else { Err(source.syntax_error(format!("Missing callee in call expression at {:?}", span), span.start)) }
            }
            DejavuSyntaxKind::FieldExpression => {
                let span = node.span();
                let mut receiver: Option<Expr> = None;
                let mut field: Option<Identifier> = None;
                let mut seen_dot = false;
                for child in node.children() {
                    match child {
                        RedTree::Node(n) => {
                            if receiver.is_none() {
                                receiver = Some(self.build_expr(n, source)?);
                            }
                            else if field.is_none() {
                                match self.build_expr(n, source)? {
                                    Expr::Ident(ident) => field = Some(ident),
                                    Expr::Path(path) if path.parts.len() == 1 => field = Some(path.parts[0].clone()),
                                    _ => return Err(source.syntax_error(format!("Expected identifier after '.', but found {:?}", n.green.kind), n.span().start)),
                                }
                            }
                        }
                        RedTree::Leaf(t) => match t.kind {
                            DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                            DejavuSyntaxKind::Dot => {
                                seen_dot = true;
                            }
                            DejavuSyntaxKind::Identifier => {
                                if seen_dot && field.is_none() {
                                    field = Some(Identifier { name: text(source, t.span.clone().into()), span: t.span.clone() });
                                }
                            }
                            _ => {}
                        },
                    }
                }
                if let (Some(receiver_val), Some(field_val)) = (receiver, field) {
                    Ok(Expr::Field { receiver: Box::new(receiver_val), field: field_val, span })
                }
                else {
                    Err(source.syntax_error(format!("Missing receiver or field in field expression at {:?}", span), span.start))
                }
            }
            DejavuSyntaxKind::IndexExpression => {
                let span = node.span();
                let mut base: Option<Expr> = None;
                let mut index: Option<Expr> = None;
                for child in node.children() {
                    match child {
                        RedTree::Node(n) => {
                            if base.is_none() {
                                base = Some(self.build_expr(n, source)?);
                            }
                            else {
                                index = Some(self.build_expr(n, source)?);
                            }
                        }
                        RedTree::Leaf(t) => match t.kind {
                            DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment | DejavuSyntaxKind::LeftBracket | DejavuSyntaxKind::RightBracket => continue,
                            _ => {}
                        },
                    }
                }
                if let (Some(base_expr), Some(index_expr)) = (base, index) {
                    Ok(Expr::Index { receiver: Box::new(base_expr), index: Box::new(index_expr), span })
                }
                else {
                    Err(source.syntax_error(format!("Missing base or index in index expression at {:?}", span), span.start))
                }
            }
            DejavuSyntaxKind::IfExpression => self.build_if(node, source),
            DejavuSyntaxKind::MatchExpression => self.build_match(node, source),
            DejavuSyntaxKind::LoopExpression => self.build_loop(node, source),
            DejavuSyntaxKind::ReturnExpression => self.build_return(node, source),
            DejavuSyntaxKind::ApplyBlock | DejavuSyntaxKind::ObjectExpression => {
                let span = node.span();
                let mut callee = None;
                let mut block = None;
                for child in node.children() {
                    match child {
                        RedTree::Node(n) => match n.green.kind {
                            DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                            DejavuSyntaxKind::BlockExpression => block = Some(self.build_block(n, source)?),
                            DejavuSyntaxKind::NamePath => {
                                if callee.is_none() {
                                    callee = Some(Expr::Path(self.build_name_path(n, source)?));
                                }
                            }
                            _ => {
                                if callee.is_none() {
                                    callee = Some(self.build_expr(n, source)?);
                                }
                            }
                        },
                        RedTree::Leaf(_) => {}
                    }
                }
                let callee = callee.ok_or_else(|| source.syntax_error("Missing callee in apply block", span.start))?;
                let block = block.ok_or_else(|| source.syntax_error("Missing block in apply block", span.end))?;
                Ok(Expr::Object { callee: Box::new(callee), block, span })
            }
            DejavuSyntaxKind::BlockExpression => {
                let block = self.build_block(node, source)?;
                Ok(Expr::Block(block))
            }
            DejavuSyntaxKind::Micro => {
                let lambda = self.build_lambda_expr(node, source)?;
                Ok(Expr::Lambda(lambda))
            }
            DejavuSyntaxKind::BreakExpression => self.build_break(node, source),
            DejavuSyntaxKind::ContinueExpression => self.build_continue(node, source),
            DejavuSyntaxKind::YieldExpression => self.build_yield(node, source),
            DejavuSyntaxKind::RaiseExpression => self.build_raise(node, source),
            DejavuSyntaxKind::CatchExpression => self.build_catch(node, source),
            DejavuSyntaxKind::ResumeExpression => self.build_resume(node, source),
            DejavuSyntaxKind::Error => Err(source.syntax_error(format!("Syntax error at {:?}", node.span()), node.span().start)),
            _ => Err(source.syntax_error(format!("Unknown expression type {:?} at {:?}", node.green.kind, node.span()), node.span().start)),
        }
    }

    pub(crate) fn build_if(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<Expr, OakError> {
        let span = node.span();
        let mut pattern = None;
        let mut condition = None;
        let mut then_branch = None;
        let mut else_branch = None;
        let mut is_else = false;

        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::Pattern => pattern = Some(self.build_pattern(n, source)?),
                    DejavuSyntaxKind::BlockExpression => {
                        if is_else {
                            else_branch = Some(self.build_block(n, source)?);
                        }
                        else {
                            then_branch = Some(self.build_block(n, source)?);
                        }
                    }
                    DejavuSyntaxKind::IfExpression => {
                        if is_else {
                            let nested_if = self.build_if(n, source)?;
                            let n_span = n.span();
                            else_branch = Some(Block { statements: vec![Statement::ExprStmt { annotations: Vec::new(), expr: nested_if, semi: false, span: n_span.clone() }], span: n_span });
                        }
                    }
                    _ => {
                        if condition.is_none() {
                            condition = Some(Box::new(self.build_expr(n, source)?));
                        }
                    }
                },
                RedTree::Leaf(t) => match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::Keyword(crate::lexer::DejavuKeywords::Else) => is_else = true,
                    _ => {}
                },
            }
        }

        Ok(Expr::If {
            pattern,
            condition: condition.ok_or_else(|| source.syntax_error("Missing if condition".to_string(), span.start))?,
            then_branch: then_branch.ok_or_else(|| source.syntax_error("Missing if then branch".to_string(), span.start))?,
            else_branch,
            span,
        })
    }

    pub(crate) fn build_match(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<Expr, OakError> {
        let span = node.span();
        let mut scrutinee = None;
        let mut arms = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::MatchArm => {
                        arms.push(self.build_match_arm(n, source)?);
                    }
                    _ => {
                        if scrutinee.is_none() {
                            scrutinee = Some(Box::new(self.build_expr(n, source)?));
                        }
                    }
                },
                RedTree::Leaf(t) => match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    _ => {}
                },
            }
        }

        Ok(Expr::Match { scrutinee: scrutinee.ok_or_else(|| source.syntax_error("Missing match scrutinee".to_string(), span.start))?, arms, span })
    }

    pub(crate) fn build_match_arm(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<MatchArm, OakError> {
        let span = node.span();
        let mut pattern = None;
        let mut guard = None;
        let mut body = None;
        let mut is_guard = false;
        let mut is_when_arm = false;

        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::Pattern => {
                        pattern = Some(self.build_pattern(n, source)?);
                    }
                    DejavuSyntaxKind::NamePath => {
                        pattern = Some(Pattern::Type { name: self.build_name_path(n, source)?, span: n.span() });
                    }
                    _ => {
                        if is_when_arm && pattern.is_none() {
                            pattern = Some(Pattern::Wildcard { span: n.span() });
                            guard = Some(self.build_expr(n, source)?)
                        }
                        else if is_guard && guard.is_none() {
                            guard = Some(self.build_expr(n, source)?)
                        }
                        else if body.is_none() {
                            body = Some(self.build_expr(n, source)?);
                        }
                    }
                },
                RedTree::Leaf(t) => match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment | DejavuSyntaxKind::Colon | DejavuSyntaxKind::Arrow | DejavuSyntaxKind::Comma => continue,
                    DejavuSyntaxKind::Keyword(crate::lexer::DejavuKeywords::When) => {
                        if pattern.is_none() && !is_guard {
                            is_when_arm = true;
                        }
                        else {
                            is_guard = true;
                        }
                    }
                    DejavuSyntaxKind::Keyword(crate::lexer::DejavuKeywords::Else) => pattern = Some(Pattern::Else { span: t.span.clone() }),
                    DejavuSyntaxKind::Keyword(crate::lexer::DejavuKeywords::Case) | DejavuSyntaxKind::Keyword(crate::lexer::DejavuKeywords::Type) => continue,
                    _ => {
                        if is_when_arm && pattern.is_none() {
                            pattern = Some(Pattern::Wildcard { span: t.span.clone() });
                        }
                    }
                },
            }
        }

        Ok(MatchArm { pattern: pattern.ok_or_else(|| source.syntax_error("Missing match arm pattern".to_string(), span.start))?, guard, body: body.ok_or_else(|| source.syntax_error("Missing match arm body".to_string(), span.start))?, span })
    }

    pub(crate) fn build_pattern(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<Pattern, OakError> {
        let span = node.span();
        let mut name_path = None;
        let mut fields = Vec::new();
        let mut is_class = false;
        let mut is_explicit_type = false;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::Identifier => {
                        if !is_class && name_path.is_none() {
                            return Ok(Pattern::Variable { name: Identifier { name: text(source, t.span.clone().into()), span: t.span.clone() }, span });
                        }
                    }
                    DejavuSyntaxKind::IntegerLiteral | DejavuSyntaxKind::StringLiteral | DejavuSyntaxKind::Keyword(crate::lexer::DejavuKeywords::True) | DejavuSyntaxKind::Keyword(crate::lexer::DejavuKeywords::False) => {
                        return Ok(Pattern::Literal { value: text(source, t.span.clone().into()), span });
                    }
                    DejavuSyntaxKind::Keyword(crate::lexer::DejavuKeywords::Else) => return Ok(Pattern::Else { span }),
                    DejavuSyntaxKind::Keyword(crate::lexer::DejavuKeywords::Is) => is_explicit_type = true,
                    DejavuSyntaxKind::Underscore => return Ok(Pattern::Wildcard { span }),
                    DejavuSyntaxKind::LeftBrace => is_class = true,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::NamePath => name_path = Some(self.build_name_path(n, source)?),
                    _ => {}
                },
            }
        }

        if is_class && name_path.is_some() {
            let mut current_field_name = None;
            for child in node.children() {
                match child {
                    RedTree::Leaf(t) if t.kind == DejavuSyntaxKind::Identifier => {
                        current_field_name = Some(Identifier { name: text(source, t.span.clone().into()), span: t.span.clone() });
                    }
                    RedTree::Node(n) if n.green.kind == DejavuSyntaxKind::Pattern => {
                        if let Some(field_name) = current_field_name.take() {
                            fields.push((field_name, self.build_pattern(n, source)?));
                        }
                    }
                    _ => {}
                }
            }
            return Ok(Pattern::Class { name: name_path.unwrap(), fields, span });
        }

        if let Some(path) = name_path {
            if is_explicit_type || path.parts.len() > 1 {
                return Ok(Pattern::Type { name: path, span });
            }
            return Ok(Pattern::Variable { name: path.parts[0].clone(), span });
        }

        Ok(Pattern::Wildcard { span })
    }

    pub(crate) fn build_loop(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<Expr, OakError> {
        let span = node.span();
        let mut label = None;
        let mut pattern = None;
        let mut condition = None;
        let mut body = None;
        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::Pattern => pattern = Some(self.build_pattern(n, source)?),
                    DejavuSyntaxKind::BlockExpression => body = Some(self.build_block(n, source)?),
                    _ => {
                        if condition.is_none() {
                            condition = Some(Box::new(self.build_expr(n, source)?));
                        }
                    }
                },
                RedTree::Leaf(t) => match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::Identifier => label = Some(text(source, t.span.into())),
                    _ => {}
                },
            }
        }
        Ok(Expr::Loop { label, pattern, condition, body: body.ok_or_else(|| source.syntax_error("Missing loop body".to_string(), span.start))?, span })
    }

    pub(crate) fn build_return(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<Expr, OakError> {
        let span = node.span();
        let mut expr = None;
        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    _ => expr = Some(Box::new(self.build_expr(n, source)?)),
                },

                RedTree::Leaf(t) => match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    _ => {}
                },
            }
        }
        Ok(Expr::Return { expr, span })
    }

    pub(crate) fn build_break(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<Expr, OakError> {
        let span = node.span();
        let mut label = None;
        let mut expr = None;
        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::Identifier => label = Some(text(source, t.span.into())),
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    _ => expr = Some(Box::new(self.build_expr(n, source)?)),
                },
            }
        }
        Ok(Expr::Break { label, expr, span })
    }

    pub(crate) fn build_continue(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<Expr, OakError> {
        let span = node.span();
        let mut label = None;
        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::Identifier => label = Some(text(source, t.span.into())),
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    _ => {}
                },
            }
        }
        Ok(Expr::Continue { label, span })
    }

    pub(crate) fn build_yield(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<Expr, OakError> {
        let span = node.span();
        let mut expr = None;
        let mut yield_from = false;
        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::Star => yield_from = true,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    _ => expr = Some(Box::new(self.build_expr(n, source)?)),
                },
            }
        }
        Ok(Expr::Yield { expr, yield_from, span })
    }

    pub(crate) fn build_raise(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<Expr, OakError> {
        let span = node.span();
        let mut expr = None;
        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    _ => expr = Some(Box::new(self.build_expr(n, source)?)),
                },
                RedTree::Leaf(t) => match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    _ => {}
                },
            }
        }
        Ok(Expr::Raise { expr: expr.ok_or_else(|| source.syntax_error("Missing raise expression".to_string(), span.start))?, span })
    }

    pub(crate) fn build_catch(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<Expr, OakError> {
        let span = node.span();
        let mut return_type = None;
        let mut expr = None;
        let mut arms = Vec::new();
        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::Type => {
                        let path_node = n.children().find_map(|c| {
                            if let RedTree::Node(child_n) = c {
                                if child_n.green.kind == DejavuSyntaxKind::NamePath {
                                    return Some(child_n);
                                }
                            }
                            None
                        });
                        if let Some(pn) = path_node {
                            return_type = Some(self.build_name_path(pn, source)?);
                        }
                    }
                    DejavuSyntaxKind::MatchArm => arms.push(self.build_match_arm(n, source)?),
                    _ => {
                        if expr.is_none() {
                            expr = Some(Box::new(self.build_expr(n, source)?));
                        }
                    }
                },
                RedTree::Leaf(t) => match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    _ => {}
                },
            }
        }

        Ok(Expr::Catch { return_type, expr: expr.ok_or_else(|| source.syntax_error("Missing catch expression".to_string(), span.start))?, arms, span })
    }

    pub(crate) fn build_resume(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<Expr, OakError> {
        let span = node.span();
        let mut expr = None;
        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    _ => expr = Some(Box::new(self.build_expr(n, source)?)),
                },
                RedTree::Leaf(t) => match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    _ => {}
                },
            }
        }
        Ok(Expr::Resume { expr, span })
    }

    pub(crate) fn build_block(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<Block, OakError> {
        let span = node.span();
        let mut statements = Vec::new();
        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment | DejavuSyntaxKind::LeftBrace | DejavuSyntaxKind::RightBrace | DejavuSyntaxKind::Comma => continue,
                    _ => return Err(source.syntax_error(format!("Unexpected token in block: {:?}", t.kind), t.span.start)),
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::LetStatement => statements.push(self.build_let(n, source)?),
                    DejavuSyntaxKind::ExpressionStatement => statements.push(self.build_expr_stmt(n, source)?),
                    _ => return Err(source.syntax_error(format!("Unexpected statement in block: {:?}", n.green.kind), n.span().start)),
                },
            }
        }
        Ok(Block { statements, span })
    }
}
