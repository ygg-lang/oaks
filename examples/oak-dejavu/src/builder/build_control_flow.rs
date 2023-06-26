use crate::{
    DejavuLanguage,
    ast::*,
    builder::{DejavuBuilder, text},
    lexer::token_type::DejavuTokenType,
    parser::element_type::DejavuElementType,
};
use oak_core::{OakError, RedNode, RedTree, Source};

impl<'config> DejavuBuilder<'config> {
    pub(crate) fn build_if<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<ExpressionNode, OakError> {
        let span = node.span();
        let mut pattern = None;
        let mut condition = None;
        let mut then_branch = None;
        let mut else_branch = None;
        let mut is_else = false;

        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    DejavuElementType::Whitespace | DejavuElementType::LineComment | DejavuElementType::BlockComment => continue,
                    DejavuElementType::Pattern => pattern = Some(self.build_pattern(n, source)?),
                    DejavuElementType::BlockExpression => {
                        if is_else {
                            else_branch = Some(self.build_block(n, source)?);
                        }
                        else {
                            then_branch = Some(self.build_block(n, source)?);
                        }
                    }
                    DejavuElementType::IfExpression => {
                        if is_else {
                            let nested_if = self.build_if(n, source)?;
                            let n_span = n.span();
                            else_branch = Some(BlockNode { statements: vec![StatementNode::Expr(ExpressionStatement { annotations: Vec::new(), expr: nested_if, semi: false, span: n_span.clone() })], span: n_span });
                        }
                    }
                    _ => {
                        if condition.is_none() {
                            condition = Some(Box::new(self.build_expr(n, source)?));
                        }
                    }
                },
                RedTree::Leaf(t) => match t.kind {
                    DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                    DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::Else) => is_else = true,
                    _ => {}
                },
            }
        }

        Ok(ExpressionNode::If(IfExpressionNode {
            pattern,
            condition: condition.ok_or_else(|| source.syntax_error("Missing if condition".to_string(), span.start))?,
            then_branch: then_branch.ok_or_else(|| source.syntax_error("Missing if then branch".to_string(), span.start))?,
            else_branch,
            span,
        }))
    }

    pub(crate) fn build_match<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<ExpressionNode, OakError> {
        let span = node.span();
        let mut scrutinee = None;
        let mut arms = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    DejavuElementType::Whitespace | DejavuElementType::LineComment | DejavuElementType::BlockComment => continue,
                    DejavuElementType::MatchArm => {
                        arms.push(self.build_match_arm(n, source)?);
                    }
                    _ => {
                        if scrutinee.is_none() {
                            scrutinee = Some(Box::new(self.build_expr(n, source)?));
                        }
                    }
                },
                RedTree::Leaf(t) => match t.kind {
                    DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                    _ => {}
                },
            }
        }

        Ok(ExpressionNode::Match(MatchExpressionNode { scrutinee: scrutinee.ok_or_else(|| source.syntax_error("Missing match scrutinee".to_string(), span.start))?, arms, span }))
    }

    pub(crate) fn build_match_arm<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<MatchArmNode, OakError> {
        let span = node.span();
        let mut pattern = None;
        let mut guard = None;
        let mut body = None;
        let mut is_guard = false;
        let mut _is_when_arm = false;

        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    DejavuElementType::Whitespace | DejavuElementType::LineComment | DejavuElementType::BlockComment => continue,
                    DejavuElementType::Pattern => pattern = Some(self.build_pattern(n, source)?),
                    _ => {
                        if is_guard {
                            guard = Some(self.build_expr(n, source)?);
                        }
                        else {
                            body = Some(self.build_expr(n, source)?);
                        }
                    }
                },
                RedTree::Leaf(t) => match t.kind {
                    DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                    DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::When) => is_guard = true,
                    DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::Case) => _is_when_arm = true,
                    _ => {}
                },
            }
        }

        Ok(MatchArmNode {
            pattern: pattern.ok_or_else(|| source.syntax_error("Missing pattern in match arm".to_string(), span.start))?,
            guard: guard.map(Box::new),
            body: Box::new(body.ok_or_else(|| source.syntax_error("Missing body in match arm".to_string(), span.end))?),
            span,
        })
    }

    pub(crate) fn build_loop<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<ExpressionNode, OakError> {
        let span = node.span();
        let mut label = None;
        let mut pattern = None;
        let mut condition = None;
        let mut body = None;

        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    DejavuElementType::Whitespace | DejavuElementType::LineComment | DejavuElementType::BlockComment => continue,
                    DejavuElementType::Pattern => pattern = Some(self.build_pattern(n, source)?),
                    DejavuElementType::BlockExpression => body = Some(self.build_block(n, source)?),
                    _ => {
                        if condition.is_none() {
                            condition = Some(Box::new(self.build_expr(n, source)?));
                        }
                    }
                },
                RedTree::Leaf(t) => match t.kind {
                    DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                    DejavuTokenType::Label => label = Some(text(source, t.span.into())),
                    _ => {}
                },
            }
        }

        Ok(ExpressionNode::Loop(LoopExpressionNode { label, pattern, condition, body: body.ok_or_else(|| source.syntax_error("Missing loop body".to_string(), span.end))?, span }))
    }

    pub(crate) fn build_return<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<ExpressionNode, OakError> {
        let span = node.span();
        let mut expr = None;

        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    DejavuElementType::Whitespace | DejavuElementType::LineComment | DejavuElementType::BlockComment => continue,
                    _ => expr = Some(Box::new(self.build_expr(n, source)?)),
                },
                RedTree::Leaf(_) => {}
            }
        }

        Ok(ExpressionNode::Return(ReturnExpressionNode { expr, span }))
    }

    pub(crate) fn build_break<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<ExpressionNode, OakError> {
        let span = node.span();
        let mut label = None;
        let mut expr = None;

        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    DejavuElementType::Whitespace | DejavuElementType::LineComment | DejavuElementType::BlockComment => continue,
                    _ => expr = Some(Box::new(self.build_expr(n, source)?)),
                },
                RedTree::Leaf(t) => match t.kind {
                    DejavuTokenType::Label => label = Some(text(source, t.span.into())),
                    _ => {}
                },
            }
        }

        Ok(ExpressionNode::Break(BreakExpressionNode { label, expr, span }))
    }

    pub(crate) fn build_continue<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<ExpressionNode, OakError> {
        let span = node.span();
        let mut label = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuTokenType::Label => label = Some(text(source, t.span.into())),
                    _ => {}
                },
                _ => {}
            }
        }

        Ok(ExpressionNode::Continue(ContinueExpressionNode { label, span }))
    }

    pub(crate) fn build_yield<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<ExpressionNode, OakError> {
        let span = node.span();
        let mut expr = None;
        let mut yield_from = false;

        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    DejavuElementType::Whitespace | DejavuElementType::LineComment | DejavuElementType::BlockComment => continue,
                    _ => expr = Some(Box::new(self.build_expr(n, source)?)),
                },
                RedTree::Leaf(t) => match t.kind {
                    DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::From) => yield_from = true,
                    _ => {}
                },
            }
        }

        Ok(ExpressionNode::Yield(YieldExpressionNode { expr, yield_from, span }))
    }

    pub(crate) fn build_raise<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<ExpressionNode, OakError> {
        let span = node.span();
        let mut expr = None;

        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    DejavuElementType::Whitespace | DejavuElementType::LineComment | DejavuElementType::BlockComment => continue,
                    _ => expr = Some(Box::new(self.build_expr(n, source)?)),
                },
                RedTree::Leaf(_) => {}
            }
        }

        Ok(ExpressionNode::Raise(RaiseExpressionNode { expr: expr.ok_or_else(|| source.syntax_error("Missing expression in raise".to_string(), span.end))?, span }))
    }

    pub(crate) fn build_catch<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<ExpressionNode, OakError> {
        let span = node.span();
        let mut return_type = None;
        let mut expr = None;
        let mut arms = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    DejavuElementType::Whitespace | DejavuElementType::LineComment | DejavuElementType::BlockComment => continue,
                    DejavuElementType::MatchArm => arms.push(self.build_match_arm(n, source)?),
                    DejavuElementType::NamePath => return_type = Some(self.build_name_path(n, source)?),
                    _ => {
                        if expr.is_none() {
                            expr = Some(Box::new(self.build_expr(n, source)?));
                        }
                    }
                },
                RedTree::Leaf(_) => {}
            }
        }

        Ok(ExpressionNode::Catch(CatchExpressionNode { return_type, expr: expr.ok_or_else(|| source.syntax_error("Missing expression in catch".to_string(), span.end))?, arms, span }))
    }

    pub(crate) fn build_resume<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<ExpressionNode, OakError> {
        let span = node.span();
        let mut expr = None;

        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    DejavuElementType::Whitespace | DejavuElementType::LineComment | DejavuElementType::BlockComment => continue,
                    _ => expr = Some(Box::new(self.build_expr(n, source)?)),
                },
                RedTree::Leaf(_) => {}
            }
        }

        Ok(ExpressionNode::Resume(ResumeExpressionNode { expr, span }))
    }
}
