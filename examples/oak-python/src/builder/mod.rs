use crate::{
    ast::{BinaryOperator, ExceptHandler, Expression, ImportName, Keyword, Literal, Parameter, Program, PythonRoot, Statement, WithItem},
    language::PythonLanguage,
    lexer::token_type::PythonTokenType,
    parser::{PythonParser, element_type::PythonElementType},
};
use oak_core::{Builder, BuilderCache, GreenNode, GreenTree, OakDiagnostics, OakError, Parser, SourceText, TextEdit, builder::BuildOutput, source::Source};

/// AST builder for the Python language.
pub struct PythonBuilder<'config> {
    /// Reference to the language configuration
    config: &'config PythonLanguage,
}

impl<'config> PythonBuilder<'config> {
    /// Parses an f-string's text content into a list of expressions.
    fn parse_fstring_text(&self, text: &str) -> Result<Vec<Expression>, OakError> {
        let mut values = Vec::new();
        let mut i = 0;
        let bytes = text.as_bytes();

        // Skip prefix (f, rf, fr)
        while i < bytes.len() && (bytes[i] == b'f' || bytes[i] == b'F' || bytes[i] == b'r' || bytes[i] == b'R') {
            i += 1;
        }

        // Skip opening quotes
        if i >= bytes.len() {
            return Ok(values);
        }
        let quote_char = bytes[i];
        let mut quote_count = 0;
        while i < bytes.len() && bytes[i] == quote_char && quote_count < 3 {
            i += 1;
            quote_count += 1;
        }

        let end_index = if bytes.len() >= quote_count { bytes.len() - quote_count } else { bytes.len() };
        let mut start = i;

        while i < end_index {
            if bytes[i] == b'{' {
                // Potential expression
                if i + 1 < end_index && bytes[i + 1] == b'{' {
                    // Escaped {{
                    i += 2;
                    continue;
                }

                // Add literal part before {
                if i > start {
                    let s = &text[start..i];
                    values.push(Expression::Literal(Literal::String(s.to_string())));
                }

                // Find matching }
                i += 1; // skip {
                let expr_start = i;
                let mut brace_level = 1;
                while i < end_index && brace_level > 0 {
                    if bytes[i] == b'{' {
                        brace_level += 1
                    }
                    else if bytes[i] == b'}' {
                        brace_level -= 1
                    }
                    i += 1
                }

                if brace_level == 0 {
                    let expr_text = &text[expr_start..i - 1];
                    let expr = self.parse_single_expression(expr_text)?;
                    values.push(Expression::FormattedValue { value: Box::new(expr), conversion: 0, format_spec: None });
                    start = i
                }
                else {
                    // Unmatched {
                    start = i
                }
            }
            else if bytes[i] == b'}' {
                if i + 1 < end_index && bytes[i + 1] == b'}' {
                    // Escaped }}
                    i += 2;
                    continue;
                }
                i += 1
            }
            else {
                i += 1
            }
        }

        if start < end_index {
            let s = &text[start..end_index];
            values.push(Expression::Literal(Literal::String(s.to_string())))
        }

        Ok(values)
    }

    /// Parses a single expression from text.
    fn parse_single_expression(&self, text: &str) -> Result<Expression, OakError> {
        let parser = PythonParser::new(self.config);
        let mut cache = oak_core::parser::session::ParseSession::<PythonLanguage>::default();
        let source = SourceText::new(text.to_string());
        let output = parser.parse(&source, &[], &mut cache);

        match output.result {
            Ok(green) => {
                for child in green.children() {
                    if let GreenTree::Node(node) = child {
                        if node.kind == PythonElementType::Expr {
                            for subchild in node.children() {
                                if let GreenTree::Node(expr_node) = subchild {
                                    if !expr_node.kind.is_trivia() {
                                        return self.build_expression(expr_node, 0, &source);
                                    }
                                }
                            }
                        }
                    }
                }
                Ok(Expression::Name(text.to_string()))
            }
            Err(e) => Err(e),
        }
    }

    /// Creates a new Python builder.
    pub fn new(config: &'config PythonLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<PythonLanguage> for PythonBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<PythonLanguage>) -> BuildOutput<PythonLanguage> {
        let parser = PythonParser::new(self.config);

        let mut parse_cache = oak_core::parser::session::ParseSession::<PythonLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut parse_cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                match self.build_root(green_tree, &source_text) {
                    Ok(ast_root) => OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics },
                    Err(build_error) => {
                        let mut diagnostics = parse_result.diagnostics;
                        diagnostics.push(build_error.clone());
                        OakDiagnostics { result: Err(build_error), diagnostics }
                    }
                }
            }
            Err(e) => OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
        }
    }
}

impl<'config> PythonBuilder<'config> {
    /// Builds a Python root from a green tree.
    pub fn build_root(&self, green_tree: &GreenNode<PythonLanguage>, source: &SourceText) -> Result<PythonRoot, OakError> {
        let mut statements = Vec::new();
        let mut current_offset = 0;
        let mut pending_decorators = Vec::new();

        for child in green_tree.children() {
            let child_len = child.len() as usize;
            match child {
                GreenTree::Node(node) => {
                    if !node.kind.is_trivia() {
                        if node.kind == PythonElementType::Decorator {
                            // Collect decorator expression
                            let mut decorator_offset = current_offset;
                            for d_child in node.children() {
                                if let GreenTree::Node(expr_node) = d_child {
                                    if !expr_node.kind.is_trivia() {
                                        pending_decorators.push(self.build_expression(expr_node, decorator_offset, source)?)
                                    }
                                }
                                decorator_offset += d_child.len() as usize
                            }
                        }
                        else {
                            if let Some(mut stmt) = self.build_statement(node, current_offset, source)? {
                                // Attach decorators to function or class def
                                match &mut stmt {
                                    Statement::FunctionDef { decorators, .. } | Statement::AsyncFunctionDef { decorators, .. } | Statement::ClassDef { decorators, .. } => *decorators = std::mem::take(&mut pending_decorators),
                                    _ => {
                                        // If it's not a function or class def, decorators are invalid here
                                        // but for now we just drop them or we could issue a warning
                                        pending_decorators.clear()
                                    }
                                }
                                statements.push(stmt)
                            }
                        }
                    }
                }
                _ => {}
            }
            current_offset += child_len
        }

        Ok(PythonRoot { program: Program { statements }, span: (0..green_tree.text_len() as usize).into() })
    }

    /// Builds a statement from a green node.
    fn build_statement(&self, node: &GreenNode<PythonLanguage>, offset: usize, source: &SourceText) -> Result<Option<Statement>, OakError> {
        match node.kind {
            PythonElementType::FunctionDef | PythonElementType::AsyncFunctionDef => {
                let is_async = node.kind == PythonElementType::AsyncFunctionDef;
                let mut name = String::new();
                let mut parameters = Vec::new();
                let mut body = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Leaf(leaf) if leaf.kind == PythonTokenType::Identifier => name = source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string(),
                        GreenTree::Node(n) if n.kind == PythonElementType::Arguments => parameters = self.build_parameters(n, current_offset, source)?,
                        GreenTree::Node(n) if n.kind == PythonElementType::Suite => body = self.build_suite(n, current_offset, source)?,
                        _ => {}
                    }
                    current_offset += child_len
                }
                if is_async { Ok(Some(Statement::AsyncFunctionDef { decorators: Vec::new(), name, parameters, return_type: None, body })) } else { Ok(Some(Statement::FunctionDef { decorators: Vec::new(), name, parameters, return_type: None, body })) }
            }
            PythonElementType::ClassDef => {
                let mut name = String::new();
                let mut bases = Vec::new();
                let mut body = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Leaf(leaf) if leaf.kind == PythonTokenType::Identifier => name = source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string(),
                        GreenTree::Node(n) if n.kind == PythonElementType::Tuple || n.kind == PythonElementType::List => {
                            // Simple bases handling: if it's a tuple or list in class def, treat as bases
                            let expr = self.build_expression(n, current_offset, source)?;
                            match expr {
                                Expression::Tuple { elts } => bases = elts,
                                Expression::List { elts } => bases = elts,
                                _ => bases.push(expr),
                            }
                        }
                        GreenTree::Node(n) if n.kind == PythonElementType::Suite => body = self.build_suite(n, current_offset, source)?,
                        GreenTree::Node(n) => {
                            // Try to see if it's an expression (base class)
                            if !n.kind.is_trivia() && n.kind != PythonElementType::Suite {
                                let expr = self.build_expression(n, current_offset, source)?;
                                bases.push(expr)
                            }
                        }
                        _ => {}
                    }
                    current_offset += child_len
                }
                Ok(Some(Statement::ClassDef { decorators: Vec::new(), name, bases, body }))
            }
            PythonElementType::Return => {
                let mut value = None;
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            value = Some(self.build_expression(n, current_offset, source)?)
                        }
                    }
                    current_offset += child_len
                }
                Ok(Some(Statement::Return(value)))
            }
            PythonElementType::AssignStmt => {
                let mut exprs = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Node(n) => {
                            if !n.kind.is_trivia() {
                                exprs.push(self.build_expression(n, current_offset, source)?)
                            }
                        }
                        _ => {}
                    }
                    current_offset += child_len
                }

                if exprs.len() >= 2 {
                    // Python supports multiple assignments: a = b = c = 0
                    // In our AST, we'll just handle the first two for now to match the Statement::Assignment definition
                    // or we could change the AST to support multiple targets.
                    // Given the current AST: Assignment { target: Expression, value: Expression }
                    // We'll treat it as target = value.
                    let target = exprs[0].clone();
                    let value = exprs[exprs.len() - 1].clone();
                    Ok(Some(Statement::Assignment { target, value }))
                }
                else {
                    Ok(None)
                }
            }

            PythonElementType::Expr => {
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Node(n) => {
                            if !n.kind.is_trivia() {
                                if n.kind == PythonElementType::AssignStmt {
                                    return self.build_statement(n, current_offset, source);
                                }
                                return Ok(Some(Statement::Expression(self.build_expression(n, current_offset, source)?)));
                            }
                        }
                        _ => {}
                    }
                    current_offset += child_len
                }
                Ok(None)
            }
            PythonElementType::If => {
                let mut test = None;
                let mut body = Vec::new();
                let mut orelse = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            if test.is_none() && n.kind != PythonElementType::Suite {
                                test = Some(self.build_expression(n, current_offset, source)?)
                            }
                            else if body.is_empty() && n.kind == PythonElementType::Suite {
                                body = self.build_suite(n, current_offset, source)?
                            }
                            else if n.kind == PythonElementType::Suite {
                                orelse = self.build_suite(n, current_offset, source)?
                            }
                        }
                    }
                    current_offset += child_len
                }

                Ok(Some(Statement::If { test: test.unwrap_or(Expression::Literal(Literal::Boolean(true))), body, orelse }))
            }
            PythonElementType::While => {
                let mut test = None;
                let mut body = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            if test.is_none() && n.kind != PythonElementType::Suite {
                                test = Some(self.build_expression(n, current_offset, source)?)
                            }
                            else if n.kind == PythonElementType::Suite {
                                body = self.build_suite(n, current_offset, source)?
                            }
                        }
                    }
                    current_offset += child_len
                }

                Ok(Some(Statement::While { test: test.unwrap_or(Expression::Literal(Literal::Boolean(true))), body, orelse: Vec::new() }))
            }
            PythonElementType::For | PythonElementType::AsyncFor => {
                let is_async = node.kind == PythonElementType::AsyncFor;
                let mut target = None;
                let mut iter = None;
                let mut body = Vec::new();
                let mut orelse = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            if target.is_none() && n.kind != PythonElementType::Suite && n.kind != PythonElementType::InKeyword.into() {
                                target = Some(self.build_expression(n, current_offset, source)?)
                            }
                            else if iter.is_none() && n.kind != PythonElementType::Suite && n.kind != PythonElementType::InKeyword.into() {
                                iter = Some(self.build_expression(n, current_offset, source)?)
                            }
                            else if n.kind == PythonElementType::Suite {
                                if body.is_empty() { body = self.build_suite(n, current_offset, source)? } else { orelse = self.build_suite(n, current_offset, source)? }
                            }
                        }
                    }
                    current_offset += child_len
                }

                let target = target.unwrap_or(Expression::Name("invalid_target".to_string()));
                let iter = iter.unwrap_or(Expression::Name("invalid_iter".to_string()));
                if is_async { Ok(Some(Statement::AsyncFor { target, iter, body, orelse })) } else { Ok(Some(Statement::For { target, iter, body, orelse })) }
            }
            PythonElementType::Pass => Ok(Some(Statement::Pass)),
            PythonElementType::Break => Ok(Some(Statement::Break)),
            PythonElementType::Continue => Ok(Some(Statement::Continue)),
            PythonElementType::Raise => {
                let mut exc = None;
                let mut cause = None;
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            let expr = self.build_expression(n, current_offset, source)?;
                            if exc.is_none() { exc = Some(expr) } else { cause = Some(expr) }
                        }
                    }
                    current_offset += child_len
                }
                Ok(Some(Statement::Raise { exc, cause }))
            }
            PythonElementType::Assert => {
                let mut test = None;
                let mut msg = None;
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            let expr = self.build_expression(n, current_offset, source)?;
                            if test.is_none() { test = Some(expr) } else { msg = Some(expr) }
                        }
                    }
                    current_offset += child_len
                }
                Ok(Some(Statement::Assert { test: test.unwrap_or(Expression::Literal(crate::ast::Literal::Boolean(true))), msg }))
            }
            PythonElementType::Import => {
                let mut names = Vec::new();
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if n.kind == PythonElementType::Alias {
                            names.push(self.build_import_name(n, current_offset, source)?)
                        }
                    }
                    current_offset += child_len
                }
                Ok(Some(Statement::Import { names }))
            }
            PythonElementType::ImportFrom => {
                let mut module = None;
                let mut names = Vec::new();
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Leaf(leaf) if leaf.kind == PythonTokenType::Identifier => {
                            if module.is_none() {
                                module = Some(source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string())
                            }
                        }
                        GreenTree::Node(n) if n.kind == PythonElementType::Alias => names.push(self.build_import_name(n, current_offset, source)?),
                        _ => {}
                    }
                    current_offset += child_len
                }
                Ok(Some(Statement::ImportFrom { module, names }))
            }
            PythonElementType::Global => {
                let mut names = Vec::new();
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Leaf(leaf) = child {
                        if leaf.kind == PythonTokenType::Identifier {
                            names.push(source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string())
                        }
                    }
                    current_offset += child_len
                }
                Ok(Some(Statement::Global { names }))
            }
            PythonElementType::Nonlocal => {
                let mut names = Vec::new();
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Leaf(leaf) = child {
                        if leaf.kind == PythonTokenType::Identifier {
                            names.push(source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string())
                        }
                    }
                    current_offset += child_len
                }
                Ok(Some(Statement::Nonlocal { names }))
            }
            PythonElementType::Try => {
                let mut body = Vec::new();
                let mut handlers = Vec::new();
                let mut orelse = Vec::new();
                let mut finalbody = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            match n.kind {
                                PythonElementType::Suite if body.is_empty() && handlers.is_empty() => body = self.build_suite(n, current_offset, source)?,
                                PythonElementType::ExceptHandler => handlers.push(self.build_except_handler(n, current_offset, source)?),
                                PythonElementType::Suite if !handlers.is_empty() && orelse.is_empty() => orelse = self.build_suite(n, current_offset, source)?,
                                PythonElementType::Suite if finalbody.is_empty() => finalbody = self.build_suite(n, current_offset, source)?,
                                _ => {}
                            }
                        }
                    }
                    current_offset += child_len
                }
                Ok(Some(Statement::Try { body, handlers, orelse, finalbody }))
            }
            PythonElementType::With => {
                let mut items = Vec::new();
                let mut body = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            if n.kind == PythonElementType::WithItem {
                                items.push(self.build_with_item(n, current_offset, source)?)
                            }
                            else if n.kind == PythonElementType::Suite {
                                body = self.build_suite(n, current_offset, source)?
                            }
                        }
                    }
                    current_offset += child_len
                }
                Ok(Some(Statement::With { items, body }))
            }
            PythonElementType::Suite => {
                // Suites are handled by build_suite
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    /// Builds a suite (list of statements) from a green node.
    fn build_suite(&self, node: &GreenNode<PythonLanguage>, offset: usize, source: &SourceText) -> Result<Vec<Statement>, OakError> {
        let mut statements = Vec::new();
        let mut current_offset = offset;

        for child in node.children() {
            let child_len = child.len() as usize;
            match child {
                GreenTree::Node(n) => {
                    if !n.kind.is_trivia() {
                        if let Some(stmt) = self.build_statement(n, current_offset, source)? {
                            statements.push(stmt)
                        }
                    }
                }
                _ => {}
            }
            current_offset += child_len
        }
        Ok(statements)
    }

    /// Builds an expression from a green node.
    fn build_expression(&self, node: &GreenNode<PythonLanguage>, offset: usize, source: &SourceText) -> Result<Expression, OakError> {
        match node.kind {
            PythonElementType::Constant => {
                let mut current_offset = offset;
                for child in node.children() {
                    if let GreenTree::Leaf(leaf) = child {
                        if !leaf.kind.is_trivia() {
                            let text = source.get_text_in((current_offset..current_offset + leaf.length as usize).into());
                            if let Ok(val) = text.parse::<i64>() {
                                return Ok(Expression::Literal(Literal::Integer(val)));
                            }
                            else if let Ok(val) = text.parse::<f64>() {
                                return Ok(Expression::Literal(Literal::Float(val)));
                            }
                            else if text == "True" {
                                return Ok(Expression::Literal(Literal::Boolean(true)));
                            }
                            else if text == "False" {
                                return Ok(Expression::Literal(Literal::Boolean(false)));
                            }
                            else if text == "None" {
                                return Ok(Expression::Literal(Literal::None));
                            }
                            else {
                                let s = text.to_string();
                                if s.starts_with('b') || s.starts_with('B') {
                                    let content = if (s.starts_with("b\"") || s.starts_with("B\"") || s.starts_with("b'")) && (s.ends_with('"') || s.ends_with('\'')) { &s[2..s.len() - 1] } else { &s[1..] };
                                    return Ok(Expression::Literal(Literal::Bytes(content.as_bytes().to_vec())));
                                }
                                let mut s = s;
                                if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
                                    s = s[1..s.len() - 1].to_string()
                                }
                                return Ok(Expression::Literal(Literal::String(s)));
                            }
                        }
                    }
                    current_offset += child.len() as usize
                }
                Ok(Expression::Name("invalid_constant".to_string()))
            }
            PythonElementType::Name => {
                let mut current_offset = offset;
                for child in node.children() {
                    if let GreenTree::Leaf(leaf) = child {
                        if !leaf.kind.is_trivia() {
                            let text = source.get_text_in((current_offset..current_offset + leaf.length as usize).into());
                            return Ok(Expression::Name(text.to_string()));
                        }
                    }
                    current_offset += child.len() as usize;
                }
                Ok(Expression::Name("invalid_name".to_string()))
            }
            PythonElementType::BinOp => {
                let mut left = None;
                let mut operator = None;
                let mut right = None;
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Node(n) => {
                            if !n.kind.is_trivia() {
                                let expr = self.build_expression(n, current_offset, source)?;
                                if left.is_none() {
                                    left = Some(Box::new(expr));
                                }
                                else if right.is_none() {
                                    right = Some(Box::new(expr));
                                }
                            }
                        }
                        GreenTree::Leaf(leaf) => {
                            if !leaf.kind.is_trivia() {
                                let op = match leaf.kind {
                                    PythonTokenType::Plus => Some(BinaryOperator::Add),
                                    PythonTokenType::Minus => Some(BinaryOperator::Sub),
                                    PythonTokenType::Star => Some(BinaryOperator::Mult),
                                    PythonTokenType::Slash => Some(BinaryOperator::Div),
                                    PythonTokenType::Percent => Some(BinaryOperator::Mod),
                                    PythonTokenType::DoubleStar => Some(BinaryOperator::Pow),
                                    PythonTokenType::LeftShift => Some(BinaryOperator::LShift),
                                    PythonTokenType::RightShift => Some(BinaryOperator::RShift),
                                    PythonTokenType::Pipe => Some(BinaryOperator::BitOr),
                                    PythonTokenType::Caret => Some(BinaryOperator::BitXor),
                                    PythonTokenType::Ampersand => Some(BinaryOperator::BitAnd),
                                    PythonTokenType::DoubleSlash => Some(BinaryOperator::FloorDiv),
                                    _ => None,
                                };
                                if let Some(op) = op {
                                    operator = Some(op);
                                }
                            }
                        }
                    }
                    current_offset += child_len;
                }

                let l_is = left.is_some();
                let op_is = operator.is_some();
                let r_is = right.is_some();
                if let (Some(l), Some(op), Some(r)) = (left, operator, right) {
                    Ok(Expression::BinaryOp { left: l, operator: op, right: r })
                }
                else {
                    println!("Warning: Invalid BinOp at {}, left={}, op={}, right={}", offset, l_is, op_is, r_is);
                    Ok(Expression::Name(format!("invalid_binop_at_{}", offset)))
                }
            }
            PythonElementType::Call => {
                let mut func = None;
                let mut args = Vec::new();
                let mut keywords = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            if func.is_none() {
                                func = Some(Box::new(self.build_expression(n, current_offset, source)?));
                            }
                            else if n.kind == PythonElementType::Keyword {
                                keywords.push(self.build_keyword(n, current_offset, source)?);
                            }
                            else if n.kind == PythonElementType::Starred {
                                let expr = self.build_starred(n, current_offset, source)?;
                                args.push(expr);
                            }
                            else {
                                args.push(self.build_expression(n, current_offset, source)?);
                            }
                        }
                    }
                    current_offset += child_len;
                }

                if let Some(f) = func { Ok(Expression::Call { func: f, args, keywords }) } else { Ok(Expression::Name("invalid_call".to_string())) }
            }
            PythonElementType::Attribute => {
                let mut value = None;
                let mut attr = String::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Node(n) => {
                            if !n.kind.is_trivia() {
                                value = Some(Box::new(self.build_expression(n, current_offset, source)?));
                            }
                        }
                        GreenTree::Leaf(leaf) if leaf.kind == PythonTokenType::Identifier => {
                            attr = source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string();
                        }
                        _ => {}
                    }
                    current_offset += child_len;
                }

                if let Some(v) = value { Ok(Expression::Attribute { value: v, attr }) } else { Ok(Expression::Name("invalid_attribute".to_string())) }
            }
            PythonElementType::Expr => {
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            return self.build_expression(n, current_offset, source);
                        }
                    }
                    current_offset += child_len;
                }
                Ok(Expression::Name("invalid_expr".to_string()))
            }
            PythonElementType::Tuple => {
                let mut exprs = Vec::new();
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            exprs.push(self.build_expression(n, current_offset, source)?);
                        }
                    }
                    current_offset += child_len;
                }
                if exprs.len() == 1 { Ok(exprs.remove(0)) } else { Ok(Expression::Tuple { elts: exprs }) }
            }
            PythonElementType::List => {
                let mut exprs = Vec::new();
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            exprs.push(self.build_expression(n, current_offset, source)?);
                        }
                    }
                    current_offset += child_len;
                }
                Ok(Expression::List { elts: exprs })
            }
            PythonElementType::Subscript => {
                let mut value = None;
                let mut slice = None;
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            if value.is_none() {
                                value = Some(Box::new(self.build_expression(n, current_offset, source)?));
                            }
                            else {
                                slice = Some(Box::new(self.build_expression(n, current_offset, source)?));
                            }
                        }
                    }
                    current_offset += child_len;
                }

                if let (Some(v), Some(s)) = (value, slice) { Ok(Expression::Subscript { value: v, slice: s }) } else { Ok(Expression::Name("invalid_subscript".to_string())) }
            }
            PythonElementType::JoinedStr => {
                let mut current_offset = offset;
                let mut values = Vec::new();
                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Node(n) => {
                            if !n.kind.is_trivia() {
                                values.push(self.build_expression(n, current_offset, source)?);
                            }
                        }
                        GreenTree::Leaf(leaf) if leaf.kind == PythonTokenType::FString => {
                            let text = source.get_text_in((current_offset..current_offset + leaf.length as usize).into());
                            values.extend(self.parse_fstring_text(&text)?);
                        }
                        _ => {}
                    }
                    current_offset += child_len;
                }
                Ok(Expression::JoinedStr { values })
            }
            PythonElementType::FormattedValue => {
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            let value = Box::new(self.build_expression(n, current_offset, source)?);
                            return Ok(Expression::FormattedValue { value, conversion: 0, format_spec: None });
                        }
                    }
                    current_offset += child_len;
                }
                Ok(Expression::Name("invalid_formatted_value".to_string()))
            }
            PythonElementType::Yield => {
                let mut value = None;
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            value = Some(Box::new(self.build_expression(n, current_offset, source)?));
                        }
                    }
                    current_offset += child_len;
                }
                Ok(Expression::Yield(value))
            }
            PythonElementType::YieldFrom => {
                let mut value = None;
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            value = Some(Box::new(self.build_expression(n, current_offset, source)?));
                        }
                    }
                    current_offset += child_len;
                }
                Ok(Expression::YieldFrom(value.unwrap_or(Box::new(Expression::Name("invalid_yield_from".to_string())))))
            }
            PythonElementType::Starred => self.build_starred(node, offset, source),
            PythonElementType::Await => {
                let mut value = None;
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            value = Some(Box::new(self.build_expression(n, current_offset, source)?));
                        }
                    }
                    current_offset += child_len;
                }
                Ok(Expression::Await(value.unwrap_or(Box::new(Expression::Name("invalid_await".to_string())))))
            }
            PythonElementType::Dict => {
                let mut keys = Vec::new();
                let mut values = Vec::new();
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            if n.kind == PythonElementType::Starred {
                                let expr = self.build_starred(n, current_offset, source)?;
                                keys.push(None);
                                values.push(expr);
                            }
                            else if keys.len() == values.len() {
                                // We are expecting a key
                                let expr = self.build_expression(n, current_offset, source)?;
                                keys.push(Some(expr));
                            }
                            else {
                                // We are expecting a value
                                let expr = self.build_expression(n, current_offset, source)?;
                                values.push(expr);
                            }
                        }
                    }
                    current_offset += child_len;
                }
                Ok(Expression::Dict { keys, values })
            }
            PythonElementType::Set => {
                let mut elts = Vec::new();
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            elts.push(self.build_expression(n, current_offset, source)?);
                        }
                    }
                    current_offset += child_len;
                }
                Ok(Expression::Set { elts })
            }
            PythonElementType::ListComp => {
                let mut elt = None;
                let mut generators = Vec::new();
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            if elt.is_none() {
                                elt = Some(Box::new(self.build_expression(n, current_offset, source)?));
                            }
                            else if n.kind == PythonElementType::Comprehension {
                                generators.push(self.build_comprehension(n, current_offset, source)?);
                            }
                        }
                    }
                    current_offset += child_len;
                }
                Ok(Expression::ListComp { elt: elt.unwrap_or(Box::new(Expression::Name("invalid_elt".to_string()))), generators })
            }
            PythonElementType::SetComp => {
                let mut elt = None;
                let mut generators = Vec::new();
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            if elt.is_none() {
                                elt = Some(Box::new(self.build_expression(n, current_offset, source)?));
                            }
                            else if n.kind == PythonElementType::Comprehension {
                                generators.push(self.build_comprehension(n, current_offset, source)?);
                            }
                        }
                    }
                    current_offset += child_len;
                }
                Ok(Expression::SetComp { elt: elt.unwrap_or(Box::new(Expression::Name("invalid_elt".to_string()))), generators })
            }
            PythonElementType::DictComp => {
                let mut key = None;
                let mut value = None;
                let mut generators = Vec::new();
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            if key.is_none() {
                                key = Some(Box::new(self.build_expression(n, current_offset, source)?));
                            }
                            else if value.is_none() {
                                value = Some(Box::new(self.build_expression(n, current_offset, source)?));
                            }
                            else if n.kind == PythonElementType::Comprehension {
                                generators.push(self.build_comprehension(n, current_offset, source)?);
                            }
                        }
                    }
                    current_offset += child_len;
                }
                Ok(Expression::DictComp { key: key.unwrap_or(Box::new(Expression::Name("invalid_key".to_string()))), value: value.unwrap_or(Box::new(Expression::Name("invalid_value".to_string()))), generators })
            }
            PythonElementType::GeneratorExp => {
                let mut elt = None;
                let mut generators = Vec::new();
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            if elt.is_none() {
                                elt = Some(Box::new(self.build_expression(n, current_offset, source)?));
                            }
                            else if n.kind == PythonElementType::Comprehension {
                                generators.push(self.build_comprehension(n, current_offset, source)?);
                            }
                        }
                    }
                    current_offset += child_len;
                }
                Ok(Expression::GeneratorExp { elt: elt.unwrap_or(Box::new(Expression::Name("invalid_elt".to_string()))), generators })
            }
            PythonElementType::Slice => {
                let mut lower = None;
                let mut upper = None;
                let mut step = None;
                let mut current_offset = offset;
                let mut colon_count = 0;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Leaf(leaf) if leaf.kind == PythonTokenType::Colon => {
                            colon_count += 1;
                        }
                        GreenTree::Node(n) if !n.kind.is_trivia() => {
                            let expr = Some(Box::new(self.build_expression(n, current_offset, source)?));
                            if colon_count == 0 {
                                lower = expr;
                            }
                            else if colon_count == 1 {
                                upper = expr;
                            }
                            else {
                                step = expr;
                            }
                        }
                        _ => {}
                    }
                    current_offset += child_len;
                }
                Ok(Expression::Slice { lower, upper, step })
            }
            _ => Ok(Expression::Name(format!("unsupported_kind_{:?}", node.kind))),
        }
    }

    /// Builds a keyword argument from a green node.
    pub(crate) fn build_keyword(&self, node: &GreenNode<PythonLanguage>, offset: usize, source: &SourceText) -> Result<Keyword, OakError> {
        let mut arg = None;
        let mut value = None;
        let mut current_offset = offset;

        for child in node.children() {
            let child_len = child.len() as usize;
            match child {
                GreenTree::Leaf(leaf) if leaf.kind == PythonTokenType::Identifier => {
                    arg = Some(source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string());
                }
                GreenTree::Node(n) if !n.kind.is_trivia() => {
                    value = Some(self.build_expression(n, current_offset, source)?);
                }
                _ => {}
            }
            current_offset += child_len;
        }

        if let Some(v) = value { Ok(Keyword { arg, value: v }) } else { Err(OakError::custom_error("Invalid keyword".to_string())) }
    }

    /// Builds a starred expression from a green node.
    pub(crate) fn build_starred(&self, node: &GreenNode<PythonLanguage>, offset: usize, source: &SourceText) -> Result<Expression, OakError> {
        let mut value = None;
        let mut is_double = false;
        let mut current_offset = offset;

        for child in node.children() {
            let child_len = child.len() as usize;
            match child {
                GreenTree::Leaf(leaf) if leaf.kind == PythonTokenType::Star => {
                    is_double = false;
                }
                GreenTree::Leaf(leaf) if leaf.kind == PythonTokenType::DoubleStar => {
                    is_double = true;
                }
                GreenTree::Node(n) if !n.kind.is_trivia() => {
                    value = Some(Box::new(self.build_expression(n, current_offset, source)?));
                }
                _ => {}
            }
            current_offset += child_len;
        }

        if let Some(v) = value { Ok(Expression::Starred { value: v, is_double }) } else { Err(OakError::custom_error("Invalid starred expression".to_string())) }
    }

    /// Builds an import name (alias) from a green node.
    fn build_import_name(&self, node: &GreenNode<PythonLanguage>, offset: usize, source: &SourceText) -> Result<ImportName, OakError> {
        let mut name = String::new();
        let mut asname = None;
        let mut current_offset = offset;

        for child in node.children() {
            let child_len = child.len() as usize;
            match child {
                GreenTree::Leaf(leaf) if leaf.kind == PythonTokenType::Identifier => {
                    let text = source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string();
                    if name.is_empty() {
                        name = text;
                    }
                    else {
                        asname = Some(text);
                    }
                }
                _ => {}
            }
            current_offset += child_len;
        }

        Ok(ImportName { name, asname })
    }

    /// Builds a list of parameters from a green node.
    fn build_parameters(&self, node: &GreenNode<PythonLanguage>, offset: usize, source: &SourceText) -> Result<Vec<Parameter>, OakError> {
        let mut parameters = Vec::new();
        let mut current_offset = offset;

        for child in node.children() {
            let child_len = child.len() as usize;
            if let GreenTree::Node(n) = child {
                if n.kind == PythonElementType::Arg {
                    parameters.push(self.build_parameter(n, current_offset, source)?);
                }
            }
            current_offset += child_len;
        }
        Ok(parameters)
    }

    /// Builds a single parameter from a green node.
    fn build_parameter(&self, node: &GreenNode<PythonLanguage>, offset: usize, source: &SourceText) -> Result<Parameter, OakError> {
        let mut name = String::new();
        let mut default = None;
        let mut is_vararg = false;
        let mut is_kwarg = false;
        let mut current_offset = offset;

        for child in node.children() {
            let child_len = child.len() as usize;
            match child {
                GreenTree::Leaf(leaf) => {
                    if leaf.kind == PythonTokenType::Identifier {
                        name = source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string();
                    }
                    else if leaf.kind == PythonTokenType::Star {
                        is_vararg = true;
                    }
                    else if leaf.kind == PythonTokenType::DoubleStar {
                        is_kwarg = true;
                    }
                }
                GreenTree::Node(n) => {
                    if !n.kind.is_trivia() {
                        default = Some(self.build_expression(n, current_offset, source)?);
                    }
                }
            }
            current_offset += child_len;
        }

        Ok(Parameter { name, annotation: None, default, is_vararg, is_kwarg })
    }

    /// Builds an exception handler from a green node.
    fn build_except_handler(&self, node: &GreenNode<PythonLanguage>, offset: usize, source: &SourceText) -> Result<ExceptHandler, OakError> {
        let mut type_ = None;
        let mut name = None;
        let mut body = Vec::new();
        let mut current_offset = offset;

        for child in node.children() {
            let child_len = child.len() as usize;
            match child {
                GreenTree::Node(n) => {
                    if !n.kind.is_trivia() {
                        if n.kind == PythonElementType::Suite {
                            body = self.build_suite(n, current_offset, source)?;
                        }
                        else if type_.is_none() {
                            type_ = Some(self.build_expression(n, current_offset, source)?);
                        }
                    }
                }
                GreenTree::Leaf(leaf) if leaf.kind == PythonTokenType::Identifier => {
                    name = Some(source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string());
                }
                _ => {}
            }
            current_offset += child_len;
        }
        Ok(ExceptHandler { type_, name, body })
    }

    /// Builds a `with` item from a green node.
    fn build_with_item(&self, node: &GreenNode<PythonLanguage>, offset: usize, source: &SourceText) -> Result<WithItem, OakError> {
        let mut context_expr = None;
        let mut optional_vars = None;
        let mut current_offset = offset;

        for child in node.children() {
            let child_len = child.len() as usize;
            if let GreenTree::Node(n) = child {
                if !n.kind.is_trivia() {
                    let expr = self.build_expression(n, current_offset, source)?;
                    if context_expr.is_none() { context_expr = Some(expr) } else { optional_vars = Some(expr) }
                }
            }
            current_offset += child_len
        }
        Ok(WithItem { context_expr: context_expr.unwrap_or(Expression::Literal(Literal::None)), optional_vars })
    }

    /// Builds a comprehension from a green node.
    fn build_comprehension(&self, node: &GreenNode<PythonLanguage>, offset: usize, source: &SourceText) -> Result<crate::ast::Comprehension, OakError> {
        let mut target = None;
        let mut iter = None;
        let mut ifs = Vec::new();
        let mut is_async = false;
        let mut current_offset = offset;

        for child in node.children() {
            let child_len = child.len() as usize;
            match child {
                GreenTree::Leaf(leaf) if leaf.kind == PythonTokenType::AsyncKeyword => is_async = true,
                GreenTree::Node(n) if !n.kind.is_trivia() && n.kind != PythonElementType::ForKeyword && n.kind != PythonElementType::InKeyword && n.kind != PythonElementType::IfKeyword => {
                    let expr = self.build_expression(n, current_offset, source)?;
                    if target.is_none() {
                        target = Some(expr)
                    }
                    else if iter.is_none() {
                        iter = Some(expr)
                    }
                    else {
                        ifs.push(expr)
                    }
                }
                _ => {}
            }
            current_offset += child_len
        }

        Ok(crate::ast::Comprehension { target: target.unwrap_or(Expression::Name("invalid_target".to_string())), iter: iter.unwrap_or(Expression::Name("invalid_iter".to_string())), ifs, is_async })
    }
}
