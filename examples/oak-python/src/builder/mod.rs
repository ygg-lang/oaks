use crate::{
    ast::{BinaryOperator, Expression, Literal, Parameter, Program, PythonRoot, Statement},
    kind::PythonSyntaxKind,
    language::PythonLanguage,
    parser::PythonParser,
};
use oak_core::{Builder, BuilderCache, GreenNode, GreenTree, OakDiagnostics, OakError, Parser, SourceText, TextEdit, TokenType, builder::BuildOutput, source::Source};

pub struct PythonBuilder<'config> {
    config: &'config PythonLanguage,
}

impl<'config> PythonBuilder<'config> {
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
    pub fn build_root(&self, green_tree: &GreenNode<PythonLanguage>, source: &SourceText) -> Result<PythonRoot, OakError> {
        let mut statements = Vec::new();
        let mut current_offset = 0;

        for child in green_tree.children() {
            let child_len = child.len() as usize;
            match child {
                GreenTree::Node(node) => {
                    if let Some(stmt) = self.build_statement(node, current_offset, source)? {
                        statements.push(stmt);
                    }
                }
                GreenTree::Leaf(_) => {}
            }
            current_offset += child_len;
        }

        Ok(PythonRoot { program: Program { statements }, span: (0..green_tree.text_len() as usize).into() })
    }

    fn build_statement(&self, node: &GreenNode<PythonLanguage>, offset: usize, source: &SourceText) -> Result<Option<Statement>, OakError> {
        match node.kind {
            PythonSyntaxKind::FunctionDef => {
                let mut name = String::new();
                let mut parameters = Vec::new();
                let mut body = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Leaf(leaf) if leaf.kind == PythonSyntaxKind::Identifier => {
                            name = source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string();
                        }
                        GreenTree::Node(n) if n.kind == PythonSyntaxKind::Arguments => {
                            parameters = self.build_parameters(n, current_offset, source)?;
                        }
                        GreenTree::Node(n) if n.kind == PythonSyntaxKind::Suite => {
                            body = self.build_suite(n, current_offset, source)?;
                        }
                        _ => {}
                    }
                    current_offset += child_len;
                }
                Ok(Some(Statement::FunctionDef { name, parameters, return_type: None, body }))
            }
            PythonSyntaxKind::ClassDef => {
                let mut name = String::new();
                let mut body = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Leaf(leaf) if leaf.kind == PythonSyntaxKind::Identifier => {
                            name = source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string();
                        }
                        GreenTree::Node(n) if n.kind == PythonSyntaxKind::Suite => {
                            body = self.build_suite(n, current_offset, source)?;
                        }
                        _ => {}
                    }
                    current_offset += child_len;
                }
                Ok(Some(Statement::ClassDef { name, bases: Vec::new(), body }))
            }
            PythonSyntaxKind::Return => {
                let mut value = None;
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        value = Some(self.build_expression(n, current_offset, source)?);
                    }
                    current_offset += child_len;
                }
                Ok(Some(Statement::Return(value)))
            }
            PythonSyntaxKind::AssignStmt => {
                let mut left = None;
                let mut right = None;
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Node(n) => {
                            if !n.kind.is_ignored() {
                                if left.is_none() {
                                    left = Some(self.build_expression(n, current_offset, source)?);
                                }
                                else {
                                    right = Some(self.build_expression(n, current_offset, source)?);
                                }
                            }
                        }
                        _ => {}
                    }
                    current_offset += child_len;
                }

                if let (Some(l), Some(r)) = (left, right) { Ok(Some(Statement::Assignment { target: l, value: r })) } else { Ok(None) }
            }
            PythonSyntaxKind::Expr => {
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Node(n) => {
                            if !n.kind.is_ignored() {
                                if n.kind == PythonSyntaxKind::AssignStmt {
                                    return self.build_statement(n, current_offset, source);
                                }
                                return Ok(Some(Statement::Expression(self.build_expression(n, current_offset, source)?)));
                            }
                        }
                        _ => {}
                    }
                    current_offset += child_len;
                }
                Ok(None)
            }
            PythonSyntaxKind::If => {
                let mut test = None;
                let mut body = Vec::new();
                let mut orelse = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if test.is_none() && n.kind != PythonSyntaxKind::Suite && !n.kind.is_ignored() {
                            test = Some(self.build_expression(n, current_offset, source)?);
                        }
                        else if body.is_empty() && n.kind == PythonSyntaxKind::Suite {
                            body = self.build_suite(n, current_offset, source)?;
                        }
                        else if n.kind == PythonSyntaxKind::Suite {
                            orelse = self.build_suite(n, current_offset, source)?;
                        }
                    }
                    current_offset += child_len;
                }

                Ok(Some(Statement::If { test: test.unwrap_or(Expression::Literal(Literal::Boolean(true))), body, orelse }))
            }
            PythonSyntaxKind::While => {
                let mut test = None;
                let mut body = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if test.is_none() && n.kind != PythonSyntaxKind::Suite && !n.kind.is_ignored() {
                            test = Some(self.build_expression(n, current_offset, source)?);
                        }
                        else if n.kind == PythonSyntaxKind::Suite {
                            body = self.build_suite(n, current_offset, source)?;
                        }
                    }
                    current_offset += child_len;
                }

                Ok(Some(Statement::While { test: test.unwrap_or(Expression::Literal(Literal::Boolean(true))), body, orelse: Vec::new() }))
            }
            PythonSyntaxKind::Pass => Ok(Some(Statement::Pass)),
            PythonSyntaxKind::Break => Ok(Some(Statement::Break)),
            PythonSyntaxKind::Continue => Ok(Some(Statement::Continue)),
            PythonSyntaxKind::Suite => {
                // Suites are handled by build_suite
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    fn build_suite(&self, node: &GreenNode<PythonLanguage>, offset: usize, source: &SourceText) -> Result<Vec<Statement>, OakError> {
        let mut statements = Vec::new();
        let mut current_offset = offset;

        for child in node.children() {
            let child_len = child.len() as usize;
            match child {
                GreenTree::Node(n) => {
                    if let Some(stmt) = self.build_statement(n, current_offset, source)? {
                        statements.push(stmt);
                    }
                }
                GreenTree::Leaf(_) => {}
            }
            current_offset += child_len;
        }
        Ok(statements)
    }

    fn build_expression(&self, node: &GreenNode<PythonLanguage>, offset: usize, source: &SourceText) -> Result<Expression, OakError> {
        match node.kind {
            PythonSyntaxKind::Constant => {
                let mut current_offset = offset;
                for child in node.children() {
                    if let GreenTree::Leaf(leaf) = child {
                        if !leaf.kind.is_ignored() {
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
                                let mut s = text.to_string();
                                if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
                                    s = s[1..s.len() - 1].to_string();
                                }
                                return Ok(Expression::Literal(Literal::String(s)));
                            }
                        }
                    }
                    current_offset += child.len() as usize;
                }
                Ok(Expression::Name("invalid_constant".to_string()))
            }
            PythonSyntaxKind::Name => {
                let mut current_offset = offset;
                for child in node.children() {
                    if let GreenTree::Leaf(leaf) = child {
                        if !leaf.kind.is_ignored() {
                            let text = source.get_text_in((current_offset..current_offset + leaf.length as usize).into());
                            return Ok(Expression::Name(text.to_string()));
                        }
                    }
                    current_offset += child.len() as usize;
                }
                Ok(Expression::Name("invalid_name".to_string()))
            }
            PythonSyntaxKind::BinOp => {
                let mut left = None;
                let mut operator = None;
                let mut right = None;
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Node(n) => {
                            if !n.kind.is_ignored() {
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
                            if !leaf.kind.is_ignored() {
                                let op = match leaf.kind {
                                    PythonSyntaxKind::Add | PythonSyntaxKind::Plus => Some(BinaryOperator::Add),
                                    PythonSyntaxKind::Sub | PythonSyntaxKind::Minus => Some(BinaryOperator::Sub),
                                    PythonSyntaxKind::Mult | PythonSyntaxKind::Star => Some(BinaryOperator::Mult),
                                    PythonSyntaxKind::Div | PythonSyntaxKind::Slash => Some(BinaryOperator::Div),
                                    PythonSyntaxKind::Mod | PythonSyntaxKind::Percent => Some(BinaryOperator::Mod),
                                    PythonSyntaxKind::Pow | PythonSyntaxKind::DoubleStar => Some(BinaryOperator::Pow),
                                    PythonSyntaxKind::LShift | PythonSyntaxKind::LeftShift => Some(BinaryOperator::LShift),
                                    PythonSyntaxKind::RShift | PythonSyntaxKind::RightShift => Some(BinaryOperator::RShift),
                                    PythonSyntaxKind::BitOr | PythonSyntaxKind::Pipe => Some(BinaryOperator::BitOr),
                                    PythonSyntaxKind::BitXor | PythonSyntaxKind::Caret => Some(BinaryOperator::BitXor),
                                    PythonSyntaxKind::BitAnd | PythonSyntaxKind::Ampersand => Some(BinaryOperator::BitAnd),
                                    PythonSyntaxKind::FloorDiv | PythonSyntaxKind::DoubleSlash => Some(BinaryOperator::FloorDiv),
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
            PythonSyntaxKind::Call => {
                let mut func = None;
                let mut args = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_ignored() {
                            if func.is_none() {
                                func = Some(Box::new(self.build_expression(n, current_offset, source)?));
                            }
                            else {
                                args.push(self.build_expression(n, current_offset, source)?);
                            }
                        }
                    }
                    current_offset += child_len;
                }

                if let Some(f) = func { Ok(Expression::Call { func: f, args, keywords: Vec::new() }) } else { Ok(Expression::Name("invalid_call".to_string())) }
            }
            PythonSyntaxKind::Attribute => {
                let mut value = None;
                let mut attr = String::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Node(n) => {
                            if !n.kind.is_ignored() {
                                value = Some(Box::new(self.build_expression(n, current_offset, source)?));
                            }
                        }
                        GreenTree::Leaf(leaf) if leaf.kind == PythonSyntaxKind::Identifier => {
                            attr = source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string();
                        }
                        _ => {}
                    }
                    current_offset += child_len;
                }

                if let Some(v) = value { Ok(Expression::Attribute { value: v, attr }) } else { Ok(Expression::Name("invalid_attribute".to_string())) }
            }
            _ => Ok(Expression::Name("expr".to_string())),
        }
    }

    fn build_parameters(&self, node: &GreenNode<PythonLanguage>, offset: usize, source: &SourceText) -> Result<Vec<Parameter>, OakError> {
        let mut parameters = Vec::new();
        let mut current_offset = offset;

        for child in node.children() {
            let child_len = child.len() as usize;
            if let GreenTree::Node(n) = child {
                if n.kind == PythonSyntaxKind::Arg {
                    parameters.push(self.build_parameter(n, current_offset, source)?);
                }
            }
            current_offset += child_len;
        }
        Ok(parameters)
    }

    fn build_parameter(&self, node: &GreenNode<PythonLanguage>, offset: usize, source: &SourceText) -> Result<Parameter, OakError> {
        let mut name = String::new();
        let mut current_offset = offset;

        for child in node.children() {
            let child_len = child.len() as usize;
            if let GreenTree::Leaf(leaf) = child {
                if leaf.kind == PythonSyntaxKind::Identifier {
                    name = source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string();
                }
            }
            current_offset += child_len;
        }

        Ok(Parameter { name, annotation: None, default: None })
    }
}
