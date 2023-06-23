use crate::{
    ast::{Expression, Literal, Parameter, Program, Statement, SwiftRoot, Type},
    language::SwiftLanguage,
    lexer::token_type::SwiftTokenType,
    parser::{SwiftParser, element_type::SwiftElementType},
};
use oak_core::{Builder, BuilderCache, GreenNode, GreenTree, OakDiagnostics, OakError, Parser, SourceText, TextEdit, builder::BuildOutput, source::Source};

#[derive(Clone)]
pub struct SwiftBuilder<'config> {
    config: &'config SwiftLanguage,
}

impl<'config> SwiftBuilder<'config> {
    pub fn new(config: &'config SwiftLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<SwiftLanguage> for SwiftBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<SwiftLanguage>) -> BuildOutput<SwiftLanguage> {
        let parser = SwiftParser::new(self.config);
        let mut parse_cache = oak_core::parser::session::ParseSession::<SwiftLanguage>::default();
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

impl<'config> SwiftBuilder<'config> {
    pub fn build_root(&self, green_tree: &GreenNode<SwiftLanguage>, source: &SourceText) -> Result<SwiftRoot, OakError> {
        let mut statements = Vec::new();
        let mut current_offset = 0;

        for child in green_tree.children() {
            let child_len = child.len() as usize;
            match child {
                GreenTree::Node(node) => {
                    if !node.kind.is_trivia() {
                        if let Some(stmt) = self.build_statement(node, current_offset, source)? {
                            statements.push(stmt);
                        }
                    }
                }
                _ => {}
            }
            current_offset += child_len;
        }

        Ok(SwiftRoot { program: Program { statements }, span: (0..green_tree.text_len() as usize).into() })
    }

    fn build_statement(&self, node: &GreenNode<SwiftLanguage>, offset: usize, source: &SourceText) -> Result<Option<Statement>, OakError> {
        // eprintln!("Building statement: {:?} at {}", node.kind, offset);
        match node.kind {
            SwiftElementType::FunctionDeclaration => {
                let mut name = String::new();
                let mut parameters = Vec::new();
                let mut body = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Leaf(leaf) if leaf.kind == SwiftTokenType::Identifier => {
                            if name.is_empty() {
                                name = source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string();
                                eprintln!("Found identifier: '{}' at {}", name, current_offset);
                            }
                        }
                        GreenTree::Node(n) if n.kind == SwiftElementType::ParameterList => {
                            parameters = self.build_parameters(n, current_offset, source)?;
                        }
                        GreenTree::Node(n) if n.kind == SwiftElementType::Block => {
                            body = self.build_block(n, current_offset, source)?;
                        }
                        _ => {}
                    }
                    current_offset += child_len;
                }
                Ok(Some(Statement::FunctionDef { name, parameters, return_type: None, body }))
            }
            SwiftElementType::VariableDeclaration => {
                let mut name = String::new();
                let mut value = None;
                let mut is_mutable = true;
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Leaf(leaf) => match leaf.kind {
                            SwiftTokenType::Let => is_mutable = false,
                            SwiftTokenType::Var => is_mutable = true,
                            SwiftTokenType::Identifier => {
                                name = source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string();
                            }
                            _ => {}
                        },
                        GreenTree::Node(n) => {
                            if !n.kind.is_trivia() && n.kind != SwiftElementType::Identifier && n.kind != SwiftElementType::Colon {
                                value = Some(self.build_expression(n, current_offset, source)?);
                            }
                        }
                    }
                    current_offset += child_len;
                }
                Ok(Some(Statement::VariableDecl { is_mutable, name, type_annotation: None, value }))
            }
            SwiftElementType::ReturnStatement => {
                let mut value = None;
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            value = Some(self.build_expression(n, current_offset, source)?);
                        }
                    }
                    current_offset += child_len;
                }
                Ok(Some(Statement::Return(value)))
            }
            SwiftElementType::IfStatement => {
                let mut test = None;
                let mut body = Vec::new();
                let mut orelse = None;
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Node(n) if n.kind == SwiftElementType::Block => {
                            if body.is_empty() {
                                body = self.build_block(n, current_offset, source)?;
                            }
                            else {
                                orelse = Some(self.build_block(n, current_offset, source)?);
                            }
                        }
                        GreenTree::Node(n) if n.kind == SwiftElementType::IfStatement => {
                            orelse = Some(vec![self.build_statement(n, current_offset, source)?.unwrap()]);
                        }
                        GreenTree::Node(n) if !n.kind.is_trivia() => {
                            test = Some(self.build_expression(n, current_offset, source)?);
                        }
                        _ => {}
                    }
                    current_offset += child_len;
                }
                Ok(Some(Statement::If { test: test.unwrap(), body, orelse }))
            }
            SwiftElementType::WhileStatement => {
                let mut test = None;
                let mut body = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Node(n) if n.kind == SwiftElementType::Block => {
                            body = self.build_block(n, current_offset, source)?;
                        }
                        GreenTree::Node(n) if !n.kind.is_trivia() => {
                            test = Some(self.build_expression(n, current_offset, source)?);
                        }
                        _ => {}
                    }
                    current_offset += child_len;
                }
                Ok(Some(Statement::While { test: test.unwrap(), body }))
            }
            SwiftElementType::ForStatement => {
                let mut variable = String::new();
                let mut iterable = None;
                let mut body = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Leaf(leaf) if leaf.kind == SwiftTokenType::Identifier => {
                            variable = source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string();
                        }
                        GreenTree::Node(n) if n.kind == SwiftElementType::Block => {
                            body = self.build_block(n, current_offset, source)?;
                        }
                        GreenTree::Node(n) if !n.kind.is_trivia() => {
                            iterable = Some(self.build_expression(n, current_offset, source)?);
                        }
                        _ => {}
                    }
                    current_offset += child_len;
                }
                Ok(Some(Statement::For { variable, iterable: iterable.unwrap(), body }))
            }
            SwiftElementType::ExpressionStatement => {
                let mut expr = None;
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            expr = Some(self.build_expression(n, current_offset, source)?);
                        }
                    }
                    current_offset += child_len;
                }
                Ok(expr.map(Statement::Expression))
            }
            SwiftElementType::Block => Ok(Some(Statement::Block(self.build_block(node, offset, source)?))),
            _ => Ok(None),
        }
    }

    fn build_block(&self, node: &GreenNode<SwiftLanguage>, offset: usize, source: &SourceText) -> Result<Vec<Statement>, OakError> {
        let mut statements = Vec::new();
        let mut current_offset = offset;

        for child in node.children() {
            let child_len = child.len() as usize;
            if let GreenTree::Node(n) = child {
                if !n.kind.is_trivia() {
                    if let Some(stmt) = self.build_statement(n, current_offset, source)? {
                        statements.push(stmt);
                    }
                }
            }
            current_offset += child_len;
        }
        Ok(statements)
    }

    fn build_parameters(&self, node: &GreenNode<SwiftLanguage>, offset: usize, source: &SourceText) -> Result<Vec<Parameter>, OakError> {
        let mut parameters = Vec::new();
        let mut current_offset = offset;

        for child in node.children() {
            let child_len = child.len() as usize;
            if let GreenTree::Node(n) = child {
                if n.kind == SwiftElementType::Parameter {
                    parameters.push(self.build_parameter(n, current_offset, source)?);
                }
            }
            current_offset += child_len;
        }
        Ok(parameters)
    }

    fn build_parameter(&self, node: &GreenNode<SwiftLanguage>, offset: usize, source: &SourceText) -> Result<Parameter, OakError> {
        let mut name = String::new();
        let mut type_name = String::new();
        let mut current_offset = offset;

        for child in node.children() {
            let child_len = child.len() as usize;
            match child {
                GreenTree::Leaf(leaf) if leaf.kind == SwiftTokenType::Identifier => {
                    if name.is_empty() {
                        name = source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string();
                        eprintln!("Found parameter name: '{}' at {}", name, current_offset);
                    }
                    else {
                        type_name = source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string();
                        eprintln!("Found parameter type: '{}' at {}", type_name, current_offset);
                    }
                }
                _ => {}
            }
            current_offset += child_len;
        }

        Ok(Parameter { name, type_annotation: Type { name: type_name } })
    }

    fn build_expression(&self, node: &GreenNode<SwiftLanguage>, offset: usize, source: &SourceText) -> Result<Expression, OakError> {
        match node.kind {
            SwiftElementType::BinaryExpression => {
                let mut left = None;
                let mut operator = String::new();
                let mut right = None;
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Node(n) => {
                            if left.is_none() {
                                left = Some(Box::new(self.build_expression(n, current_offset, source)?));
                            }
                            else {
                                right = Some(Box::new(self.build_expression(n, current_offset, source)?));
                            }
                        }
                        GreenTree::Leaf(leaf) if !leaf.kind.is_trivia() => {
                            operator = source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string();
                        }
                        _ => {}
                    }
                    current_offset += child_len;
                }
                Ok(Expression::Binary { left: left.unwrap(), operator, right: right.unwrap() })
            }
            SwiftElementType::UnaryExpression => {
                let mut operator = String::new();
                let mut operand = None;
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Leaf(leaf) if !leaf.kind.is_trivia() => {
                            operator = source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string();
                        }
                        GreenTree::Node(n) => {
                            operand = Some(Box::new(self.build_expression(n, current_offset, source)?));
                        }
                        _ => {}
                    }
                    current_offset += child_len;
                }
                Ok(Expression::Unary { operator, operand: operand.unwrap() })
            }
            SwiftElementType::IdentifierExpression => {
                let text = source.get_text_in((offset..offset + node.text_len() as usize).into()).trim().to_string();
                Ok(Expression::Identifier(text))
            }
            SwiftElementType::LiteralExpression => {
                let text = source.get_text_in((offset..offset + node.text_len() as usize).into()).trim().to_string();
                // Simple literal detection
                if text == "true" {
                    Ok(Expression::Literal(Literal::Boolean(true)))
                }
                else if text == "false" {
                    Ok(Expression::Literal(Literal::Boolean(false)))
                }
                else if text == "nil" {
                    Ok(Expression::Literal(Literal::Nil))
                }
                else if text.starts_with('"') {
                    Ok(Expression::Literal(Literal::String(text[1..text.len() - 1].to_string())))
                }
                else {
                    Ok(Expression::Literal(Literal::Number(text)))
                }
            }
            SwiftElementType::CallExpression => {
                let mut callee = None;
                let mut arguments = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Node(n) if callee.is_none() => {
                            callee = Some(Box::new(self.build_expression(n, current_offset, source)?));
                        }
                        GreenTree::Node(n) => {
                            arguments.push(self.build_expression(n, current_offset, source)?);
                        }
                        _ => {}
                    }
                    current_offset += child_len;
                }
                Ok(Expression::Call { callee: callee.unwrap(), arguments })
            }
            SwiftElementType::MemberExpression => {
                let mut object = None;
                let mut member = String::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Node(n) if !n.kind.is_trivia() => {
                            object = Some(Box::new(self.build_expression(n, current_offset, source)?));
                        }
                        GreenTree::Leaf(leaf) if leaf.kind == SwiftTokenType::Identifier => {
                            member = source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string();
                        }
                        _ => {}
                    }
                    current_offset += child_len;
                }
                Ok(Expression::Member { object: object.unwrap(), member })
            }
            _ => Err(OakError::custom_error(format!("Unsupported expression kind: {:?}", node.kind))),
        }
    }
}
