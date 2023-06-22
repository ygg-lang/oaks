use crate::{ast::*, kind::JavaSyntaxKind, language::JavaLanguage, parser::JavaParser};
use oak_core::{
    GreenNode, Parser,
    builder::{BuildOutput, Builder, BuilderCache},
    source::{Source, TextEdit},
    tree::red_tree::{RedNode, RedTree},
};

pub struct JavaBuilder<'config> {
    language: &'config JavaLanguage,
}

impl<'config> JavaBuilder<'config> {
    pub fn new(language: &'config JavaLanguage) -> Self {
        Self { language }
    }

    fn build_root(&self, green: &GreenNode<JavaLanguage>, source: &str) -> JavaRoot {
        let red = RedNode::new(green, 0);
        let mut items = Vec::new();

        for child in red.children() {
            if let RedTree::Node(node) = child {
                match node.green.kind {
                    JavaSyntaxKind::CompilationUnit => {
                        for sub_child in node.children() {
                            if let RedTree::Node(sub_node) = sub_child {
                                if let Some(item) = self.build_item(sub_node, source) {
                                    items.push(item);
                                }
                            }
                        }
                    }
                    _ => {
                        if let Some(item) = self.build_item(node, source) {
                            items.push(item);
                        }
                    }
                }
            }
        }

        JavaRoot { items }
    }

    fn build_item(&self, node: RedNode<JavaLanguage>, source: &str) -> Option<Item> {
        match node.green.kind {
            JavaSyntaxKind::ClassDeclaration => Some(Item::Class(self.build_class(node, source))),
            JavaSyntaxKind::InterfaceDeclaration => Some(Item::Interface(InterfaceDeclaration { name: self.extract_identifier(node, source), span: node.span() })),
            JavaSyntaxKind::Package => Some(Item::Package(PackageDeclaration { name: self.extract_identifier(node, source), span: node.span() })),
            JavaSyntaxKind::Import => Some(Item::Import(ImportDeclaration {
                path: self.extract_identifier(node, source),
                is_static: false, // TODO: Check for static keyword
                span: node.span(),
            })),
            _ => None,
        }
    }

    fn get_text<'a>(&self, span: core::range::Range<usize>, source: &'a str) -> &'a str {
        let start = span.start.min(source.len());
        let end = span.end.min(source.len());
        &source[start..end]
    }

    fn extract_identifier(&self, node: RedNode<JavaLanguage>, source: &str) -> String {
        for child in node.children() {
            match child {
                RedTree::Leaf(leaf) => {
                    if leaf.kind == JavaSyntaxKind::Identifier {
                        let name = self.get_text(leaf.span, source).to_string();
                        eprintln!("DEBUG: Found identifier '{}' at {:?}", name, leaf.span);
                        return name;
                    }
                }
                RedTree::Node(sub_node) => {
                    let res = self.extract_identifier(sub_node, source);
                    if !res.is_empty() {
                        return res;
                    }
                }
            }
        }
        String::new()
    }

    fn build_class(&self, node: RedNode<JavaLanguage>, source: &str) -> ClassDeclaration {
        let name = self.extract_identifier(node, source);
        let mut members = Vec::new();

        eprintln!("DEBUG: Building class '{}' with {} children", name, node.children().count());
        for child in node.children() {
            match child {
                RedTree::Node(sub_node) => {
                    eprintln!("DEBUG: Child node kind: {:?}", sub_node.green.kind);
                    match sub_node.green.kind {
                        JavaSyntaxKind::MethodDeclaration => {
                            eprintln!("DEBUG: Found MethodDeclaration");
                            members.push(Member::Method(self.build_method(sub_node, source)));
                        }
                        JavaSyntaxKind::FieldDeclaration => {
                            eprintln!("DEBUG: Found FieldDeclaration");
                            members.push(Member::Field(self.build_field(sub_node, source)));
                        }
                        _ => {
                            // Recursively look for members in case they are wrapped in a block
                            self.collect_members(sub_node, source, &mut members);
                        }
                    }
                }
                RedTree::Leaf(leaf) => {
                    eprintln!("DEBUG: Child leaf kind: {:?}", leaf.kind);
                }
            }
        }

        ClassDeclaration { name, members, span: node.span() }
    }

    fn collect_members(&self, node: RedNode<JavaLanguage>, source: &str, members: &mut Vec<Member>) {
        for child in node.children() {
            if let RedTree::Node(sub_node) = child {
                eprintln!("DEBUG: collect_members kind: {:?}", sub_node.green.kind);
                match sub_node.green.kind {
                    JavaSyntaxKind::MethodDeclaration => {
                        eprintln!("DEBUG: collect_members Found MethodDeclaration");
                        members.push(Member::Method(self.build_method(sub_node, source)));
                    }
                    JavaSyntaxKind::FieldDeclaration => {
                        eprintln!("DEBUG: collect_members Found FieldDeclaration");
                        members.push(Member::Field(self.build_field(sub_node, source)));
                    }
                    _ => {
                        self.collect_members(sub_node, source, members);
                    }
                }
            }
        }
    }

    fn build_method(&self, node: RedNode<JavaLanguage>, source: &str) -> MethodDeclaration {
        let name_debug = self.extract_identifier(node, source);
        eprintln!("DEBUG: Building method '{}'", name_debug);
        let mut name = String::new();
        let mut return_type = String::new();
        let mut parameters = Vec::new();
        let mut body = Vec::new();
        let mut is_static = false;

        for child in node.children() {
            match child {
                RedTree::Leaf(leaf) => match leaf.kind {
                    JavaSyntaxKind::Identifier => {
                        if return_type.is_empty() {
                            return_type = self.get_text(leaf.span, source).to_string();
                        }
                        else {
                            name = self.get_text(leaf.span, source).to_string();
                        }
                    }
                    JavaSyntaxKind::Static => is_static = true,
                    JavaSyntaxKind::Void | JavaSyntaxKind::Int | JavaSyntaxKind::Boolean => {
                        return_type = self.get_text(leaf.span, source).to_string();
                    }
                    _ => {}
                },
                RedTree::Node(sub_node) => match sub_node.green.kind {
                    JavaSyntaxKind::Parameter => {
                        parameters.push(self.build_parameter(sub_node, source));
                    }
                    JavaSyntaxKind::BlockStatement => {
                        eprintln!("DEBUG: Found block statement for method '{}'", name_debug);
                        body = self.build_block(sub_node, source);
                    }
                    _ => {}
                },
            }
        }

        if return_type.is_empty() {
            return_type = "void".to_string();
        }

        eprintln!("DEBUG: Method '{}' has {} statements in body", name, body.len());
        MethodDeclaration { name, return_type, parameters, body, is_static, span: node.span() }
    }

    fn build_parameter(&self, node: RedNode<JavaLanguage>, source: &str) -> Parameter {
        let mut name = String::new();
        let mut r#type = String::new();
        let mut is_array = false;

        for child in node.children() {
            match child {
                RedTree::Leaf(leaf) => match leaf.kind {
                    JavaSyntaxKind::Identifier => {
                        if r#type.is_empty() {
                            r#type = self.get_text(leaf.span, source).to_string();
                        }
                        else {
                            name = self.get_text(leaf.span, source).to_string();
                        }
                    }
                    JavaSyntaxKind::Int | JavaSyntaxKind::Boolean | JavaSyntaxKind::Void => {
                        r#type = self.get_text(leaf.span, source).to_string();
                    }
                    JavaSyntaxKind::LeftBracket => {
                        is_array = true;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        if is_array {
            r#type.push_str("[]");
        }

        Parameter { name, r#type }
    }

    fn build_field(&self, node: RedNode<JavaLanguage>, source: &str) -> FieldDeclaration {
        let mut name = String::new();
        let mut r#type = String::new();

        for child in node.children() {
            if let RedTree::Leaf(leaf) = child {
                match leaf.kind {
                    JavaSyntaxKind::Identifier => {
                        if r#type.is_empty() {
                            r#type = self.get_text(leaf.span, source).to_string();
                        }
                        else {
                            name = self.get_text(leaf.span, source).to_string();
                        }
                    }
                    JavaSyntaxKind::Int | JavaSyntaxKind::Boolean | JavaSyntaxKind::Void => {
                        r#type = self.get_text(leaf.span, source).to_string();
                    }
                    _ => {}
                }
            }
        }

        FieldDeclaration { name, r#type, span: node.span() }
    }

    fn build_block(&self, node: RedNode<JavaLanguage>, source: &str) -> Vec<Statement> {
        let mut statements = Vec::new();
        for child in node.children() {
            if let RedTree::Node(sub_node) = child {
                statements.push(self.build_statement(sub_node, source));
            }
        }
        statements
    }

    fn build_statement(&self, node: RedNode<JavaLanguage>, source: &str) -> Statement {
        match node.green.kind {
            JavaSyntaxKind::ExpressionStatement => {
                eprintln!("DEBUG: ExpressionStatement children count: {}", node.children().count());
                let mut last_expr_node = None;
                for (i, child) in node.children().enumerate() {
                    match &child {
                        RedTree::Node(sub_node) => {
                            eprintln!("  Child {}: Node({:?})", i, sub_node.green.kind);
                            last_expr_node = Some(sub_node.clone());
                        }
                        RedTree::Leaf(leaf) => eprintln!("  Child {}: Leaf({:?})", i, leaf.kind),
                    }
                }
                if let Some(sub_node) = last_expr_node {
                    return Statement::Expression(self.build_expression(sub_node, source));
                }
                Statement::Block(vec![]) // Should not happen
            }
            JavaSyntaxKind::BlockStatement => Statement::Block(self.build_block(node, source)),
            JavaSyntaxKind::ReturnStatement => {
                let mut expr = None;
                for child in node.children() {
                    if let RedTree::Node(sub_node) = child {
                        expr = Some(self.build_expression(sub_node, source));
                        break;
                    }
                }
                Statement::Return(expr)
            }
            _ => Statement::Block(vec![]),
        }
    }

    fn build_expression(&self, node: RedNode<JavaLanguage>, source: &str) -> Expression {
        eprintln!("DEBUG: build_expression kind: {:?}", node.green.kind);
        match node.green.kind {
            JavaSyntaxKind::LiteralExpression => {
                for child in node.children() {
                    if let RedTree::Leaf(leaf) = child {
                        match leaf.kind {
                            JavaSyntaxKind::IntegerLiteral => {
                                let val = self.get_text(leaf.span, source).parse().unwrap_or(0);
                                return Expression::Literal(Literal::Integer(val));
                            }
                            JavaSyntaxKind::StringLiteral => {
                                let text = self.get_text(leaf.span, source);
                                let content = if text.len() >= 2 { &text[1..text.len() - 1] } else { text };
                                return Expression::Literal(Literal::String(content.to_string()));
                            }
                            JavaSyntaxKind::BooleanLiteral => {
                                let val = self.get_text(leaf.span, source) == "true";
                                return Expression::Literal(Literal::Boolean(val));
                            }
                            _ => {}
                        }
                    }
                }
                Expression::Identifier("unknown".to_string())
            }
            JavaSyntaxKind::MethodCall => {
                let mut target = None;
                let mut name = String::new();
                let mut arguments = Vec::new();
                let mut first_node = true;

                for child in node.children() {
                    match child {
                        RedTree::Node(sub_node) => {
                            if first_node {
                                first_node = false;
                                let expr = self.build_expression(sub_node.clone(), source);
                                match expr {
                                    Expression::FieldAccess(fa) => {
                                        target = Some(fa.target);
                                        name = fa.name;
                                    }
                                    Expression::Identifier(id) => {
                                        name = id;
                                    }
                                    _ => {
                                        target = Some(Box::new(expr));
                                    }
                                }
                            }
                            else {
                                arguments.push(self.build_expression(sub_node, source));
                            }
                        }
                        RedTree::Leaf(leaf) if leaf.kind == JavaSyntaxKind::Identifier => {
                            if name.is_empty() {
                                name = self.get_text(leaf.span, source).to_string();
                            }
                        }
                        _ => {}
                    }
                }
                eprintln!("DEBUG: Built MethodCall: name={}, args={}", name, arguments.len());
                Expression::MethodCall(MethodCall { target, name, arguments })
            }
            JavaSyntaxKind::MemberSelect => {
                let mut target = None;
                let mut name = String::new();

                for child in node.children() {
                    match child {
                        RedTree::Node(sub_node) => {
                            if target.is_none() {
                                target = Some(Box::new(self.build_expression(sub_node.clone(), source)));
                            }
                            else if sub_node.green.kind == JavaSyntaxKind::Identifier {
                                name = self.extract_identifier(sub_node.clone(), source);
                            }
                        }
                        RedTree::Leaf(leaf) if leaf.kind == JavaSyntaxKind::Identifier => {
                            name = self.get_text(leaf.span, source).to_string();
                        }
                        _ => {}
                    }
                }
                eprintln!("DEBUG: Built MemberSelect: name={}", name);
                if let Some(target) = target { Expression::FieldAccess(FieldAccess { target, name }) } else { Expression::Identifier(name) }
            }
            _ => {
                // Check for Identifier leaf directly
                for child in node.children() {
                    if let RedTree::Leaf(leaf) = child {
                        if leaf.kind == JavaSyntaxKind::Identifier {
                            let name = self.get_text(leaf.span, source).to_string();
                            eprintln!("DEBUG: Built Identifier: {}", name);
                            return Expression::Identifier(name);
                        }
                    }
                    else if let RedTree::Node(sub_node) = child {
                        return self.build_expression(sub_node, source);
                    }
                }
                Expression::Identifier("unknown".to_string())
            }
        }
    }
}

impl<'config> Builder<JavaLanguage> for JavaBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, text: &S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<JavaLanguage>) -> BuildOutput<JavaLanguage> {
        let parser = JavaParser::new(self.language);
        let output = parser.parse(text, edits, cache);
        let result = output.result.map(|green| self.build_root(green, &text.get_text_in((0..text.length()).into())));
        oak_core::errors::OakDiagnostics { result, diagnostics: output.diagnostics }
    }
}
