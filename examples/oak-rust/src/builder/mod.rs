use crate::{
    RustLanguage, RustParser, RustSyntaxKind,
    ast::{RustRoot, *},
};
use oak_core::{
    Arc, Builder, GreenNode, IncrementalCache, OakDiagnostics, OakError, RedNode, RedTree, SourceText, source::Source,
};
use std::range::Range;

impl<'config> Builder<RustLanguage> for RustParser<'config> {
    fn build_incremental(
        &self,
        text: impl Source,
        changed: usize,
        cache: IncrementalCache<RustLanguage>,
    ) -> OakDiagnostics<RustRoot> {
        todo!()
    }
}

impl<'config> RustParser<'config> {
    pub(crate) fn build_root(&self, green_tree: GreenNode<RustSyntaxKind>, source: &SourceText) -> Result<RustRoot, OakError> {
        let red_root = RedNode::new(Arc::new(green_tree), 0);
        let mut items = Vec::new();
        for child in red_root.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    RustSyntaxKind::Function => {
                        let func = self.build_function(n, source)?;
                        items.push(Item::Function(func));
                    }
                    RustSyntaxKind::LetStatement => {
                        let stmt = self.build_let(n, source)?;
                        items.push(Item::Statement(stmt));
                    }
                    RustSyntaxKind::ExpressionStatement => {
                        let stmt = self.parse_expr_stmt(n, source)?;
                        items.push(Item::Statement(stmt));
                    }
                    _ => {
                        return Err(OakError::syntax_error(
                            "Unexpected item in root".to_string(),
                            source.get_location(n.span().start),
                        ));
                    }
                },
                RedTree::Leaf(t) => {
                    return Err(OakError::syntax_error(
                        "Unexpected token in root".to_string(),
                        source.get_location(t.span.start),
                    ));
                }
            }
        }
        Ok(RustRoot { items })
    }

    /// 从红绿树提取强类型 AST

    pub(crate) fn build_function(&self, node: RedNode<RustSyntaxKind>, source: &SourceText) -> Result<Function, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut params = Vec::new();
        let mut body = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == RustSyntaxKind::Identifier {
                        name.name = text(source, t.span.clone());
                        name.span = t.span.clone();
                    }
                }
                RedTree::Node(n) => match n.green.kind {
                    RustSyntaxKind::ParameterList => {
                        params = self.build_param_list(n, source)?;
                    }
                    RustSyntaxKind::BlockExpression => {
                        body = Some(self.build_block(n, source)?);
                    }
                    _ => {
                        return Err(OakError::syntax_error(
                            "Unexpected node in function definition".to_string(),
                            source.get_location(n.span().start),
                        ));
                    }
                },
            }
        }
        if let Some(body) = body {
            Ok(Function { name, params, body, span })
        }
        else {
            Err(OakError::syntax_error(format!("Missing function body at {:?}", span), source.get_location(span.start)))
        }
    }

    fn build_param_list(&self, node: RedNode<RustSyntaxKind>, source: &SourceText) -> Result<Vec<Param>, OakError> {
        let mut params = Vec::new();
        for child in node.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == RustSyntaxKind::Parameter {
                    params.push(self.parse_param(n, source)?);
                }
            }
        }
        Ok(params)
    }

    fn parse_param(&self, node: RedNode<RustSyntaxKind>, source: &SourceText) -> Result<Param, OakError> {
        let span = node.span();
        let mut name: Option<Identifier> = None;
        let mut ty = None;
        // children: IDENT ':' IDENT
        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == RustSyntaxKind::Identifier {
                        if name.is_none() {
                            name = Some(Identifier { name: text(source, t.span.clone()), span: t.span.clone() });
                        }
                        else {
                            ty = Some(text(source, t.span.clone()));
                        }
                    }
                    else if t.kind != RustSyntaxKind::Colon {
                        return Err(OakError::syntax_error(
                            "Unexpected token in parameter definition".to_string(),
                            source.get_location(t.span.start),
                        ));
                    }
                }
                _ => {
                    return Err(OakError::syntax_error(
                        "Unexpected token in parameter definition".to_string(),
                        source.get_location(child.span().start),
                    ));
                }
            }
        }
        return if let (Some(name), Some(ty)) = (name, ty) {
            Ok(Param { name, ty, span })
        }
        else {
            Err(OakError::syntax_error(
                format!("Missing name or type in parameter at {:?}", span),
                source.get_location(span.start),
            ))
        };
    }

    fn build_block(&self, node: RedNode<RustSyntaxKind>, source: &SourceText) -> Result<Block, OakError> {
        let span = node.span();
        let mut statements = Vec::new();
        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    RustSyntaxKind::LetStatement => statements.push(self.build_let(n, source)?),
                    RustSyntaxKind::ExpressionStatement => statements.push(self.parse_expr_stmt(n, source)?),
                    _ => {
                        return Err(OakError::syntax_error(
                            "Unexpected statement in block".to_string(),
                            source.get_location(n.span().start),
                        ));
                    }
                },
                RedTree::Leaf(t) => {
                    if t.kind != RustSyntaxKind::LeftBrace && t.kind != RustSyntaxKind::RightBrace {
                        return Err(OakError::syntax_error(
                            "Unexpected token in block".to_string(),
                            source.get_location(t.span.start),
                        ));
                    }
                }
            }
        }
        Ok(Block { statements, span })
    }

    fn parse_expr_stmt(&self, node: RedNode<RustSyntaxKind>, source: &SourceText) -> Result<Statement, OakError> {
        let span = node.span();
        let mut children_iter = node.children().peekable();

        let expr_node = children_iter.next().ok_or_else(|| {
            OakError::syntax_error("Missing expression in expression statement".to_string(), source.get_location(span.start))
        })?;

        let expr = match expr_node {
            RedTree::Node(n) => RustParser::parse_expr(self, n, source)?,
            RedTree::Leaf(t) => {
                return Err(OakError::syntax_error(
                    "Expected an expression, found a token".to_string(),
                    source.get_location(t.span.start),
                ));
            }
        };

        let mut semi = false;
        if let Some(RedTree::Leaf(t)) = children_iter.peek() {
            if t.kind == RustSyntaxKind::Semicolon {
                semi = true;
                children_iter.next(); // Consume the semicolon
            }
        }

        if let Some(unexpected_child) = children_iter.next() {
            return Err(OakError::syntax_error(
                "Unexpected token or expression after semicolon".to_string(),
                source.get_location(unexpected_child.span().start),
            ));
        }

        Ok(Statement::ExprStmt { expr, semi, span })
    }

    fn build_let(&self, node: RedNode<RustSyntaxKind>, source: &SourceText) -> Result<Statement, OakError> {
        let span = node.span();
        let mut children_iter = node.children().peekable();

        // Expect 'let' keyword
        let let_keyword = children_iter
            .next()
            .ok_or_else(|| OakError::syntax_error("Missing 'let' keyword".to_string(), source.get_location(span.start)))?;
        match let_keyword {
            RedTree::Leaf(t) if t.kind == RustSyntaxKind::Let => {}
            _ => {
                return Err(OakError::syntax_error(
                    "Expected 'let' keyword".to_string(),
                    source.get_location(let_keyword.span().start),
                ));
            }
        }

        // Expect identifier
        let name_node = children_iter.next().ok_or_else(|| {
            OakError::syntax_error("Missing identifier in let statement".to_string(), source.get_location(span.start))
        })?;
        let name = match name_node {
            RedTree::Leaf(t) if t.kind == RustSyntaxKind::Identifier => {
                Identifier { name: text(source, t.span.clone()), span: t.span.clone() }
            }
            _ => {
                return Err(OakError::syntax_error(
                    "Expected identifier in let statement".to_string(),
                    source.get_location(name_node.span().start),
                ));
            }
        };

        let mut expr: Option<Expr> = None;

        // Check for optional '=' and expression
        if let Some(RedTree::Leaf(t)) = children_iter.peek() {
            if t.kind == RustSyntaxKind::Eq {
                children_iter.next(); // Consume '=' token

                let expr_node = children_iter.next().ok_or_else(|| {
                    OakError::syntax_error(
                        "Missing expression after '=' in let statement".to_string(),
                        source.get_location(span.end),
                    )
                })?;

                expr = Some(match expr_node {
                    RedTree::Node(n) => RustParser::parse_expr(self, n, source)?,
                    RedTree::Leaf(t) => {
                        return Err(OakError::syntax_error(
                            "Expected an expression, found a token after '=' in let statement".to_string(),
                            source.get_location(t.span.start),
                        ));
                    }
                });
            }
        }

        if let Some(unexpected_child) = children_iter.next() {
            return Err(OakError::syntax_error(
                "Unexpected token or expression in let statement".to_string(),
                source.get_location(unexpected_child.span().start),
            ));
        }

        let expr = expr.ok_or_else(|| {
            OakError::syntax_error("Missing expression in let statement".to_string(), source.get_location(span.start))
        })?;

        Ok(Statement::Let { name, expr, span })
    }

    pub(crate) fn parse_expr(&self, node: RedNode<RustSyntaxKind>, source: &SourceText) -> Result<Expr, OakError> {
        match node.green.kind {
            RustSyntaxKind::IdentifierExpression => {
                let span = node.span();
                // child: IDENT
                for child in node.children() {
                    if let RedTree::Leaf(t) = child {
                        if t.kind == RustSyntaxKind::Identifier {
                            return Ok(Expr::Ident(Identifier { name: text(source, t.span.clone()), span: t.span.clone() }));
                        }
                    }
                }
                Err(OakError::syntax_error(
                    format!("Missing identifier in identifier expression at {:?}", span),
                    source.get_location(span.start),
                ))
            }
            RustSyntaxKind::LiteralExpression => {
                let span = node.span();
                for child in node.children() {
                    if let RedTree::Leaf(t) = child {
                        return Ok(Expr::Literal { value: text(source, t.span), span });
                    }
                }
                Err(OakError::syntax_error(
                    format!("Missing literal in literal expression at {:?}", span),
                    source.get_location(span.start),
                ))
            }
            RustSyntaxKind::BooleanLiteral => {
                let span = node.span();
                for child in node.children() {
                    if let RedTree::Leaf(t) = child {
                        return Ok(Expr::Bool { value: t.kind == RustSyntaxKind::True, span });
                    }
                }
                Err(OakError::syntax_error(
                    format!("Missing boolean literal in boolean literal expression at {:?}", span),
                    source.get_location(span.start),
                ))
            }
            RustSyntaxKind::ParenthesizedExpression => {
                let span = node.span();
                for child in node.children() {
                    if let RedTree::Node(n) = child {
                        return Ok(Expr::Paren { expr: Box::new(RustParser::parse_expr(self, n, source)?), span });
                    }
                }
                Err(OakError::syntax_error(
                    format!("Missing expression in parenthesized expression at {:?}", span),
                    source.get_location(span.start),
                ))
            }
            RustSyntaxKind::UnaryExpression => {
                let span = node.span();
                let mut op = RustSyntaxKind::Error;
                let mut sub: Option<Expr> = None;
                for child in node.children() {
                    match child {
                        RedTree::Leaf(t) => {
                            op = t.kind;
                        }
                        RedTree::Node(n) => {
                            sub = Some(RustParser::parse_expr(self, n, source)?);
                        }
                    }
                }
                if let Some(sub_expr) = sub {
                    Ok(Expr::Unary { op, expr: Box::new(sub_expr), span })
                }
                else {
                    Err(OakError::syntax_error(
                        format!("Missing operand in unary expression at {:?}", span),
                        source.get_location(span.start),
                    ))
                }
            }
            RustSyntaxKind::BinaryExpression => {
                let span = node.span();
                let mut left: Option<Expr> = None;
                let mut op = RustSyntaxKind::Error;
                let mut right: Option<Expr> = None;
                let mut idx = 0;
                for child in node.children() {
                    match child {
                        RedTree::Node(n) => {
                            if idx == 0 {
                                left = Some(RustParser::parse_expr(self, n, source)?);
                            }
                            else {
                                right = Some(RustParser::parse_expr(self, n, source)?);
                            }
                        }
                        RedTree::Leaf(t) => {
                            op = t.kind;
                        }
                    }
                    idx += 1;
                }
                if let (Some(left_expr), Some(right_expr)) = (left, right) {
                    Ok(Expr::Binary { left: Box::new(left_expr), op, right: Box::new(right_expr), span })
                }
                else {
                    Err(OakError::syntax_error(
                        format!("Missing operands in binary expression at {:?}", span),
                        source.get_location(span.start),
                    ))
                }
            }
            RustSyntaxKind::CallExpression => {
                let span = node.span();
                // children: callee '(' args... ')' with commas
                let mut callee: Option<Expr> = None;
                let mut args: Vec<Expr> = Vec::new();
                let mut seen_paren = false;
                for child in node.children() {
                    match child {
                        RedTree::Node(n) => {
                            if !seen_paren && callee.is_none() {
                                callee = Some(RustParser::parse_expr(self, n, source)?);
                            }
                            else {
                                args.push(RustParser::parse_expr(self, n, source)?);
                            }
                        }
                        RedTree::Leaf(t) => {
                            if t.kind == RustSyntaxKind::LeftParen {
                                seen_paren = true;
                            }
                        }
                    }
                }
                if let Some(callee_expr) = callee {
                    Ok(Expr::Call { callee: Box::new(callee_expr), args, span })
                }
                else {
                    Err(OakError::syntax_error(
                        format!("Missing callee in call expression at {:?}", span),
                        source.get_location(span.start),
                    ))
                }
            }
            RustSyntaxKind::FieldExpression => {
                let span = node.span();
                let mut receiver: Option<Expr> = None;
                let mut field: Option<Identifier> = None;
                let mut idx = 0;
                for child in node.children() {
                    match child {
                        RedTree::Node(n) => {
                            if idx == 0 {
                                // The first node is the receiver
                                receiver = Some(RustParser::parse_expr(self, n, source)?);
                            }
                        }
                        RedTree::Leaf(t) => {
                            if idx == 2 && t.kind == RustSyntaxKind::Identifier {
                                // The third child (leaf) is the identifier
                                field = Some(Identifier { name: text(source, t.span.clone()), span: t.span.clone() });
                            }
                        }
                    }
                    idx += 1;
                }
                if let (Some(receiver_val), Some(field_val)) = (receiver, field) {
                    Ok(Expr::Field { receiver: Box::new(receiver_val), field: field_val, span })
                }
                else {
                    Err(OakError::syntax_error(
                        format!("Missing receiver or field in field expression at {:?}", span),
                        source.get_location(span.start),
                    ))
                }
            }
            RustSyntaxKind::IndexExpression => {
                let span = node.span();
                // children: base '[' index ']'
                let mut base: Option<Expr> = None;
                let mut index: Option<Expr> = None;
                let mut idx = 0;
                for child in node.children() {
                    match child {
                        RedTree::Node(n) => {
                            if idx == 0 {
                                base = Some(RustParser::parse_expr(self, n, source)?);
                            }
                            else {
                                index = Some(RustParser::parse_expr(self, n, source)?);
                            }
                        }
                        _ => {}
                    }
                    idx += 1;
                }
                if let (Some(base_expr), Some(index_expr)) = (base, index) {
                    Ok(Expr::Index { receiver: Box::new(base_expr), index: Box::new(index_expr), span })
                }
                else {
                    Err(OakError::syntax_error(
                        format!("Missing base or index in index expression at {:?}", span),
                        source.get_location(span.start),
                    ))
                }
            }
            RustSyntaxKind::BlockExpression => Ok(Expr::Block(self.build_block(node, source)?)),
            _ => Err(OakError::syntax_error(
                format!("Unknown expression type at {:?}", node.span()),
                source.get_location(node.span().start),
            )),
        }
    }
}

#[inline]
fn text(source: &SourceText, span: Range<usize>) -> String {
    source.get_text_in(span).to_string()
}
