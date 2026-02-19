use crate::{
    ValkyrieLanguage,
    ast::*,
    builder::{ValkyrieBuilder, text},
    lexer::{ValkyrieKeywords, token_type::ValkyrieTokenType},
    parser::{element_type::ValkyrieElementType, parse_string_segments},
};
use oak_core::{OakError, RedNode, RedTree, Source};

/// Counts the number of leading double quotes in a string.
fn count_leading_quotes(text: &str) -> u8 {
    let mut count = 0u8;
    for ch in text.chars() {
        if ch == '"' {
            count += 1;
        }
        else {
            break;
        }
    }
    count
}

/// Extracts the content of a string literal by removing leading and trailing quotes.
fn extract_content(text: &str, quote_count: u8) -> &str {
    let start = quote_count as usize;
    let end = text.len().saturating_sub(quote_count as usize);
    if start >= end {
        return "";
    }
    &text[start..end]
}

impl<'config> ValkyrieBuilder<'config> {
    pub(crate) fn build_expr<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<Expr, OakError> {
        let span = node.span();
        match node.green.kind {
            ValkyrieElementType::LiteralExpression => self.build_literal(node, source),
            ValkyrieElementType::BooleanLiteral => self.build_bool_literal(node, source),
            ValkyrieElementType::BinaryExpression => self.build_binary(node, source),
            ValkyrieElementType::UnaryExpression => self.build_unary(node, source),
            ValkyrieElementType::CallExpression => self.build_call(node, source),
            ValkyrieElementType::FieldExpression => self.build_field_expr(node, source),
            ValkyrieElementType::IndexExpression => self.build_index(node, source),
            ValkyrieElementType::OffsetExpression => self.build_offset(node, source),
            ValkyrieElementType::ParenthesizedExpression => self.build_paren(node, source),
            ValkyrieElementType::BlockExpression => Ok(Expr::Block(self.build_block(node, source)?)),
            ValkyrieElementType::LambdaExpression => Ok(Expr::Lambda(self.build_lambda_expr(node, source)?)),
            ValkyrieElementType::ObjectExpression => self.build_object(node, source),
            ValkyrieElementType::IfExpression => self.build_if(node, source),
            ValkyrieElementType::MatchExpression => self.build_match(node, source),
            ValkyrieElementType::LoopExpression => self.build_loop(node, source),
            ValkyrieElementType::ReturnExpression => self.build_return(node, source),
            ValkyrieElementType::BreakExpression => self.build_break(node, source),
            ValkyrieElementType::ContinueExpression => self.build_continue(node, source),
            ValkyrieElementType::YieldExpression => self.build_yield(node, source),
            ValkyrieElementType::RaiseExpression => self.build_raise(node, source),
            ValkyrieElementType::CatchExpression => self.build_catch(node, source),
            ValkyrieElementType::IdentifierExpression => self.build_identifier_expr(node, source),
            ValkyrieElementType::PathExpression => self.build_path_expr(node, source),
            ValkyrieElementType::AnonymousClass => self.build_anonymous_class(node, source),
            _ => Err(source.syntax_error(format!("Unexpected expression kind: {:?}", node.green.kind), span.start)),
        }
    }

    pub(crate) fn build_literal<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<Expr, OakError> {
        let span = node.span();
        let mut prefix: Option<Identifier> = None;
        let mut string_value: Option<String> = None;
        let mut string_span: Option<oak_core::Range<usize>> = None;

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    ValkyrieTokenType::StringPrefix => {
                        let prefix_text = text(source, t.span);
                        prefix = Some(Identifier { name: prefix_text, span: t.span });
                    }
                    ValkyrieTokenType::StringLiteral => {
                        string_value = Some(text(source, t.span));
                        string_span = Some(t.span);
                    }
                    ValkyrieTokenType::IntegerLiteral | ValkyrieTokenType::FloatLiteral => {
                        let value = text(source, t.span);
                        return Ok(Expr::StringLiteral(StringLiteral { prefix: None, quote_count: 0, segments: vec![StringSegment::Text { content: value, span: t.span }], span }));
                    }
                    _ => {}
                }
            }
        }

        if let (Some(raw_text), Some(str_span)) = (string_value, string_span) {
            let quote_count = count_leading_quotes(&raw_text);
            let content = extract_content(&raw_text, quote_count);
            let is_raw = prefix.as_ref().map(|p| p.name == "r").unwrap_or(false);
            let content_start = str_span.start + quote_count as usize;
            let segments = parse_string_segments(content, content_start, is_raw);

            Ok(Expr::StringLiteral(StringLiteral { prefix, quote_count, segments, span }))
        }
        else {
            Err(source.syntax_error("Missing string literal value".to_string(), span.start))
        }
    }

    pub(crate) fn build_bool_literal<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<Expr, OakError> {
        let span = node.span();
        let mut value = false;
        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == ValkyrieTokenType::BoolLiteral {
                    let text_val = text(source, t.span);
                    value = text_val == "true";
                }
            }
        }
        Ok(Expr::Bool { value, span })
    }

    pub(crate) fn build_binary<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<Expr, OakError> {
        let span = node.span();
        let mut left = None;
        let mut right = None;
        let mut op = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    _ => {
                        if op.is_none() && left.is_some() {
                            op = Some(t.kind);
                        }
                    }
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieElementType::Whitespace | ValkyrieElementType::Newline | ValkyrieElementType::LineComment | ValkyrieElementType::BlockComment => continue,
                    _ => {
                        if left.is_none() {
                            left = Some(Box::new(self.build_expr(n, source)?));
                        }
                        else if right.is_none() {
                            right = Some(Box::new(self.build_expr(n, source)?));
                        }
                    }
                },
            }
        }

        let left = left.ok_or_else(|| source.syntax_error("Missing left operand".to_string(), span.start))?;
        let right = right.ok_or_else(|| source.syntax_error("Missing right operand".to_string(), span.start))?;
        let op = op.unwrap_or(ValkyrieTokenType::Plus);

        Ok(Expr::Binary { left, op, right, span })
    }

    pub(crate) fn build_unary<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<Expr, OakError> {
        let span = node.span();
        let mut expr = None;
        let mut op = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    _ => {
                        if op.is_none() {
                            op = Some(t.kind);
                        }
                    }
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieElementType::Whitespace | ValkyrieElementType::Newline | ValkyrieElementType::LineComment | ValkyrieElementType::BlockComment => continue,
                    _ => {
                        expr = Some(Box::new(self.build_expr(n, source)?));
                    }
                },
            }
        }

        let expr = expr.ok_or_else(|| source.syntax_error("Missing operand".to_string(), span.start))?;
        let op = op.unwrap_or(ValkyrieTokenType::Bang);

        Ok(Expr::Unary { op, expr, span })
    }

    pub(crate) fn build_call<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<Expr, OakError> {
        let span = node.span();
        let mut callee = None;
        let mut args = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieElementType::Whitespace | ValkyrieElementType::Newline | ValkyrieElementType::LineComment | ValkyrieElementType::BlockComment => continue,
                    ValkyrieElementType::ArgList => {
                        for arg_child in n.children() {
                            if let RedTree::Node(arg_n) = arg_child {
                                if let Ok(arg) = self.build_expr(arg_n, source) {
                                    args.push(arg);
                                }
                            }
                        }
                    }
                    _ => {
                        if callee.is_none() {
                            callee = Some(Box::new(self.build_expr(n, source)?));
                        }
                    }
                },
            }
        }

        let callee = callee.ok_or_else(|| source.syntax_error("Missing callee".to_string(), span.start))?;

        Ok(Expr::Call { callee, args, span })
    }

    pub(crate) fn build_field_expr<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<Expr, OakError> {
        let span = node.span();
        let mut receiver = None;
        let mut field = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    ValkyrieTokenType::Identifier => {
                        if receiver.is_some() && field.is_none() {
                            field = Some(Identifier { name: text(source, t.span), span: t.span });
                        }
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieElementType::Whitespace | ValkyrieElementType::Newline | ValkyrieElementType::LineComment | ValkyrieElementType::BlockComment => continue,
                    _ => {
                        if receiver.is_none() {
                            receiver = Some(Box::new(self.build_expr(n, source)?));
                        }
                    }
                },
            }
        }

        let receiver = receiver.ok_or_else(|| source.syntax_error("Missing receiver".to_string(), span.start))?;
        let field = field.ok_or_else(|| source.syntax_error("Missing field".to_string(), span.start))?;

        Ok(Expr::Field { receiver, field, span })
    }

    pub(crate) fn build_index<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<Expr, OakError> {
        let span = node.span();
        let mut receiver = None;
        let mut index = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieElementType::Whitespace | ValkyrieElementType::Newline | ValkyrieElementType::LineComment | ValkyrieElementType::BlockComment => continue,
                    _ => {
                        if receiver.is_none() {
                            receiver = Some(Box::new(self.build_expr(n, source)?));
                        }
                        else if index.is_none() {
                            index = Some(Box::new(self.build_expr(n, source)?));
                        }
                    }
                },
            }
        }

        let receiver = receiver.ok_or_else(|| source.syntax_error("Missing receiver".to_string(), span.start))?;
        let index = index.ok_or_else(|| source.syntax_error("Missing index".to_string(), span.start))?;

        Ok(Expr::Index { receiver, index, span })
    }

    /// 构建基数索引表达式。
    ///
    /// 基数索引使用 `⁅ ⁆` 括号，表示从 0 开始的偏移量访问。
    /// 与普通索引 `[ ]`（从 1 开始）不同，基数索引更接近底层指针算术风格。
    pub(crate) fn build_offset<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<Expr, OakError> {
        let span = node.span();
        let mut receiver = None;
        let mut offset = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieElementType::Whitespace | ValkyrieElementType::Newline | ValkyrieElementType::LineComment | ValkyrieElementType::BlockComment => continue,
                    _ => {
                        if receiver.is_none() {
                            receiver = Some(Box::new(self.build_expr(n, source)?));
                        }
                        else if offset.is_none() {
                            offset = Some(Box::new(self.build_expr(n, source)?));
                        }
                    }
                },
            }
        }

        let receiver = receiver.ok_or_else(|| source.syntax_error("Missing receiver".to_string(), span.start))?;
        let offset = offset.ok_or_else(|| source.syntax_error("Missing offset".to_string(), span.start))?;

        Ok(Expr::Offset { receiver, offset, span })
    }

    pub(crate) fn build_paren<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<Expr, OakError> {
        let span = node.span();
        let mut expr = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieElementType::Whitespace | ValkyrieElementType::Newline | ValkyrieElementType::LineComment | ValkyrieElementType::BlockComment => continue,
                    _ => {
                        if expr.is_none() {
                            expr = Some(Box::new(self.build_expr(n, source)?));
                        }
                    }
                },
            }
        }

        let expr = expr.ok_or_else(|| source.syntax_error("Missing parenthesized expression".to_string(), span.start))?;

        Ok(Expr::Paren { expr, span })
    }

    pub(crate) fn build_block<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<Block, OakError> {
        let span = node.span();
        let mut statements = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieElementType::Whitespace | ValkyrieElementType::Newline | ValkyrieElementType::LineComment | ValkyrieElementType::BlockComment => continue,
                    ValkyrieElementType::LetStatement => {
                        if let Ok(stmt) = self.build_let(n, source) {
                            statements.push(stmt);
                        }
                    }
                    ValkyrieElementType::ExprStatement => {
                        if let Ok(stmt) = self.build_expr_stmt(n, source) {
                            statements.push(stmt);
                        }
                    }
                    _ => {
                        if let Ok(expr) = self.build_expr(n, source) {
                            statements.push(Statement::ExprStmt { annotations: Vec::new(), expr, semi: false, span: n.span() });
                        }
                    }
                },
            }
        }

        Ok(Block { statements, span })
    }

    pub(crate) fn build_object<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<Expr, OakError> {
        let span = node.span();
        let mut callee = None;
        let mut fields = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieElementType::Whitespace | ValkyrieElementType::Newline | ValkyrieElementType::LineComment | ValkyrieElementType::BlockComment => continue,
                    ValkyrieElementType::BlockExpression => {
                        for block_child in n.children() {
                            if let RedTree::Node(stmt_n) = block_child {
                                match stmt_n.green.kind {
                                    ValkyrieElementType::Whitespace | ValkyrieElementType::Newline | ValkyrieElementType::LineComment | ValkyrieElementType::BlockComment => continue,
                                    ValkyrieElementType::ExprStatement => {
                                        for expr_child in stmt_n.children() {
                                            if let RedTree::Node(expr_n) = expr_child {
                                                match expr_n.green.kind {
                                                    ValkyrieElementType::Whitespace | ValkyrieElementType::Newline | ValkyrieElementType::LineComment | ValkyrieElementType::BlockComment => continue,
                                                    ValkyrieElementType::BinaryExpression => {
                                                        if let Some((name, value)) = self.extract_object_field(&expr_n, source)? {
                                                            fields.push((name, Some(value)));
                                                        }
                                                    }
                                                    ValkyrieElementType::IdentifierExpression => {
                                                        if let Ok(Expr::Ident(ident)) = self.build_identifier_expr(expr_n.clone(), source) {
                                                            fields.push((ident, None));
                                                        }
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
                    }
                    _ => {
                        if callee.is_none() {
                            callee = Some(Box::new(self.build_expr(n, source)?));
                        }
                    }
                },
            }
        }

        let callee = callee.ok_or_else(|| source.syntax_error("Missing object callee".to_string(), span.start))?;

        Ok(Expr::Object { callee, fields, span })
    }

    /// Extracts a field name and value from an object field expression.
    ///
    /// Supports both new syntax (`:` separator) and deprecated syntax (`=` separator).
    /// When the deprecated `=` syntax is detected, a warning is logged.
    fn extract_object_field<S: Source + ?Sized>(&self, node: &RedNode<ValkyrieLanguage>, source: &S) -> Result<Option<(Identifier, Expr)>, OakError> {
        let mut field_name = None;
        let mut value = None;
        let mut separator_found = false;
        let mut uses_deprecated_syntax = false;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    ValkyrieTokenType::Eq => {
                        if !separator_found {
                            separator_found = true;
                            uses_deprecated_syntax = true;
                        }
                    }
                    ValkyrieTokenType::Colon => {
                        if !separator_found {
                            separator_found = true;
                        }
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieElementType::Whitespace | ValkyrieElementType::Newline | ValkyrieElementType::LineComment | ValkyrieElementType::BlockComment => continue,
                    ValkyrieElementType::IdentifierExpression => {
                        if field_name.is_none() {
                            if let Ok(Expr::Ident(ident)) = self.build_identifier_expr(n.clone(), source) {
                                field_name = Some(ident);
                            }
                        }
                    }
                    _ => {
                        if field_name.is_some() && value.is_none() {
                            value = Some(self.build_expr(n, source)?);
                        }
                    }
                },
            }
        }

        if uses_deprecated_syntax {
            if let Some(ref name) = field_name {
                eprintln!(
                    "Warning: Use of deprecated '=' syntax in object field at offset {}. Use ':' instead. Field: '{}'",
                    name.span.start, name.name
                );
            }
        }

        if let (Some(name), Some(val)) = (field_name, value) { Ok(Some((name, val))) } else { Ok(None) }
    }

    pub(crate) fn build_identifier_expr<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<Expr, OakError> {
        let span = node.span();
        let mut name = String::new();
        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == ValkyrieTokenType::Identifier {
                    name = text(source, t.span);
                    return Ok(Expr::Ident(Identifier { name, span: t.span }));
                }
            }
        }
        Ok(Expr::Ident(Identifier { name, span }))
    }

    pub(crate) fn build_path_expr<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<Expr, OakError> {
        let path = self.build_name_path(node, source)?;
        Ok(Expr::Path(path))
    }

    pub(crate) fn build_if<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<Expr, OakError> {
        let span = node.span();
        let mut pattern = None;
        let mut condition = None;
        let mut then_branch = None;
        let mut else_branch = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieElementType::Whitespace | ValkyrieElementType::Newline | ValkyrieElementType::LineComment | ValkyrieElementType::BlockComment => continue,
                    ValkyrieElementType::Pattern => {
                        pattern = Some(self.build_pattern(n, source)?);
                    }
                    ValkyrieElementType::BlockExpression => {
                        if then_branch.is_none() {
                            then_branch = Some(self.build_block(n, source)?);
                        }
                        else {
                            else_branch = Some(self.build_block(n, source)?);
                        }
                    }
                    _ => {
                        if condition.is_none() {
                            condition = Some(Box::new(self.build_expr(n, source)?));
                        }
                    }
                },
            }
        }

        let condition = condition.ok_or_else(|| source.syntax_error("Missing if condition".to_string(), span.start))?;
        let then_branch = then_branch.ok_or_else(|| source.syntax_error("Missing if then branch".to_string(), span.start))?;

        Ok(Expr::If { pattern, condition, then_branch, else_branch, span })
    }

    pub(crate) fn build_match<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<Expr, OakError> {
        let span = node.span();
        let mut scrutinee = None;
        let mut arms = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieElementType::Whitespace | ValkyrieElementType::Newline | ValkyrieElementType::LineComment | ValkyrieElementType::BlockComment => continue,
                    ValkyrieElementType::MatchArm => {
                        if let Ok(arm) = self.build_match_arm(n, source) {
                            arms.push(arm);
                        }
                    }
                    _ => {
                        if scrutinee.is_none() {
                            scrutinee = Some(Box::new(self.build_expr(n, source)?));
                        }
                    }
                },
            }
        }

        let scrutinee = scrutinee.ok_or_else(|| source.syntax_error("Missing match scrutinee".to_string(), span.start))?;

        Ok(Expr::Match { scrutinee, arms, span })
    }

    pub(crate) fn build_match_arm<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<MatchArm, OakError> {
        let span = node.span();
        let mut pattern = None;
        let mut guard = None;
        let mut body = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieElementType::Whitespace | ValkyrieElementType::Newline | ValkyrieElementType::LineComment | ValkyrieElementType::BlockComment => continue,
                    ValkyrieElementType::Pattern => {
                        pattern = Some(self.build_pattern(n, source)?);
                    }
                    _ => {
                        if body.is_none() {
                            body = Some(self.build_expr(n, source)?);
                        }
                        else if guard.is_none() {
                            guard = Some(self.build_expr(n, source)?);
                        }
                    }
                },
            }
        }

        let pattern = pattern.ok_or_else(|| source.syntax_error("Missing match arm pattern".to_string(), span.start))?;
        let body = body.ok_or_else(|| source.syntax_error("Missing match arm body".to_string(), span.start))?;

        Ok(MatchArm { pattern, guard, body, span })
    }

    pub(crate) fn build_pattern<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<Pattern, OakError> {
        let span = node.span();
        let mut name_path: Option<NamePath> = None;
        let mut fields: Option<Vec<(Identifier, Option<Pattern>)>> = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    ValkyrieTokenType::Underscore => {
                        if name_path.is_none() && fields.is_none() {
                            return Ok(Pattern::Wildcard { span: t.span });
                        }
                    }
                    ValkyrieTokenType::Identifier => {
                        if name_path.is_none() && fields.is_none() {
                            let name = text(source, t.span);
                            return Ok(Pattern::Variable { name: Identifier { name, span: t.span }, span: t.span });
                        }
                    }
                    ValkyrieTokenType::IntegerLiteral | ValkyrieTokenType::FloatLiteral | ValkyrieTokenType::StringLiteral => {
                        if name_path.is_none() && fields.is_none() {
                            let value = text(source, t.span);
                            return Ok(Pattern::Literal { value, span: t.span });
                        }
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieElementType::Whitespace | ValkyrieElementType::Newline | ValkyrieElementType::LineComment | ValkyrieElementType::BlockComment => continue,
                    ValkyrieElementType::NamePath => {
                        if name_path.is_none() {
                            name_path = Some(self.build_name_path(n, source)?);
                        }
                    }
                    ValkyrieElementType::BlockExpression => {
                        if name_path.is_some() && fields.is_none() {
                            fields = Some(self.build_pattern_fields(&n, source)?);
                        }
                    }
                    _ => {}
                },
            }
        }

        match (name_path, fields) {
            (Some(name), Some(fields)) => Ok(Pattern::Class { name, fields, span }),
            (Some(name), None) => Ok(Pattern::Type { name, span }),
            _ => Ok(Pattern::Wildcard { span }),
        }
    }

    /// Builds pattern fields from a block expression.
    ///
    /// Supports both new syntax (`:` separator) and deprecated syntax (`=` separator).
    /// When the deprecated `=` syntax is detected, a warning is logged.
    fn build_pattern_fields<S: Source + ?Sized>(&self, node: &RedNode<ValkyrieLanguage>, source: &S) -> Result<Vec<(Identifier, Option<Pattern>)>, OakError> {
        let mut fields = Vec::new();

        for child in node.children() {
            if let RedTree::Node(stmt_n) = child {
                match stmt_n.green.kind {
                    ValkyrieElementType::Whitespace | ValkyrieElementType::Newline | ValkyrieElementType::LineComment | ValkyrieElementType::BlockComment => continue,
                    ValkyrieElementType::ExprStatement | ValkyrieElementType::BinaryExpression => {
                        if let Some(field) = self.extract_pattern_field(&stmt_n, source)? {
                            fields.push(field);
                        }
                    }
                    ValkyrieElementType::IdentifierExpression => {
                        if let Ok(Expr::Ident(ident)) = self.build_identifier_expr(stmt_n.clone(), source) {
                            fields.push((ident, None));
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(fields)
    }

    /// Extracts a pattern field from a binary expression or statement.
    ///
    /// Supports both new syntax (`:` separator) and deprecated syntax (`=` separator).
    fn extract_pattern_field<S: Source + ?Sized>(&self, node: &RedNode<ValkyrieLanguage>, source: &S) -> Result<Option<(Identifier, Option<Pattern>)>, OakError> {
        let mut field_name = None;
        let mut field_pattern = None;
        let mut separator_found = false;
        let mut uses_deprecated_syntax = false;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    ValkyrieTokenType::Eq => {
                        if !separator_found {
                            separator_found = true;
                            uses_deprecated_syntax = true;
                        }
                    }
                    ValkyrieTokenType::Colon => {
                        if !separator_found {
                            separator_found = true;
                        }
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieElementType::Whitespace | ValkyrieElementType::Newline | ValkyrieElementType::LineComment | ValkyrieElementType::BlockComment => continue,
                    ValkyrieElementType::IdentifierExpression => {
                        if field_name.is_none() {
                            if let Ok(Expr::Ident(ident)) = self.build_identifier_expr(n.clone(), source) {
                                field_name = Some(ident);
                            }
                        }
                    }
                    ValkyrieElementType::Pattern => {
                        if field_name.is_some() && field_pattern.is_none() {
                            field_pattern = Some(self.build_pattern(n, source)?);
                        }
                    }
                    _ => {
                        if field_name.is_some() && field_pattern.is_none() {
                            if let Ok(pattern) = self.build_pattern(n, source) {
                                field_pattern = Some(pattern);
                            }
                        }
                    }
                },
            }
        }

        if uses_deprecated_syntax {
            if let Some(ref name) = field_name {
                eprintln!(
                    "Warning: Use of deprecated '=' syntax in pattern field at offset {}. Use ':' instead. Field: '{}'",
                    name.span.start, name.name
                );
            }
        }

        if let Some(name) = field_name { Ok(Some((name, field_pattern))) } else { Ok(None) }
    }

    pub(crate) fn build_loop<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<Expr, OakError> {
        let span = node.span();
        let mut kind = LoopKind::default();
        let mut label = None;
        let mut pattern = None;
        let mut condition = None;
        let mut body = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    ValkyrieTokenType::Label => {
                        label = Some(text(source, t.span));
                    }
                    ValkyrieTokenType::Keyword(ValkyrieKeywords::For) => {
                        kind = LoopKind::For;
                    }
                    ValkyrieTokenType::Keyword(ValkyrieKeywords::Loop) => {
                        kind = LoopKind::Loop;
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieElementType::Whitespace | ValkyrieElementType::Newline | ValkyrieElementType::LineComment | ValkyrieElementType::BlockComment => continue,
                    ValkyrieElementType::Pattern => {
                        pattern = Some(self.build_pattern(n, source)?);
                    }
                    ValkyrieElementType::BlockExpression => {
                        body = Some(self.build_block(n, source)?);
                    }
                    _ => {
                        if condition.is_none() {
                            condition = Some(Box::new(self.build_expr(n, source)?));
                        }
                    }
                },
            }
        }

        let body = body.ok_or_else(|| source.syntax_error("Missing loop body".to_string(), span.start))?;

        Ok(Expr::Loop { kind, label, pattern, condition, body, span })
    }

    pub(crate) fn build_return<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<Expr, OakError> {
        let span = node.span();
        let mut expr = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieElementType::Whitespace | ValkyrieElementType::Newline | ValkyrieElementType::LineComment | ValkyrieElementType::BlockComment => continue,
                    _ => {
                        if expr.is_none() {
                            expr = Some(Box::new(self.build_expr(n, source)?));
                        }
                    }
                },
            }
        }

        Ok(Expr::Return { expr, span })
    }

    pub(crate) fn build_break<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<Expr, OakError> {
        let span = node.span();
        let mut label = None;
        let mut expr = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    ValkyrieTokenType::Label => {
                        label = Some(text(source, t.span));
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieElementType::Whitespace | ValkyrieElementType::Newline | ValkyrieElementType::LineComment | ValkyrieElementType::BlockComment => continue,
                    _ => {
                        if expr.is_none() {
                            expr = Some(Box::new(self.build_expr(n, source)?));
                        }
                    }
                },
            }
        }

        Ok(Expr::Break { label, expr, span })
    }

    pub(crate) fn build_continue<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<Expr, OakError> {
        let span = node.span();
        let mut label = None;

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    ValkyrieTokenType::Label => {
                        label = Some(text(source, t.span));
                    }
                    _ => {}
                }
            }
        }

        Ok(Expr::Continue { label, span })
    }

    pub(crate) fn build_yield<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<Expr, OakError> {
        let span = node.span();
        let mut expr = None;
        let yield_from = false;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieElementType::Whitespace | ValkyrieElementType::Newline | ValkyrieElementType::LineComment | ValkyrieElementType::BlockComment => continue,
                    _ => {
                        if expr.is_none() {
                            expr = Some(Box::new(self.build_expr(n, source)?));
                        }
                    }
                },
            }
        }

        Ok(Expr::Yield { expr, yield_from, span })
    }

    pub(crate) fn build_raise<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<Expr, OakError> {
        let span = node.span();
        let mut expr = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieElementType::Whitespace | ValkyrieElementType::Newline | ValkyrieElementType::LineComment | ValkyrieElementType::BlockComment => continue,
                    _ => {
                        if expr.is_none() {
                            expr = Some(Box::new(self.build_expr(n, source)?));
                        }
                    }
                },
            }
        }

        let expr = expr.ok_or_else(|| source.syntax_error("Missing raise expression".to_string(), span.start))?;

        Ok(Expr::Raise { expr, span })
    }

    pub(crate) fn build_catch<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<Expr, OakError> {
        let span = node.span();
        let mut expr = None;
        let mut arms = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieElementType::Whitespace | ValkyrieElementType::Newline | ValkyrieElementType::LineComment | ValkyrieElementType::BlockComment => continue,
                    ValkyrieElementType::MatchArm => arms.push(self.build_match_arm(n, source)?),
                    _ => {
                        if expr.is_none() {
                            expr = Some(Box::new(self.build_expr(n, source)?));
                        }
                    }
                },
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    _ => {}
                },
            }
        }

        let expr = expr.ok_or_else(|| source.syntax_error("Missing catch expression".to_string(), span.start))?;

        Ok(Expr::Catch { expr, arms, span })
    }

    pub(crate) fn build_name_path<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<NamePath, OakError> {
        let span = node.span();
        let mut parts = Vec::new();

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    ValkyrieTokenType::Identifier => parts.push(Identifier { name: text(source, t.span), span: t.span }),
                    _ => {}
                }
            }
        }
        Ok(NamePath { parts, span })
    }

    /// Builds an anonymous class expression.
    ///
    /// Syntax: `class { ... }` or `class: Trait { ... }`
    pub(crate) fn build_anonymous_class<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<Expr, OakError> {
        let span = node.span();
        let mut parents = Vec::new();
        let mut items = Vec::new();
        let mut captures = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    ValkyrieTokenType::Identifier => {
                        parents.push(text(source, t.span));
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieElementType::Whitespace | ValkyrieElementType::Newline | ValkyrieElementType::LineComment | ValkyrieElementType::BlockComment => continue,
                    ValkyrieElementType::NamePath => {
                        let path = self.build_name_path(n, source)?;
                        if let Some(first) = path.parts.first() {
                            parents.push(first.name.clone());
                        }
                    }
                    ValkyrieElementType::BlockExpression => {
                        for inner_child in n.children() {
                            if let RedTree::Node(inner_n) = inner_child {
                                if let Ok(item) = self.build_item(inner_n, source) {
                                    items.push(item);
                                }
                            }
                        }
                    }
                    ValkyrieElementType::LetStatement => {
                        let stmt = self.build_let(n, source)?;
                        items.push(Item::Statement(stmt));
                    }
                    ValkyrieElementType::ExprStatement => {
                        let stmt = self.build_expr_stmt(n, source)?;
                        items.push(Item::Statement(stmt));
                    }
                    ValkyrieElementType::Micro => {
                        let micro = self.build_micro(n, source)?;
                        items.push(Item::Micro(micro));
                    }
                    _ => {}
                },
            }
        }

        Ok(Expr::AnonymousClass { parents, items, captures, span })
    }
}
