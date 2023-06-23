use crate::{ast::*, language::CSharpLanguage, lexer::token_type::CSharpTokenType, parser::CSharpElementType};
use core::range::Range;
use oak_core::{
    GreenNode, Parser, TokenType,
    builder::{BuildOutput, Builder, BuilderCache},
    source::{Source, TextEdit},
    tree::red_tree::{RedNode, RedTree},
};

pub struct CSharpBuilder<'config> {
    language: &'config CSharpLanguage,
}

impl<'config> CSharpBuilder<'config> {
    pub fn new(language: &'config CSharpLanguage) -> Self {
        Self { language }
    }

    fn build_root(&self, green: &GreenNode<CSharpLanguage>, source: &str) -> CSharpRoot {
        let red = RedNode::new(green, 0);
        let mut items = Vec::new();

        for child in red.children() {
            if let RedTree::Node(node) = child {
                if let Some(item) = self.build_item(node, source) {
                    items.push(item);
                }
            }
        }

        CSharpRoot { items }
    }

    fn build_item(&self, node: RedNode<CSharpLanguage>, source: &str) -> Option<Item> {
        match node.green.kind {
            CSharpElementType::NamespaceDeclaration => Some(Item::Namespace(self.build_namespace(node, source))),
            CSharpElementType::UsingDirective => Some(Item::Using(self.build_using(node, source))),
            CSharpElementType::ClassDeclaration => Some(Item::Class(self.build_class(node, source))),
            CSharpElementType::InterfaceDeclaration => Some(Item::Interface(self.build_interface(node, source))),
            CSharpElementType::StructDeclaration => Some(Item::Struct(self.build_struct(node, source))),
            CSharpElementType::EnumDeclaration => Some(Item::Enum(self.build_enum(node, source))),
            CSharpElementType::RecordDeclaration => Some(Item::Record(self.build_record(node, source))),
            CSharpElementType::DelegateDeclaration => Some(Item::Delegate(self.build_delegate(node, source))),
            _ => None,
        }
    }

    fn get_text<'a>(&self, span: Range<usize>, source: &'a str) -> &'a str {
        let start = span.start;
        let end = span.end;
        if start > source.len() || end > source.len() || start > end {
            return "";
        }
        &source[start..end]
    }

    fn extract_identifier(&self, node: RedNode<CSharpLanguage>, source: &str) -> String {
        for child in node.children() {
            match child {
                RedTree::Leaf(leaf) => {
                    if leaf.kind == CSharpTokenType::Identifier {
                        return self.get_text(leaf.span, source).trim().to_string();
                    }
                }
                RedTree::Node(sub_node) => {
                    if sub_node.kind::<CSharpElementType>() == CSharpElementType::IdentifierName {
                        return self.get_text(sub_node.span(), source).trim().to_string();
                    }
                    let id = self.extract_identifier(sub_node, source);
                    if !id.is_empty() {
                        return id;
                    }
                }
            }
        }
        String::new()
    }

    fn extract_attributes(&self, node: RedNode<CSharpLanguage>, source: &str) -> Vec<Attribute> {
        let mut attributes = Vec::new();
        for child in node.children() {
            if let RedTree::Node(sub_node) = child {
                if sub_node.green.kind == CSharpElementType::AttributeList {
                    for attr_child in sub_node.children() {
                        if let RedTree::Node(attr_node) = attr_child {
                            if attr_node.green.kind == CSharpElementType::Attribute {
                                let name = self.extract_identifier(attr_node.clone(), source);
                                let mut arguments = Vec::new();
                                // TODO: Extract arguments
                                attributes.push(Attribute { name, arguments })
                            }
                        }
                    }
                }
            }
        }
        attributes
    }

    fn build_namespace(&self, node: RedNode<CSharpLanguage>, source: &str) -> NamespaceDeclaration {
        let name = self.extract_identifier(node.clone(), source);
        let attributes = self.extract_attributes(node.clone(), source);
        let mut items = Vec::new();

        for child in node.children() {
            if let RedTree::Node(sub_node) = child {
                if let Some(item) = self.build_item(sub_node, source) {
                    items.push(item)
                }
            }
        }

        NamespaceDeclaration { name, attributes, items, span: node.span() }
    }

    fn build_using(&self, node: RedNode<CSharpLanguage>, source: &str) -> UsingDirective {
        let path = self.extract_identifier(node.clone(), source);
        let is_static = self.get_text(node.span(), source).contains("static");
        let is_global = self.get_text(node.span(), source).contains("global");
        UsingDirective {
            path,
            is_static,
            alias: None, // TODO
            is_global,
            span: node.span(),
        }
    }

    fn build_class(&self, node: RedNode<CSharpLanguage>, source: &str) -> ClassDeclaration {
        let name = self.extract_identifier(node.clone(), source);
        let mut members = Vec::new();
        let modifiers = self.extract_modifiers(node.clone(), source);
        let attributes = self.extract_attributes(node.clone(), source);

        for child in node.children() {
            if let RedTree::Node(sub_node) = child {
                self.collect_members(sub_node, source, &mut members)
            }
        }

        ClassDeclaration { name, attributes, modifiers, base_types: Vec::new(), type_parameters: Vec::new(), constraints: Vec::new(), members, span: node.span() }
    }

    fn build_interface(&self, node: RedNode<CSharpLanguage>, source: &str) -> InterfaceDeclaration {
        let name = self.extract_identifier(node.clone(), source);
        let modifiers = self.extract_modifiers(node.clone(), source);
        let attributes = self.extract_attributes(node.clone(), source);
        let mut members = Vec::new();
        for child in node.children() {
            if let RedTree::Node(sub_node) = child {
                self.collect_members(sub_node, source, &mut members)
            }
        }
        InterfaceDeclaration { name, attributes, modifiers, members, type_parameters: Vec::new(), span: node.span() }
    }

    fn build_struct(&self, node: RedNode<CSharpLanguage>, source: &str) -> StructDeclaration {
        let name = self.extract_identifier(node.clone(), source);
        let modifiers = self.extract_modifiers(node.clone(), source);
        let attributes = self.extract_attributes(node.clone(), source);
        let mut members = Vec::new();
        for child in node.children() {
            if let RedTree::Node(sub_node) = child {
                self.collect_members(sub_node, source, &mut members)
            }
        }
        StructDeclaration { name, attributes, modifiers, members, type_parameters: Vec::new(), span: node.span() }
    }

    fn build_enum(&self, node: RedNode<CSharpLanguage>, source: &str) -> EnumDeclaration {
        let name = self.extract_identifier(node.clone(), source);
        let modifiers = self.extract_modifiers(node.clone(), source);
        let attributes = self.extract_attributes(node.clone(), source);
        let mut members = Vec::new();
        // 处理枚举成员
        for child in node.children() {
            if let RedTree::Node(sub_node) = child {
                if sub_node.kind::<CSharpElementType>() == CSharpElementType::IdentifierName {
                    members.push(EnumMember { name: self.get_text(sub_node.span(), source).to_string(), attributes: Vec::new(), value: None })
                }
            }
        }
        EnumDeclaration { name, attributes, modifiers, members, span: node.span() }
    }

    fn build_record(&self, node: RedNode<CSharpLanguage>, source: &str) -> RecordDeclaration {
        let name = self.extract_identifier(node.clone(), source);
        let modifiers = self.extract_modifiers(node.clone(), source);
        let attributes = self.extract_attributes(node.clone(), source);
        let mut members = Vec::new();
        for child in node.children() {
            if let RedTree::Node(sub_node) = child {
                self.collect_members(sub_node, source, &mut members)
            }
        }
        RecordDeclaration { name, attributes, modifiers, members, type_parameters: Vec::new(), span: node.span() }
    }

    fn build_delegate(&self, node: RedNode<CSharpLanguage>, source: &str) -> DelegateDeclaration {
        let name = self.extract_identifier(node.clone(), source);
        let modifiers = self.extract_modifiers(node.clone(), source);
        let attributes = self.extract_attributes(node.clone(), source);
        DelegateDeclaration { name, attributes, modifiers, return_type: "void".to_string(), type_parameters: Vec::new(), parameters: Vec::new(), span: node.span() }
    }

    fn extract_modifiers(&self, node: RedNode<CSharpLanguage>, source: &str) -> Vec<String> {
        let mut modifiers = Vec::new();
        for child in node.children() {
            if let RedTree::Leaf(leaf) = child {
                if leaf.kind.is_keyword() {
                    let text = self.get_text(leaf.span, source).trim();
                    match text {
                        "public" | "private" | "protected" | "internal" | "static" | "readonly" | "abstract" | "virtual" | "override" | "async" | "volatile" | "sealed" | "extern" | "partial" | "new" | "unsafe" => modifiers.push(text.to_string()),
                        _ => {}
                    }
                }
            }
        }
        modifiers
    }

    fn collect_members(&self, node: RedNode<CSharpLanguage>, source: &str, members: &mut Vec<Member>) {
        match node.green.kind {
            CSharpElementType::MethodDeclaration => members.push(Member::Method(self.build_method(node, source))),
            CSharpElementType::FieldDeclaration => members.push(Member::Field(self.build_field(node, source))),
            CSharpElementType::PropertyDeclaration => members.push(Member::Property(self.build_property(node, source))),
            CSharpElementType::IndexerDeclaration => members.push(Member::Indexer(self.build_indexer(node, source))),
            CSharpElementType::EventDeclaration => members.push(Member::Event(self.build_event(node, source))),
            _ => {
                for child in node.children() {
                    if let RedTree::Node(sub_node) = child {
                        self.collect_members(sub_node, source, members)
                    }
                }
            }
        }
    }

    fn build_method(&self, node: RedNode<CSharpLanguage>, source: &str) -> MethodDeclaration {
        let name = self.extract_identifier(node.clone(), source);
        let modifiers = self.extract_modifiers(node.clone(), source);
        let attributes = self.extract_attributes(node.clone(), source);
        let is_async = modifiers.contains(&"async".to_string());

        MethodDeclaration {
            name,
            attributes,
            modifiers,
            return_type: "void".to_string(), // TODO
            type_parameters: Vec::new(),     // TODO
            parameters: Vec::new(),          // TODO
            body: self.build_body(node.clone(), source),
            is_async,
            span: node.span(),
        }
    }

    fn build_field(&self, node: RedNode<CSharpLanguage>, source: &str) -> FieldDeclaration {
        let name = self.extract_identifier(node.clone(), source);
        let attributes = self.extract_attributes(node.clone(), source);
        FieldDeclaration {
            name,
            attributes,
            r#type: "object".to_string(), // TODO
            modifiers: self.extract_modifiers(node, source),
            initializer: None, // TODO
            span: Range::default(),
        }
    }

    fn build_property(&self, node: RedNode<CSharpLanguage>, source: &str) -> PropertyDeclaration {
        let name = self.extract_identifier(node.clone(), source);
        let attributes = self.extract_attributes(node.clone(), source);
        PropertyDeclaration {
            name,
            attributes,
            r#type: "object".to_string(), // TODO
            modifiers: self.extract_modifiers(node, source),
            get_accessor: None, // TODO
            set_accessor: None, // TODO
            span: node.span(),
        }
    }

    fn build_indexer(&self, node: RedNode<CSharpLanguage>, source: &str) -> IndexerDeclaration {
        let attributes = self.extract_attributes(node.clone(), source);
        IndexerDeclaration {
            attributes,
            r#type: "object".to_string(), // TODO
            parameters: Vec::new(),       // TODO
            get_accessor: None,           // TODO
            set_accessor: None,           // TODO
            span: node.span(),
        }
    }

    fn build_event(&self, node: RedNode<CSharpLanguage>, source: &str) -> EventDeclaration {
        let name = self.extract_identifier(node.clone(), source);
        let attributes = self.extract_attributes(node.clone(), source);
        EventDeclaration {
            name,
            attributes,
            r#type: "object".to_string(), // TODO
            modifiers: self.extract_modifiers(node, source),
            span: node.span(),
        }
    }

    fn build_body(&self, node: RedNode<CSharpLanguage>, source: &str) -> Option<Vec<Statement>> {
        for child in node.children() {
            if let RedTree::Node(sub_node) = child {
                if sub_node.green.kind == CSharpElementType::Block {
                    let mut statements = Vec::new();
                    for grandchild in sub_node.children() {
                        if let RedTree::Node(grandchild_node) = grandchild {
                            if let Some(stmt) = self.build_statement(grandchild_node, source) {
                                statements.push(stmt)
                            }
                        }
                    }
                    return Some(statements);
                }
            }
        }
        None
    }

    fn build_statement(&self, node: RedNode<CSharpLanguage>, source: &str) -> Option<Statement> {
        match node.green.kind {
            CSharpElementType::ExpressionStatement => {
                for child in node.children() {
                    if let RedTree::Node(sub_node) = child {
                        if let Some(expr) = self.build_expression(sub_node, source) {
                            return Some(Statement::Expression(expr));
                        }
                    }
                }
                None
            }
            CSharpElementType::ReturnStatement => {
                for child in node.children() {
                    if let RedTree::Node(sub_node) = child {
                        if let Some(expr) = self.build_expression(sub_node, source) {
                            return Some(Statement::Return(Some(expr)));
                        }
                    }
                }
                Some(Statement::Return(None))
            }
            CSharpElementType::Block => {
                let mut statements = Vec::new();
                for child in node.children() {
                    if let RedTree::Node(sub_node) = child {
                        if let Some(stmt) = self.build_statement(sub_node, source) {
                            statements.push(stmt)
                        }
                    }
                }
                Some(Statement::Block(statements))
            }
            CSharpElementType::IfStatement => {
                // Simplified
                Some(Statement::If { condition: Expression::Literal(Literal::Boolean(true)), then_branch: Box::new(Statement::Block(Vec::new())), else_branch: None })
            }
            CSharpElementType::BreakStatement => Some(Statement::Break),
            CSharpElementType::ContinueStatement => Some(Statement::Continue),
            _ => None,
        }
    }

    fn build_expression(&self, node: RedNode<CSharpLanguage>, source: &str) -> Option<Expression> {
        match node.green.kind {
            CSharpElementType::LiteralExpression => {
                let text = self.get_text(node.span(), source).trim();
                if text == "true" {
                    Some(Expression::Literal(Literal::Boolean(true)))
                }
                else if text == "false" {
                    Some(Expression::Literal(Literal::Boolean(false)))
                }
                else if text == "null" {
                    Some(Expression::Literal(Literal::Null))
                }
                else if let Ok(n) = text.parse::<i64>() {
                    Some(Expression::Literal(Literal::Integer(n)))
                }
                else {
                    Some(Expression::Literal(Literal::String(text.to_string())))
                }
            }
            CSharpElementType::IdentifierName => Some(Expression::Identifier(self.get_text(node.span(), source).trim().to_string())),
            CSharpElementType::AwaitExpression => {
                for child in node.children() {
                    if let RedTree::Node(sub_node) = child {
                        if let Some(expr) = self.build_expression(sub_node, source) {
                            return Some(Expression::Await(Box::new(expr)));
                        }
                    }
                }
                None
            }
            _ => None,
        }
    }
}

impl<'config> Builder<CSharpLanguage> for CSharpBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<CSharpLanguage>) -> BuildOutput<CSharpLanguage> {
        let mut session = oak_core::parser::ParseSession::<CSharpLanguage>::default();
        let parser = crate::parser::CSharpParser::new(self.language);
        let output = parser.parse(source, edits, &mut session);

        let mut result = Err(oak_core::OakError::custom_error("Build failed"));
        if let Ok(green) = &output.result {
            let root = self.build_root(green, source.get_text_in((0..source.length()).into()).as_ref());
            result = Ok(root)
        }

        oak_core::errors::OakDiagnostics { result, diagnostics: output.diagnostics }
    }
}
