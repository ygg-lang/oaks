use crate::{
    ValkyrieLanguage,
    ast::{
        pattern_nodes::{ClassPattern, LiteralPattern, TypePattern, VariablePattern, WildcardPattern},
        term_nodes::{Break, Continue, Raise, Resume, Return},
        *,
    },
    builder::ValkyrieBuilder,
    lexer::{ValkyrieKeywords, token_type::ValkyrieTokenType},
};
use oak_core::{OakError, RedNode, RedTree, Source};

impl<'config> ValkyrieBuilder<'config> {
    pub(crate) fn build_if<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<TermExpression, OakError> {
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
                    crate::parser::element_type::ValkyrieElementType::Whitespace
                    | crate::parser::element_type::ValkyrieElementType::Newline
                    | crate::parser::element_type::ValkyrieElementType::LineComment
                    | crate::parser::element_type::ValkyrieElementType::BlockComment => continue,
                    crate::parser::element_type::ValkyrieElementType::Pattern => {
                        pattern = Some(self.build_pattern(n, source)?);
                    }
                    crate::parser::element_type::ValkyrieElementType::BlockExpression => {
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

        Ok(TermExpression::If { pattern, condition, then_branch, else_branch, span })
    }

    pub(crate) fn build_match<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<TermExpression, OakError> {
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
                    crate::parser::element_type::ValkyrieElementType::Whitespace
                    | crate::parser::element_type::ValkyrieElementType::Newline
                    | crate::parser::element_type::ValkyrieElementType::LineComment
                    | crate::parser::element_type::ValkyrieElementType::BlockComment => continue,
                    crate::parser::element_type::ValkyrieElementType::MatchArm => {
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

        Ok(TermExpression::Match { scrutinee, arms, span })
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
                    crate::parser::element_type::ValkyrieElementType::Whitespace
                    | crate::parser::element_type::ValkyrieElementType::Newline
                    | crate::parser::element_type::ValkyrieElementType::LineComment
                    | crate::parser::element_type::ValkyrieElementType::BlockComment => continue,
                    crate::parser::element_type::ValkyrieElementType::Pattern => {
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

    pub(crate) fn build_loop<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<TermExpression, OakError> {
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
                        label = Some(crate::builder::text(source, t.span));
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
                    crate::parser::element_type::ValkyrieElementType::Whitespace
                    | crate::parser::element_type::ValkyrieElementType::Newline
                    | crate::parser::element_type::ValkyrieElementType::LineComment
                    | crate::parser::element_type::ValkyrieElementType::BlockComment => continue,
                    crate::parser::element_type::ValkyrieElementType::Pattern => {
                        pattern = Some(self.build_pattern(n, source)?);
                    }
                    crate::parser::element_type::ValkyrieElementType::BlockExpression => {
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

        Ok(TermExpression::Loop { kind, label, pattern, condition, body, span })
    }

    pub(crate) fn build_return<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<TermExpression, OakError> {
        let span = node.span();
        let mut expr = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    crate::parser::element_type::ValkyrieElementType::Whitespace
                    | crate::parser::element_type::ValkyrieElementType::Newline
                    | crate::parser::element_type::ValkyrieElementType::LineComment
                    | crate::parser::element_type::ValkyrieElementType::BlockComment => continue,
                    _ => {
                        if expr.is_none() {
                            expr = Some(Box::new(self.build_expr(n, source)?));
                        }
                    }
                },
            }
        }

        Ok(TermExpression::Return(Box::new(Return { base: expr.map(|e| *e), span })))
    }

    pub(crate) fn build_break<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<TermExpression, OakError> {
        let span = node.span();
        let mut label = None;
        let mut expr = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    ValkyrieTokenType::Label => {
                        label = Some(crate::builder::text(source, t.span));
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    crate::parser::element_type::ValkyrieElementType::Whitespace
                    | crate::parser::element_type::ValkyrieElementType::Newline
                    | crate::parser::element_type::ValkyrieElementType::LineComment
                    | crate::parser::element_type::ValkyrieElementType::BlockComment => continue,
                    _ => {
                        if expr.is_none() {
                            expr = Some(Box::new(self.build_expr(n, source)?));
                        }
                    }
                },
            }
        }

        Ok(TermExpression::Break(Box::new(Break { label, base: expr.map(|e| *e), span })))
    }

    pub(crate) fn build_continue<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<TermExpression, OakError> {
        let span = node.span();
        let mut label = None;

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    ValkyrieTokenType::Label => {
                        label = Some(crate::builder::text(source, t.span));
                    }
                    _ => {}
                }
            }
        }

        Ok(TermExpression::Continue(Box::new(Continue { label, span })))
    }

    pub(crate) fn build_yield<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<TermExpression, OakError> {
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
                    crate::parser::element_type::ValkyrieElementType::Whitespace
                    | crate::parser::element_type::ValkyrieElementType::Newline
                    | crate::parser::element_type::ValkyrieElementType::LineComment
                    | crate::parser::element_type::ValkyrieElementType::BlockComment => continue,
                    _ => {
                        if expr.is_none() {
                            expr = Some(Box::new(self.build_expr(n, source)?));
                        }
                    }
                },
            }
        }

        Ok(TermExpression::Yield { expr, yield_from, span })
    }

    pub(crate) fn build_raise<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<TermExpression, OakError> {
        let span = node.span();
        let mut expr = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    crate::parser::element_type::ValkyrieElementType::Whitespace
                    | crate::parser::element_type::ValkyrieElementType::Newline
                    | crate::parser::element_type::ValkyrieElementType::LineComment
                    | crate::parser::element_type::ValkyrieElementType::BlockComment => continue,
                    _ => {
                        if expr.is_none() {
                            expr = Some(Box::new(self.build_expr(n, source)?));
                        }
                    }
                },
            }
        }

        let expr = expr.ok_or_else(|| source.syntax_error("Missing raise expression".to_string(), span.start))?;

        Ok(TermExpression::Raise(Box::new(Raise { base: Some(*expr), span })))
    }

    pub(crate) fn build_resume<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<TermExpression, OakError> {
        let span = node.span();
        let mut expr = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    crate::parser::element_type::ValkyrieElementType::Whitespace
                    | crate::parser::element_type::ValkyrieElementType::Newline
                    | crate::parser::element_type::ValkyrieElementType::LineComment
                    | crate::parser::element_type::ValkyrieElementType::BlockComment => continue,
                    _ => {
                        if expr.is_none() {
                            expr = Some(Box::new(self.build_expr(n, source)?));
                        }
                    }
                },
            }
        }

        let expr = expr.ok_or_else(|| source.syntax_error("Missing resume expression".to_string(), span.start))?;

        Ok(TermExpression::Resume(Box::new(Resume { base: Some(*expr), span })))
    }

    pub(crate) fn build_catch<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<TermExpression, OakError> {
        let span = node.span();
        let mut expr = None;
        let mut arms = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    crate::parser::element_type::ValkyrieElementType::Whitespace
                    | crate::parser::element_type::ValkyrieElementType::Newline
                    | crate::parser::element_type::ValkyrieElementType::LineComment
                    | crate::parser::element_type::ValkyrieElementType::BlockComment => continue,
                    crate::parser::element_type::ValkyrieElementType::MatchArm => arms.push(self.build_match_arm(n, source)?),
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

        Ok(TermExpression::Catch { expr, arms, span })
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
                            return Ok(Pattern::Wildcard(Box::new(WildcardPattern { span: t.span })));
                        }
                    }
                    ValkyrieTokenType::Identifier => {
                        if name_path.is_none() && fields.is_none() {
                            let name = crate::builder::text(source, t.span);
                            return Ok(Pattern::Variable(Box::new(VariablePattern { name: Identifier { name, span: t.span }, span: t.span })));
                        }
                    }
                    ValkyrieTokenType::IntegerLiteral | ValkyrieTokenType::FloatLiteral | ValkyrieTokenType::StringLiteral => {
                        if name_path.is_none() && fields.is_none() {
                            let value = crate::builder::text(source, t.span);
                            return Ok(Pattern::Literal(Box::new(LiteralPattern { value, span: t.span })));
                        }
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    crate::parser::element_type::ValkyrieElementType::Whitespace
                    | crate::parser::element_type::ValkyrieElementType::Newline
                    | crate::parser::element_type::ValkyrieElementType::LineComment
                    | crate::parser::element_type::ValkyrieElementType::BlockComment => continue,
                    crate::parser::element_type::ValkyrieElementType::NamePath => {
                        if name_path.is_none() {
                            name_path = Some(self.build_name_path(n, source)?);
                        }
                    }
                    crate::parser::element_type::ValkyrieElementType::BlockExpression => {
                        if name_path.is_some() && fields.is_none() {
                            fields = Some(self.build_pattern_fields(&n, source)?);
                        }
                    }
                    _ => {}
                },
            }
        }

        match (name_path, fields) {
            (Some(name), Some(fields)) => Ok(Pattern::Class(Box::new(ClassPattern { name, fields, span }))),
            (Some(name), None) => Ok(Pattern::Type(Box::new(TypePattern { name, span }))),
            _ => Ok(Pattern::Wildcard(Box::new(WildcardPattern { span }))),
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
                    crate::parser::element_type::ValkyrieElementType::Whitespace
                    | crate::parser::element_type::ValkyrieElementType::Newline
                    | crate::parser::element_type::ValkyrieElementType::LineComment
                    | crate::parser::element_type::ValkyrieElementType::BlockComment => continue,
                    crate::parser::element_type::ValkyrieElementType::ExprStatement | crate::parser::element_type::ValkyrieElementType::BinaryExpression => {
                        if let Some(field) = self.extract_pattern_field(&stmt_n, source)? {
                            fields.push(field);
                        }
                    }
                    crate::parser::element_type::ValkyrieElementType::IdentifierExpression => {
                        if let Ok(expr) = self.build_identifier_expr(stmt_n.clone(), source) {
                            if let TermExpression::NamePath(path) = expr {
                                if let Some(ident) = path.parts.first().cloned() {
                                    fields.push((ident, None));
                                }
                            }
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
                    crate::parser::element_type::ValkyrieElementType::Whitespace
                    | crate::parser::element_type::ValkyrieElementType::Newline
                    | crate::parser::element_type::ValkyrieElementType::LineComment
                    | crate::parser::element_type::ValkyrieElementType::BlockComment => continue,
                    crate::parser::element_type::ValkyrieElementType::IdentifierExpression => {
                        if field_name.is_none() {
                            if let Ok(expr) = self.build_identifier_expr(n.clone(), source) {
                                if let TermExpression::NamePath(path) = expr {
                                    if let Some(ident) = path.parts.first().cloned() {
                                        field_name = Some(ident);
                                    }
                                }
                            }
                        }
                    }
                    crate::parser::element_type::ValkyrieElementType::Pattern => {
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
                eprintln!("Warning: Use of deprecated '=' syntax in pattern field at offset {}. Use ':' instead. Field: '{}'", name.span.start, name.name);
            }
        }

        if let Some(name) = field_name { Ok(Some((name, field_pattern))) } else { Ok(None) }
    }
}
