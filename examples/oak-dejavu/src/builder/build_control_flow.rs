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

    /// Builds a ForControlNode from a ForControl red node.
    pub(crate) fn build_for_control<S: Source + ?Sized>(&self, n: RedNode<DejavuLanguage>, source: &S) -> Result<ForControlNode, OakError> {
        let span = n.span();
        let mut pattern = None;
        let mut iterable = None;
        let mut body = Vec::new();
        let mut else_body = None;
        let mut in_else = false;

        for child in n.children() {
            match child {
                RedTree::Node(child_node) => match child_node.green.kind {
                    DejavuElementType::Whitespace | DejavuElementType::LineComment | DejavuElementType::BlockComment => continue,
                    DejavuElementType::Pattern => {
                        pattern = Some(self.build_pattern(child_node, source)?);
                    }
                    DejavuElementType::Expression => {
                        if iterable.is_none() {
                            iterable = Some(self.build_expr(child_node, source)?);
                        }
                    }
                    DejavuElementType::ElseBranch => {
                        let mut else_items = Vec::new();
                        for else_child in child_node.children() {
                            if let RedTree::Node(else_n) = else_child {
                                if let Ok(item) = self.build_item(else_n, source) {
                                    else_items.push(item);
                                }
                            }
                        }
                        else_body = Some(else_items);
                    }
                    _ => {
                        if in_else {
                            if else_body.is_none() {
                                else_body = Some(Vec::new());
                            }
                            if let Ok(item) = self.build_item(child_node, source) {
                                if let Some(ref mut else_items) = else_body {
                                    else_items.push(item);
                                }
                            }
                        }
                        else {
                            if let Ok(item) = self.build_item(child_node, source) {
                                body.push(item);
                            }
                        }
                    }
                },
                RedTree::Leaf(t) => match t.kind {
                    DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                    DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::Else) => {
                        in_else = true;
                    }
                    _ => {}
                },
            }
        }

        let default_pattern = PatternNode::Variable(VariablePatternNode { name: IdentifierNode { name: "_".to_string(), span: span.clone() }, span: span.clone() });
        let default_iterable = ExpressionNode::Ident(IdentifierNode { name: "[]".to_string(), span: span.clone() });

        Ok(ForControlNode { pattern: pattern.unwrap_or(default_pattern), iterable: iterable.unwrap_or(default_iterable), body, else_body, span })
    }

    /// Builds an IfControlNode from an IfControl red node.
    pub(crate) fn build_if_control<S: Source + ?Sized>(&self, n: RedNode<DejavuLanguage>, source: &S) -> Result<IfControlNode, OakError> {
        let span = n.span();
        let mut condition = None;
        let mut then_body = Vec::new();
        let mut else_branch = None;
        let mut in_else = false;

        for child in n.children() {
            match child {
                RedTree::Node(child_node) => match child_node.green.kind {
                    DejavuElementType::Whitespace | DejavuElementType::LineComment | DejavuElementType::BlockComment => continue,
                    DejavuElementType::Expression => {
                        if condition.is_none() {
                            condition = Some(self.build_expr(child_node, source)?);
                        }
                    }
                    DejavuElementType::ElseBranch => {
                        else_branch = Some(self.build_else_branch(child_node, source)?);
                    }
                    DejavuElementType::IfControl => {
                        if in_else {
                            let nested_if = self.build_if_control(child_node, source)?;
                            else_branch = Some(ElseBranchNode::Elif { condition: nested_if.condition, body: nested_if.then_body, else_branch: nested_if.else_branch.map(Box::new) });
                        }
                    }
                    _ => {
                        if in_else {
                            if else_branch.is_none() {
                                else_branch = Some(ElseBranchNode::Else { body: Vec::new() });
                            }
                            if let Ok(item) = self.build_item(child_node, source) {
                                if let Some(ElseBranchNode::Else { ref mut body }) = else_branch {
                                    body.push(item);
                                }
                            }
                        }
                        else {
                            if let Ok(item) = self.build_item(child_node, source) {
                                then_body.push(item);
                            }
                        }
                    }
                },
                RedTree::Leaf(t) => match t.kind {
                    DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                    DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::Else) => {
                        in_else = true;
                    }
                    _ => {}
                },
            }
        }

        let default_condition = ExpressionNode::Bool(BooleanLiteralNode { value: false, span: span.clone() });

        Ok(IfControlNode { condition: condition.unwrap_or(default_condition), then_body, else_branch, span })
    }

    /// Builds a WhileControlNode from a WhileControl red node.
    pub(crate) fn build_while_control<S: Source + ?Sized>(&self, n: RedNode<DejavuLanguage>, source: &S) -> Result<WhileControlNode, OakError> {
        let span = n.span();
        let mut condition = None;
        let mut body = Vec::new();

        for child in n.children() {
            match child {
                RedTree::Node(child_node) => match child_node.green.kind {
                    DejavuElementType::Whitespace | DejavuElementType::LineComment | DejavuElementType::BlockComment => continue,
                    DejavuElementType::Expression => {
                        if condition.is_none() {
                            condition = Some(self.build_expr(child_node, source)?);
                        }
                    }
                    _ => {
                        if let Ok(item) = self.build_item(child_node, source) {
                            body.push(item);
                        }
                    }
                },
                RedTree::Leaf(t) => match t.kind {
                    DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                    _ => {}
                },
            }
        }

        let default_condition = ExpressionNode::Bool(BooleanLiteralNode { value: false, span: span.clone() });

        Ok(WhileControlNode { condition: condition.unwrap_or(default_condition), body, span })
    }

    /// Builds a LoopControlNode from a LoopControl red node.
    pub(crate) fn build_loop_control<S: Source + ?Sized>(&self, n: RedNode<DejavuLanguage>, source: &S) -> Result<LoopControlNode, OakError> {
        let span = n.span();
        let mut pattern = None;
        let mut iterable = None;
        let mut body = Vec::new();

        for child in n.children() {
            match child {
                RedTree::Node(child_node) => match child_node.green.kind {
                    DejavuElementType::Whitespace | DejavuElementType::LineComment | DejavuElementType::BlockComment => continue,
                    DejavuElementType::Pattern => {
                        if pattern.is_none() {
                            pattern = Some(self.build_pattern(child_node, source)?);
                        }
                    }
                    DejavuElementType::Expression => {
                        if iterable.is_none() {
                            iterable = Some(self.build_expr(child_node, source)?);
                        }
                    }
                    _ => {
                        if let Ok(item) = self.build_item(child_node, source) {
                            body.push(item);
                        }
                    }
                },
                RedTree::Leaf(t) => match t.kind {
                    DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                    _ => {}
                },
            }
        }

        let default_pattern = PatternNode::Variable(VariablePatternNode { name: IdentifierNode { name: "_".to_string(), span: span.clone() }, span: span.clone() });
        let default_iterable = ExpressionNode::Ident(IdentifierNode { name: "[]".to_string(), span: span.clone() });

        Ok(LoopControlNode { pattern: pattern.unwrap_or(default_pattern), iterable: iterable.unwrap_or(default_iterable), body, span })
    }

    /// Builds an ElseBranchNode from an ElseBranch red node.
    pub(crate) fn build_else_branch<S: Source + ?Sized>(&self, n: RedNode<DejavuLanguage>, source: &S) -> Result<ElseBranchNode, OakError> {
        let span = n.span();
        let mut is_elif = false;
        let mut condition = None;
        let mut body = Vec::new();
        let mut nested_else = None;

        for child in n.children() {
            match child {
                RedTree::Node(child_node) => match child_node.green.kind {
                    DejavuElementType::Whitespace | DejavuElementType::LineComment | DejavuElementType::BlockComment => continue,
                    DejavuElementType::Expression => {
                        if condition.is_none() {
                            condition = Some(self.build_expr(child_node, source)?);
                        }
                    }
                    DejavuElementType::IfControl => {
                        let nested_if = self.build_if_control(child_node, source)?;
                        condition = Some(nested_if.condition);
                        body = nested_if.then_body;
                        nested_else = nested_if.else_branch.map(Box::new);
                        is_elif = true;
                    }
                    DejavuElementType::ElseBranch => {
                        nested_else = Some(Box::new(self.build_else_branch(child_node, source)?));
                    }
                    _ => {
                        if let Ok(item) = self.build_item(child_node, source) {
                            body.push(item);
                        }
                    }
                },
                RedTree::Leaf(t) => match t.kind {
                    DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                    DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::If) => {
                        is_elif = true;
                    }
                    _ => {}
                },
            }
        }

        if is_elif {
            let default_condition = ExpressionNode::Bool(BooleanLiteralNode { value: false, span: span.clone() });
            Ok(ElseBranchNode::Elif { condition: condition.unwrap_or(default_condition), body, else_branch: nested_else })
        }
        else {
            Ok(ElseBranchNode::Else { body })
        }
    }
}
