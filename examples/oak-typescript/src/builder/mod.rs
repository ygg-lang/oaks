use crate::{ast::*, kind::TypeScriptSyntaxKind, language::TypeScriptLanguage, lexer::TypeScriptLexer, parser::TypeScriptParser};
use oak_core::{Builder, BuilderCache, GreenNode, Lexer, OakDiagnostics, OakError, Parser, RedNode, RedTree, SourceText, TextEdit, TokenType, source::Source};

/// TypeScript 语言的 AST 构建器
#[derive(Clone)]
pub struct TypeScriptBuilder<'config> {
    config: &'config TypeScriptLanguage,
}

impl<'config> TypeScriptBuilder<'config> {
    pub fn new(config: &'config TypeScriptLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<TypeScriptLanguage> for TypeScriptBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<TypeScriptLanguage>) -> OakDiagnostics<TypeScriptRoot> {
        let parser = TypeScriptParser::new(self.config);
        let lexer = TypeScriptLexer::new(&self.config);

        let mut session = oak_core::parser::session::ParseSession::<TypeScriptLanguage>::default();
        lexer.lex(source, edits, &mut session);
        let parse_result = parser.parse(source, edits, &mut session);

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
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}

impl<'config> TypeScriptBuilder<'config> {
    pub(crate) fn build_root(&self, green_tree: &GreenNode<TypeScriptLanguage>, source: &SourceText) -> Result<TypeScriptRoot, OakError> {
        let red_root = RedNode::new(green_tree, 0);
        let span = red_root.span();
        let mut statements = Vec::new();

        println!("Root node kind: {:?}", red_root.green.kind);

        if red_root.green.kind == TypeScriptSyntaxKind::SourceFile {
            for child in red_root.children() {
                if let RedTree::Node(node) = child {
                    println!("  Root child kind: {:?}", node.green.kind);
                    if let Some(stmt) = self.build_statement(&node, source)? {
                        statements.push(stmt);
                    }
                }
            }
        }
        else {
            for child in red_root.children() {
                if let RedTree::Node(node) = child {
                    println!("  Root child kind (non-sourcefile): {:?}", node.green.kind);
                    if node.green.kind == TypeScriptSyntaxKind::SourceFile {
                        for sub_child in node.children() {
                            if let RedTree::Node(sub_node) = sub_child {
                                println!("    Sub-child kind: {:?}", sub_node.green.kind);
                                if let Some(stmt) = self.build_statement(&sub_node, source)? {
                                    statements.push(stmt);
                                }
                            }
                        }
                    }
                    else {
                        if let Some(stmt) = self.build_statement(&node, source)? {
                            statements.push(stmt);
                        }
                    }
                }
            }
        }

        println!("Total statements built: {}", statements.len());
        Ok(TypeScriptRoot { statements, span: span.into() })
    }

    fn build_statement(&self, node: &RedNode<TypeScriptLanguage>, source: &SourceText) -> Result<Option<Statement>, OakError> {
        let kind = node.green.kind;
        let span = node.span();
        println!("    Building statement: {:?}", kind);

        match kind {
            TypeScriptSyntaxKind::SourceFile => {
                for child in node.children() {
                    if let RedTree::Node(child_node) = child {
                        if let Some(stmt) = self.build_statement(&child_node, source)? {
                            return Ok(Some(stmt));
                        }
                    }
                }
                Ok(None)
            }
            TypeScriptSyntaxKind::VariableDeclaration => {
                let mut name = String::new();
                let mut value = None;
                for child in node.children() {
                    match child {
                        RedTree::Node(child_node) => {
                            let child_kind = child_node.green.kind;
                            if child_kind == TypeScriptSyntaxKind::IdentifierName {
                                name = source.get_text_in(child_node.span().into()).to_string();
                            }
                            else if child_kind == TypeScriptSyntaxKind::BinaryExpression
                                || child_kind == TypeScriptSyntaxKind::NewExpression
                                || child_kind == TypeScriptSyntaxKind::CallExpression
                                || child_kind == TypeScriptSyntaxKind::MemberExpression
                                || child_kind == TypeScriptSyntaxKind::NumericLiteral
                                || child_kind == TypeScriptSyntaxKind::StringLiteral
                                || child_kind == TypeScriptSyntaxKind::BooleanLiteral
                                || child_kind == TypeScriptSyntaxKind::True
                                || child_kind == TypeScriptSyntaxKind::False
                                || child_kind == TypeScriptSyntaxKind::Null
                            {
                                if value.is_none() {
                                    value = self.build_expression(&child_node, source)?;
                                }
                            }
                            else {
                                // Fallback for other potential expression nodes
                                if value.is_none() {
                                    if let Some(expr) = self.build_expression(&child_node, source)? {
                                        value = Some(expr);
                                    }
                                }
                            }
                        }
                        RedTree::Leaf(leaf) => {
                            if leaf.kind == TypeScriptSyntaxKind::IdentifierName {
                                name = source.get_text_in(leaf.span.into()).to_string();
                            }
                        }
                    }
                }
                Ok(Some(Statement::VariableDeclaration(VariableDeclaration { name, value, span: span.into() })))
            }
            TypeScriptSyntaxKind::FunctionDeclaration => {
                let mut name = String::new();
                let mut params = Vec::new();
                let mut body = Vec::new();

                for child in node.children() {
                    match child {
                        RedTree::Node(child_node) => {
                            let child_kind = child_node.green.kind;
                            if child_kind == TypeScriptSyntaxKind::IdentifierName {
                                if name.is_empty() {
                                    name = source.get_text_in(child_node.span().into()).to_string();
                                }
                            }
                            else if child_kind == TypeScriptSyntaxKind::Parameter {
                                for sub_child in child_node.children() {
                                    match sub_child {
                                        RedTree::Node(sub_node) => {
                                            if sub_node.green.kind == TypeScriptSyntaxKind::IdentifierName {
                                                params.push(source.get_text_in(sub_node.span().into()).to_string());
                                            }
                                        }
                                        RedTree::Leaf(leaf) => {
                                            if leaf.kind == TypeScriptSyntaxKind::IdentifierName {
                                                params.push(source.get_text_in(leaf.span.into()).to_string());
                                            }
                                        }
                                    }
                                }
                            }
                            else if child_kind == TypeScriptSyntaxKind::BlockStatement {
                                for sub_child in child_node.children() {
                                    if let RedTree::Node(sub_node) = sub_child {
                                        if let Some(s) = self.build_statement(&sub_node, source)? {
                                            body.push(s);
                                        }
                                    }
                                }
                            }
                        }
                        RedTree::Leaf(leaf) => {
                            if leaf.kind == TypeScriptSyntaxKind::IdentifierName {
                                if name.is_empty() {
                                    name = source.get_text_in(leaf.span.into()).to_string();
                                }
                            }
                        }
                    }
                }
                Ok(Some(Statement::FunctionDeclaration(FunctionDeclaration { name, params, body, span: span.into() })))
            }
            TypeScriptSyntaxKind::ClassDeclaration => {
                let mut name = String::new();
                let mut extends = None;
                let mut body = Vec::new();

                for child in node.children() {
                    match child {
                        RedTree::Node(child_node) => {
                            let child_kind = child_node.green.kind;
                            if child_kind == TypeScriptSyntaxKind::IdentifierName {
                                if name.is_empty() {
                                    name = source.get_text_in(child_node.span().into()).to_string();
                                }
                                else {
                                    extends = Some(source.get_text_in(child_node.span().into()).to_string());
                                }
                            }
                            else if child_kind == TypeScriptSyntaxKind::ClassBody {
                                // Parse class members
                                for member_child in child_node.children() {
                                    if let RedTree::Node(member_node) = member_child {
                                        let member_kind = member_node.green.kind;
                                        let member_span = member_node.span();
                                        match member_kind {
                                            TypeScriptSyntaxKind::PropertyDeclaration => {
                                                let mut prop_name = String::new();
                                                let mut prop_type = None;
                                                let mut initializer = None;
                                                for p_child in member_node.children() {
                                                    if let RedTree::Node(p_node) = p_child {
                                                        let p_kind = p_node.green.kind;
                                                        if p_kind == TypeScriptSyntaxKind::IdentifierName {
                                                            if prop_name.is_empty() {
                                                                prop_name = source.get_text_in(p_node.span().into()).to_string();
                                                            }
                                                            else {
                                                                prop_type = Some(source.get_text_in(p_node.span().into()).to_string());
                                                            }
                                                        }
                                                        else if let Some(expr) = self.build_expression(&p_node, source)? {
                                                            initializer = Some(expr);
                                                        }
                                                    }
                                                }
                                                let prop_name = prop_name.trim();
                                                if !prop_name.is_empty() && prop_name != ";" {
                                                    body.push(ClassMember::Property { name: prop_name.to_string(), ty: prop_type, initializer, span: member_span.into() });
                                                }
                                            }
                                            TypeScriptSyntaxKind::MethodDeclaration | TypeScriptSyntaxKind::ConstructorDeclaration => {
                                                let mut meth_name = if member_kind == TypeScriptSyntaxKind::ConstructorDeclaration { "constructor".to_string() } else { String::new() };
                                                let mut meth_params = Vec::new();
                                                let mut meth_body = Vec::new();
                                                for m_child in member_node.children() {
                                                    if let RedTree::Node(m_node) = m_child {
                                                        let m_kind = m_node.green.kind;
                                                        if m_kind == TypeScriptSyntaxKind::IdentifierName && meth_name.is_empty() {
                                                            meth_name = source.get_text_in(m_node.span().into()).to_string();
                                                        }
                                                        else if m_kind == TypeScriptSyntaxKind::Parameter {
                                                            for param_child in m_node.children() {
                                                                if let RedTree::Node(pn) = param_child {
                                                                    if pn.green.kind == TypeScriptSyntaxKind::IdentifierName {
                                                                        meth_params.push(source.get_text_in(pn.span().into()).to_string());
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        else if m_kind == TypeScriptSyntaxKind::BlockStatement {
                                                            for b_child in m_node.children() {
                                                                if let RedTree::Node(bn) = b_child {
                                                                    if let Some(s) = self.build_statement(&bn, source)? {
                                                                        meth_body.push(s);
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                let meth_name = meth_name.trim();
                                                if !meth_name.is_empty() {
                                                    body.push(ClassMember::Method { name: meth_name.to_string(), params: meth_params, body: meth_body, span: member_span.into() });
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Some(Statement::ClassDeclaration(ClassDeclaration { name, extends, body, span: span.into() })))
            }
            TypeScriptSyntaxKind::ExpressionStatement => {
                for child in node.children() {
                    if let RedTree::Node(child_node) = child {
                        if let Some(expr) = self.build_expression(&child_node, source)? {
                            return Ok(Some(Statement::ExpressionStatement(expr)));
                        }
                    }
                }
                Ok(None)
            }
            TypeScriptSyntaxKind::ImportDeclaration => {
                let mut module_specifier = String::new();
                let mut imports = Vec::new();

                fn walk_import(node: &RedNode<TypeScriptLanguage>, source: &SourceText, module_specifier: &mut String, imports: &mut Vec<String>) {
                    for child in node.children() {
                        match child {
                            RedTree::Node(child_node) => match child_node.green.kind {
                                TypeScriptSyntaxKind::StringLiteral => {
                                    let text = source.get_text_in(child_node.span().into());
                                    if text.len() >= 2 {
                                        *module_specifier = text[1..text.len() - 1].to_string();
                                    }
                                }
                                TypeScriptSyntaxKind::IdentifierName => {
                                    imports.push(source.get_text_in(child_node.span().into()).to_string());
                                }
                                _ => walk_import(&child_node, source, module_specifier, imports),
                            },
                            RedTree::Leaf(leaf) => match leaf.kind {
                                TypeScriptSyntaxKind::StringLiteral => {
                                    let text = source.get_text_in(leaf.span.into());
                                    if text.len() >= 2 {
                                        *module_specifier = text[1..text.len() - 1].to_string();
                                    }
                                }
                                TypeScriptSyntaxKind::IdentifierName => {
                                    imports.push(source.get_text_in(leaf.span.into()).to_string());
                                }
                                _ => {}
                            },
                        }
                    }
                }

                walk_import(node, source, &mut module_specifier, &mut imports);

                Ok(Some(Statement::ImportDeclaration(ImportDeclaration { module_specifier, imports, span: span.into() })))
            }
            TypeScriptSyntaxKind::ExportDeclaration => {
                for child in node.children() {
                    if let RedTree::Node(child_node) = child {
                        if let Some(stmt) = self.build_statement(&child_node, source)? {
                            return Ok(Some(Statement::ExportDeclaration(ExportDeclaration { declaration: Box::new(stmt), span: span.into() })));
                        }
                    }
                }
                Ok(None)
            }
            TypeScriptSyntaxKind::ReturnStatement => {
                let mut value = None;
                for child in node.children() {
                    if let RedTree::Node(child_node) = child {
                        if let Some(expr) = self.build_expression(&child_node, source)? {
                            value = Some(expr);
                            break;
                        }
                    }
                }
                Ok(Some(Statement::ReturnStatement(value)))
            }
            _ => Ok(None),
        }
    }

    fn build_expression(&self, node: &RedNode<TypeScriptLanguage>, source: &SourceText) -> Result<Option<Expression>, OakError> {
        let kind = node.green.kind;

        match kind {
            TypeScriptSyntaxKind::IdentifierName => Ok(Some(Expression::Identifier(source.get_text_in(node.span().into()).to_string()))),
            TypeScriptSyntaxKind::NumericLiteral => Ok(Some(Expression::NumericLiteral(source.get_text_in(node.span().into()).parse().unwrap_or(0.0)))),
            TypeScriptSyntaxKind::StringLiteral => {
                let text = source.get_text_in(node.span().into());
                if text.len() >= 2 { Ok(Some(Expression::StringLiteral(text[1..text.len() - 1].to_string()))) } else { Ok(Some(Expression::StringLiteral(text.to_string()))) }
            }
            TypeScriptSyntaxKind::BigIntLiteral => Ok(Some(Expression::BigIntLiteral(source.get_text_in(node.span().into()).to_string()))),
            TypeScriptSyntaxKind::True => Ok(Some(Expression::BooleanLiteral(true))),
            TypeScriptSyntaxKind::False => Ok(Some(Expression::BooleanLiteral(false))),
            TypeScriptSyntaxKind::CallExpression => {
                let mut func = None;
                let mut args = Vec::new();
                for child in node.children() {
                    if let RedTree::Node(child_node) = child {
                        if func.is_none() {
                            func = self.build_expression(&child_node, source)?;
                        }
                        else if child_node.green.kind == TypeScriptSyntaxKind::CallArgument {
                            for sub_child in child_node.children() {
                                if let RedTree::Node(sub_node) = sub_child {
                                    if let Some(arg) = self.build_expression(&sub_node, source)? {
                                        args.push(arg);
                                    }
                                }
                            }
                        }
                        else if let Some(arg) = self.build_expression(&child_node, source)? {
                            args.push(arg);
                        }
                    }
                }
                if let Some(f) = func { Ok(Some(Expression::CallExpression { func: Box::new(f), args })) } else { Ok(None) }
            }

            TypeScriptSyntaxKind::NewExpression => {
                let mut func = None;
                let mut args = Vec::new();
                for child in node.children() {
                    if let RedTree::Node(child_node) = child {
                        if func.is_none() {
                            func = self.build_expression(&child_node, source)?;
                        }
                        else if child_node.green.kind == TypeScriptSyntaxKind::CallArgument {
                            for sub_child in child_node.children() {
                                if let RedTree::Node(sub_node) = sub_child {
                                    if let Some(arg) = self.build_expression(&sub_node, source)? {
                                        args.push(arg);
                                    }
                                }
                            }
                        }
                        else if let Some(arg) = self.build_expression(&child_node, source)? {
                            args.push(arg);
                        }
                    }
                }
                if let Some(f) = func { Ok(Some(Expression::NewExpression { func: Box::new(f), args })) } else { Ok(None) }
            }

            TypeScriptSyntaxKind::MemberExpression => {
                let mut object = None;
                let mut property = None;
                let mut computed = false;
                for child in node.children() {
                    match child {
                        RedTree::Node(child_node) => {
                            if object.is_none() {
                                object = self.build_expression(&child_node, source)?;
                            }
                            else {
                                property = self.build_expression(&child_node, source)?;
                            }
                        }
                        RedTree::Leaf(leaf) => {
                            if leaf.kind == TypeScriptSyntaxKind::LeftBracket {
                                computed = true;
                            }
                        }
                    }
                }
                if let (Some(obj), Some(prop)) = (object, property) { Ok(Some(Expression::MemberExpression { object: Box::new(obj), property: Box::new(prop), computed, optional: false })) } else { Ok(None) }
            }

            TypeScriptSyntaxKind::AsExpression => {
                let mut expression = None;
                let mut type_annotation = String::new();
                for child in node.children() {
                    match child {
                        RedTree::Node(child_node) => {
                            if expression.is_none() {
                                expression = self.build_expression(&child_node, source)?;
                            }
                            else {
                                type_annotation = source.get_text_in(child_node.span().into()).to_string();
                            }
                        }
                        RedTree::Leaf(leaf) => {
                            if expression.is_some() && leaf.kind == TypeScriptSyntaxKind::IdentifierName {
                                type_annotation = source.get_text_in(leaf.span.into()).to_string();
                            }
                        }
                    }
                }
                if let Some(expr) = expression { Ok(Some(Expression::AsExpression { expression: Box::new(expr), type_annotation })) } else { Ok(None) }
            }

            TypeScriptSyntaxKind::Null => Ok(Some(Expression::NullLiteral)),
            TypeScriptSyntaxKind::RegexLiteral => Ok(Some(Expression::RegexLiteral(source.get_text_in(node.span().into()).to_string()))),
            TypeScriptSyntaxKind::TemplateString => Ok(Some(Expression::TemplateString(source.get_text_in(node.span().into()).to_string()))),

            TypeScriptSyntaxKind::UnaryExpression => {
                let mut operator = String::new();
                let mut argument = None;
                for child in node.children() {
                    match child {
                        RedTree::Leaf(leaf) => {
                            if operator.is_empty() {
                                operator = source.get_text_in(leaf.span.into()).to_string();
                            }
                        }
                        RedTree::Node(child_node) => {
                            argument = self.build_expression(&child_node, source)?;
                        }
                    }
                }
                if let Some(arg) = argument { Ok(Some(Expression::UnaryExpression { operator, argument: Box::new(arg) })) } else { Ok(None) }
            }

            TypeScriptSyntaxKind::BinaryExpression => {
                let mut left = None;
                let mut operator = String::new();
                let mut right = None;

                for child in node.children() {
                    match child {
                        RedTree::Node(child_node) => {
                            if left.is_none() {
                                left = self.build_expression(&child_node, source)?;
                            }
                            else {
                                right = self.build_expression(&child_node, source)?;
                            }
                        }
                        RedTree::Leaf(leaf) => {
                            if left.is_some() && operator.is_empty() && !leaf.kind.is_ignored() {
                                operator = source.get_text_in(leaf.span.into()).to_string();
                            }
                        }
                    }
                }

                if let (Some(l), Some(r)) = (left, right) {
                    if operator == "="
                        || operator == "+="
                        || operator == "-="
                        || operator == "*="
                        || operator == "/="
                        || operator == "%="
                        || operator == "**="
                        || operator == "<<="
                        || operator == ">>="
                        || operator == ">>>="
                        || operator == "&="
                        || operator == "|="
                        || operator == "^="
                        || operator == "&&="
                        || operator == "||="
                        || operator == "??="
                    {
                        Ok(Some(Expression::AssignmentExpression { left: Box::new(l), operator, right: Box::new(r) }))
                    }
                    else {
                        Ok(Some(Expression::BinaryExpression { left: Box::new(l), operator, right: Box::new(r) }))
                    }
                }
                else {
                    Ok(None)
                }
            }

            TypeScriptSyntaxKind::ConditionalExpression => {
                let mut parts = Vec::new();
                for child in node.children() {
                    if let RedTree::Node(child_node) = child {
                        if let Some(expr) = self.build_expression(&child_node, source)? {
                            parts.push(expr);
                        }
                    }
                }
                if parts.len() == 3 {
                    let mut iter = parts.into_iter();
                    Ok(Some(Expression::ConditionalExpression { test: Box::new(iter.next().unwrap()), consequent: Box::new(iter.next().unwrap()), alternate: Box::new(iter.next().unwrap()) }))
                }
                else {
                    Ok(None)
                }
            }

            _ => {
                // 如果是一个包裹节点，递归处理
                for child in node.children() {
                    if let RedTree::Node(child_node) = child {
                        if let Some(expr) = self.build_expression(&child_node, source)? {
                            return Ok(Some(expr));
                        }
                    }
                }
                Ok(None)
            }
        }
    }
}
