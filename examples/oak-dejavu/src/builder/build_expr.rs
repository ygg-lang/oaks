use crate::{
    DejavuLanguage,
    ast::*,
    builder::{DejavuBuilder, text},
    lexer::token_type::DejavuTokenType,
    parser::element_type::DejavuElementType,
};
use oak_core::{OakError, RedNode, RedTree, Source};

impl<'config> DejavuBuilder<'config> {
    pub(crate) fn build_expr<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<ExpressionNode, OakError> {
        let node_kind = node.green.kind;
        let node_span = node.span();
        if node_kind == DejavuElementType::Error {
            return Err(source.syntax_error("Syntax error in expression".to_string(), node_span.start));
        }
        match node_kind {
            DejavuElementType::Expression => {
                for child in node.children() {
                    if let RedTree::Node(n) = child {
                        return self.build_expr(n, source);
                    }
                }
                Err(source.syntax_error(format!("Empty expression at {:?}", node_span), node_span.start))
            }
            DejavuElementType::IdentifierExpression => {
                let span = node.span();
                let mut ident: Option<IdentifierNode> = None;
                for child in node.children() {
                    match child {
                        RedTree::Leaf(t) => match t.kind {
                            DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                            DejavuTokenType::Identifier => {
                                let t_text = text(source, t.span.clone().into());
                                ident = Some(IdentifierNode { name: t_text, span: t.span.clone() });
                            }
                            DejavuTokenType::At | DejavuTokenType::Bolt => {
                                continue;
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
                if let Some(id) = ident {
                    return Ok(ExpressionNode::Ident(id));
                }
                Err(source.syntax_error(format!("Missing identifier in identifier expression at {:?}", span), span.start))
            }
            DejavuElementType::PathExpression | DejavuElementType::NamePath => {
                let mut path = NamePathNode { parts: Vec::new(), span: Default::default() };
                if node_kind == DejavuElementType::NamePath {
                    path = self.build_name_path(node, source)?;
                }
                else {
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
                }
                Ok(ExpressionNode::Path(path))
            }
            DejavuElementType::LiteralExpression => {
                let span = node.span();
                for child in node.children() {
                    match child {
                        RedTree::Leaf(t) => match t.kind {
                            DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                            DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::True) => return Ok(ExpressionNode::Bool(BooleanLiteralNode { value: true, span })),
                            DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::False) => return Ok(ExpressionNode::Bool(BooleanLiteralNode { value: false, span })),
                            _ => return Ok(ExpressionNode::Literal(LiteralExpressionNode { value: text(source, t.span.into()), span })),
                        },
                        RedTree::Node(_) => {}
                    }
                }
                Err(source.syntax_error(format!("Missing literal in literal expression at {:?}", span), span.start))
            }
            DejavuElementType::BooleanLiteral => {
                let span = node.span();
                for child in node.children() {
                    match child {
                        RedTree::Leaf(t) => match t.kind {
                            DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                            _ => return Ok(ExpressionNode::Bool(BooleanLiteralNode { value: t.kind == DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::True), span })),
                        },
                        RedTree::Node(_) => {}
                    }
                }
                Err(source.syntax_error(format!("Missing boolean literal in boolean literal expression at {:?}", span), span.start))
            }
            DejavuElementType::ParenthesizedExpression => {
                let span = node.span();
                for child in node.children() {
                    match child {
                        RedTree::Leaf(t) => match t.kind {
                            DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment | DejavuTokenType::LeftParen | DejavuTokenType::RightParen => continue,
                            _ => {}
                        },
                        RedTree::Node(n) => return Ok(ExpressionNode::Paren(ParenthesizedExpressionNode { expr: Box::new(self.build_expr(n, source)?), span })),
                    }
                }
                Err(source.syntax_error(format!("Missing expression in parenthesized expression at {:?}", span), span.start))
            }
            DejavuElementType::UnaryExpression => {
                let span = node.span();
                let mut op: Option<DejavuTokenType> = None;
                let mut expr: Option<ExpressionNode> = None;
                for child in node.children() {
                    match child {
                        RedTree::Node(n) => expr = Some(self.build_expr(n, source)?),
                        RedTree::Leaf(t) => match t.kind {
                            DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                            _ => {
                                if let oak_core::UniversalTokenRole::Operator = oak_core::TokenType::role(&t.kind) {
                                    op = Some(t.kind);
                                }
                            }
                        },
                    }
                }
                if let (Some(op_kind), Some(expr_val)) = (op, expr) {
                    Ok(ExpressionNode::Unary(UnaryExpressionNode { op: op_kind, expr: Box::new(expr_val), span }))
                }
                else {
                    Err(source.syntax_error(format!("Missing operand in unary expression at {:?}", span), span.start))
                }
            }
            DejavuElementType::BinaryExpression => {
                let span = node.span();
                let mut left: Option<ExpressionNode> = None;
                let mut op: Option<DejavuTokenType> = None;
                let mut right: Option<ExpressionNode> = None;
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
                            DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                            _ => {
                                if let oak_core::UniversalTokenRole::Operator = oak_core::TokenType::role(&t.kind) {
                                    op = Some(t.kind);
                                }
                            }
                        },
                    }
                }
                if let (Some(left_expr), Some(op_kind), Some(right_expr)) = (left, op, right) {
                    Ok(ExpressionNode::Binary(BinaryExpressionNode { left: Box::new(left_expr), op: op_kind, right: Box::new(right_expr), span }))
                }
                else {
                    Err(source.syntax_error(format!("Missing operands in binary expression at {:?}", span), span.start))
                }
            }
            DejavuElementType::CallExpression => {
                let span = node.span();
                let mut callee: Option<ExpressionNode> = None;
                let mut args: Vec<ExpressionNode> = Vec::new();
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
                            DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment | DejavuTokenType::Comma | DejavuTokenType::RightParen => continue,
                            DejavuTokenType::LeftParen => {
                                seen_paren = true;
                            }
                            _ => {}
                        },
                    }
                }
                if let Some(callee_expr) = callee { Ok(ExpressionNode::Call(CallExpressionNode { callee: Box::new(callee_expr), args, span })) } else { Err(source.syntax_error(format!("Missing callee in call expression at {:?}", span), span.start)) }
            }
            DejavuElementType::FieldExpression => {
                let span = node.span();
                let mut receiver: Option<ExpressionNode> = None;
                let mut field: Option<IdentifierNode> = None;
                let mut seen_dot = false;
                for child in node.children() {
                    match child {
                        RedTree::Node(n) => {
                            if receiver.is_none() {
                                receiver = Some(self.build_expr(n, source)?);
                            }
                            else if field.is_none() {
                                match self.build_expr(n, source)? {
                                    ExpressionNode::Ident(ident) => field = Some(ident),
                                    ExpressionNode::Path(path) if path.parts.len() == 1 => field = Some(path.parts[0].clone()),
                                    _ => return Err(source.syntax_error(format!("Expected identifier after '.', but found {:?}", n.green.kind), n.span().start)),
                                }
                            }
                        }
                        RedTree::Leaf(t) => match t.kind {
                            DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                            DejavuTokenType::Dot => {
                                seen_dot = true;
                            }
                            DejavuTokenType::Identifier => {
                                if seen_dot && field.is_none() {
                                    field = Some(IdentifierNode { name: text(source, t.span.clone().into()), span: t.span.clone() });
                                }
                            }
                            _ => {}
                        },
                    }
                }
                if let (Some(receiver_val), Some(field_val)) = (receiver, field) {
                    Ok(ExpressionNode::Field(FieldExpressionNode { receiver: Box::new(receiver_val), field: field_val, span }))
                }
                else {
                    Err(source.syntax_error(format!("Missing receiver or field in field expression at {:?}", span), span.start))
                }
            }
            DejavuElementType::IndexExpression => {
                let span = node.span();
                let mut base: Option<ExpressionNode> = None;
                let mut index: Option<ExpressionNode> = None;
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
                            DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment | DejavuTokenType::LeftBracket | DejavuTokenType::RightBracket => continue,
                            _ => {}
                        },
                    }
                }
                if let (Some(base_expr), Some(index_expr)) = (base, index) {
                    Ok(ExpressionNode::Index(IndexExpressionNode { receiver: Box::new(base_expr), index: Box::new(index_expr), span }))
                }
                else {
                    Err(source.syntax_error(format!("Missing base or index in index expression at {:?}", span), span.start))
                }
            }
            DejavuElementType::IfExpression => self.build_if(node, source),
            DejavuElementType::MatchExpression => self.build_match(node, source),
            DejavuElementType::LoopExpression => self.build_loop(node, source),
            DejavuElementType::ReturnExpression => self.build_return(node, source),
            DejavuElementType::ApplyBlock | DejavuElementType::ObjectExpression => {
                let span = node.span();
                let mut callee = None;
                let mut block = None;
                for child in node.children() {
                    match child {
                        RedTree::Node(n) => match n.green.kind {
                            DejavuElementType::Whitespace | DejavuElementType::LineComment | DejavuElementType::BlockComment => continue,
                            DejavuElementType::BlockExpression => block = Some(self.build_block(n, source)?),
                            DejavuElementType::NamePath => {
                                if callee.is_none() {
                                    callee = Some(ExpressionNode::Path(self.build_name_path(n, source)?));
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
                let callee = callee.ok_or_else(|| source.syntax_error("Missing callee in apply block".to_string(), span.start))?;
                let block = block.ok_or_else(|| source.syntax_error("Missing block in apply block".to_string(), span.end))?;
                Ok(ExpressionNode::Object(ObjectExpressionNode { callee: Box::new(callee), block, span }))
            }
            DejavuElementType::BlockExpression => {
                let block = self.build_block(node, source)?;
                Ok(ExpressionNode::Block(block))
            }
            DejavuElementType::Micro => {
                let lambda = self.build_lambda_expr(node, source)?;
                Ok(ExpressionNode::Lambda(lambda))
            }
            DejavuElementType::BreakExpression => self.build_break(node, source),
            DejavuElementType::ContinueExpression => self.build_continue(node, source),
            DejavuElementType::YieldExpression => self.build_yield(node, source),
            DejavuElementType::RaiseExpression => self.build_raise(node, source),
            DejavuElementType::CatchExpression => self.build_catch(node, source),
            DejavuElementType::ResumeExpression => self.build_resume(node, source),
            DejavuElementType::Error => Err(source.syntax_error(format!("Syntax error at {:?}", node.span()), node.span().start)),
            _ => Err(source.syntax_error(format!("Unknown expression type {:?} at {:?}", node.green.kind, node.span()), node.span().start)),
        }
    }

    pub(crate) fn build_lambda_expr<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<LambdaExpressionNode, OakError> {
        let span = node.span();
        let mut params = Vec::new();
        let mut return_type = None;
        let mut body = None;

        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    DejavuElementType::Whitespace | DejavuElementType::LineComment | DejavuElementType::BlockComment => continue,
                    DejavuElementType::Param => params.push(self.build_param(n, source)?),
                    DejavuElementType::BlockExpression => body = Some(self.build_block(n, source)?),
                    DejavuElementType::NamePath => return_type = Some(text(source, n.span().into())),
                    _ => {}
                },
                RedTree::Leaf(_) => {}
            }
        }

        Ok(LambdaExpressionNode { params, return_type, body: body.ok_or_else(|| source.syntax_error("Missing body in lambda".to_string(), span.end))?, span })
    }
}
