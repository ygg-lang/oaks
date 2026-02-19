#![doc = include_str!("readme.md")]

use crate::{ast::*, language::VbNetLanguage, lexer::token_type::VbNetTokenType, parser::element_type::VbNetElementType};
use oak_core::{
    builder::Builder,
    parser::Parser,
    source::Source,
    tree::{GreenNode, GreenTree},
};

/// VB.NET AST builder
pub struct VbNetBuilder<'config> {
    config: &'config VbNetLanguage,
}

impl<'config> VbNetBuilder<'config> {
    /// Creates a new VB.NET builder
    pub fn new(config: &'config VbNetLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<VbNetLanguage> for VbNetBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[oak_core::TextEdit], cache: &'a mut impl oak_core::builder::BuilderCache<VbNetLanguage>) -> oak_core::builder::BuildOutput<VbNetLanguage> {
        let parser = crate::parser::VbNetParser::new(self.config);
        let parse_output = parser.parse(source, edits, cache);

        match parse_output.result {
            Ok(green_node) => {
                let ast = self.build_ast(&green_node, source);
                oak_core::errors::OakDiagnostics { result: Ok(ast), diagnostics: parse_output.diagnostics }
            }
            Err(err) => oak_core::errors::OakDiagnostics { result: Err(err), diagnostics: parse_output.diagnostics },
        }
    }
}

impl<'config> VbNetBuilder<'config> {
    fn build_ast<'a, S: Source + ?Sized>(&self, node: &GreenNode<'a, VbNetLanguage>, source: &'a S) -> VbNetRoot {
        let mut items = Vec::new();
        self.build_items(node, source, &mut items);
        VbNetRoot { items }
    }

    fn build_items<'a, S: Source + ?Sized>(&self, node: &GreenNode<'a, VbNetLanguage>, source: &'a S, items: &mut Vec<Item>) {
        for child in node.children() {
            match child {
                GreenTree::Node(child_node) => {
                    match child_node.kind() {
                        VbNetElementType::Namespace => {
                            if let Some(namespace) = self.build_namespace(child_node, source) {
                                items.push(Item::Namespace(namespace));
                            }
                        }
                        VbNetElementType::Imports => {
                            if let Some(imports) = self.build_imports(child_node, source) {
                                items.push(Item::Imports(imports));
                            }
                        }
                        VbNetElementType::Class => {
                            if let Some(class) = self.build_class(child_node, source) {
                                items.push(Item::Class(class));
                            }
                        }
                        VbNetElementType::Function => {
                            if let Some(function) = self.build_function(child_node, source) {
                                items.push(Item::Function(function));
                            }
                        }
                        VbNetElementType::Sub => {
                            if let Some(sub) = self.build_sub(child_node, source) {
                                items.push(Item::Sub(sub));
                            }
                        }
                        VbNetElementType::Property => {
                            if let Some(property) = self.build_property(child_node, source) {
                                items.push(Item::Property(property));
                            }
                        }
                        VbNetElementType::Dim => {
                            if let Some(variable) = self.build_variable(child_node, source) {
                                items.push(Item::Variable(variable));
                            }
                        }
                        _ => {
                            // Recursively process child nodes
                            self.build_items(child_node, source, items);
                        }
                    }
                }
                GreenTree::Leaf(_) => {
                    // Skip leaf nodes
                }
            }
        }
    }

    fn build_namespace<'a, S: Source + ?Sized>(&self, node: &GreenNode<'a, VbNetLanguage>, source: &'a S) -> Option<NamespaceDeclaration> {
        let mut name = String::new();
        let mut items = Vec::new();
        let span = (0..node.text_len() as usize).into();
        let mut current_offset = 0;

        for child in node.children() {
            match child {
                GreenTree::Leaf(leaf) => {
                    if leaf.kind() == VbNetTokenType::Identifier {
                        let leaf_start = current_offset;
                        let leaf_end = current_offset + leaf.length() as usize;
                        name = source.get_text_in((leaf_start..leaf_end).into()).to_string();
                    }
                }
                GreenTree::Node(child_node) => {
                    self.build_items(child_node, source, &mut items);
                }
            }
            current_offset += child.len() as usize;
        }

        Some(NamespaceDeclaration { name, items, span })
    }

    fn build_imports<'a, S: Source + ?Sized>(&self, node: &GreenNode<'a, VbNetLanguage>, source: &'a S) -> Option<ImportsDirective> {
        let mut path = String::new();
        let span = (0..node.text_len() as usize).into();
        let mut current_offset = 0;
        let mut found_imports = false;

        for child in node.children() {
            match child {
                GreenTree::Leaf(leaf) => {
                    if leaf.kind() == VbNetTokenType::Imports {
                        found_imports = true;
                    }
                    else if found_imports && leaf.kind() == VbNetTokenType::Identifier {
                        if !path.is_empty() {
                            path.push('.');
                        }
                        let leaf_start = current_offset;
                        let leaf_end = current_offset + leaf.length() as usize;
                        path.push_str(&source.get_text_in((leaf_start..leaf_end).into()));
                    }
                    else if found_imports && leaf.kind() == VbNetTokenType::Dot {
                        path.push('.');
                    }
                }
                GreenTree::Node(_) => {}
            }
            current_offset += child.len() as usize;
        }

        Some(ImportsDirective { path, alias: None, span })
    }

    fn build_class<'a, S: Source + ?Sized>(&self, node: &GreenNode<'a, VbNetLanguage>, source: &'a S) -> Option<ClassDeclaration> {
        let mut name = String::new();
        let mut attributes = Vec::new();
        let mut modifiers = Vec::new();
        let mut base_types = Vec::new();
        let mut members = Vec::new();
        let span = (0..node.text_len() as usize).into();
        let mut current_offset = 0;
        let mut skipped_class = false;

        for child in node.children() {
            match child {
                GreenTree::Leaf(leaf) => match leaf.kind() {
                    VbNetTokenType::Class => {
                        skipped_class = true;
                    }
                    VbNetTokenType::Identifier if skipped_class => {
                        if name.is_empty() {
                            let leaf_start = current_offset;
                            let leaf_end = current_offset + leaf.length() as usize;
                            name = source.get_text_in((leaf_start..leaf_end).into()).to_string();
                        }
                    }
                    VbNetTokenType::Public | VbNetTokenType::Private | VbNetTokenType::Protected | VbNetTokenType::Friend | VbNetTokenType::Shared | VbNetTokenType::MustInherit | VbNetTokenType::NotInheritable | VbNetTokenType::Partial => {
                        let leaf_start = current_offset;
                        let leaf_end = current_offset + leaf.length() as usize;
                        modifiers.push(source.get_text_in((leaf_start..leaf_end).into()).to_string());
                    }
                    _ => {}
                },
                GreenTree::Node(child_node) => {
                    self.build_members(child_node, source, &mut members);
                }
            }
            current_offset += child.len() as usize;
        }

        Some(ClassDeclaration { name, attributes, modifiers, base_types, members, span })
    }

    fn build_function<'a, S: Source + ?Sized>(&self, node: &GreenNode<'a, VbNetLanguage>, source: &'a S) -> Option<FunctionDeclaration> {
        let mut name = String::new();
        let mut attributes = Vec::new();
        let mut modifiers = Vec::new();
        let mut return_type = String::new();
        let mut parameters = Vec::new();
        let mut body = None;
        let span = (0..node.text_len() as usize).into();
        let mut current_offset = 0;
        let mut skipped_function = false;

        for child in node.children() {
            match child {
                GreenTree::Leaf(leaf) => match leaf.kind() {
                    VbNetTokenType::Function => {
                        skipped_function = true;
                    }
                    VbNetTokenType::Identifier if skipped_function => {
                        if name.is_empty() {
                            let leaf_start = current_offset;
                            let leaf_end = current_offset + leaf.length() as usize;
                            name = source.get_text_in((leaf_start..leaf_end).into()).to_string();
                        }
                    }
                    VbNetTokenType::Public
                    | VbNetTokenType::Private
                    | VbNetTokenType::Protected
                    | VbNetTokenType::Friend
                    | VbNetTokenType::Shared
                    | VbNetTokenType::Overrides
                    | VbNetTokenType::Overloads
                    | VbNetTokenType::Overridable
                    | VbNetTokenType::NotOverridable
                    | VbNetTokenType::MustOverride
                    | VbNetTokenType::ReadOnly
                    | VbNetTokenType::WriteOnly
                    | VbNetTokenType::Static
                    | VbNetTokenType::Partial
                    | VbNetTokenType::Async => {
                        let leaf_start = current_offset;
                        let leaf_end = current_offset + leaf.length() as usize;
                        modifiers.push(source.get_text_in((leaf_start..leaf_end).into()).to_string());
                    }
                    _ => {}
                },
                GreenTree::Node(child_node) => {
                    if child_node.kind() == VbNetElementType::Statement {
                        let mut statements = Vec::new();
                        self.build_statements(child_node, source, &mut statements);
                        body = Some(statements);
                    }
                }
            }
            current_offset += child.len() as usize;
        }

        Some(FunctionDeclaration { name, attributes, modifiers, return_type, parameters, body, span })
    }

    fn build_sub<'a, S: Source + ?Sized>(&self, node: &GreenNode<'a, VbNetLanguage>, source: &'a S) -> Option<SubDeclaration> {
        let mut name = String::new();
        let mut attributes = Vec::new();
        let mut modifiers = Vec::new();
        let mut parameters = Vec::new();
        let mut body = None;
        let span = (0..node.text_len() as usize).into();
        let mut current_offset = 0;
        let mut skipped_sub = false;

        for child in node.children() {
            match child {
                GreenTree::Leaf(leaf) => match leaf.kind() {
                    VbNetTokenType::Sub => {
                        skipped_sub = true;
                    }
                    VbNetTokenType::Identifier if skipped_sub => {
                        if name.is_empty() {
                            let leaf_start = current_offset;
                            let leaf_end = current_offset + leaf.length() as usize;
                            name = source.get_text_in((leaf_start..leaf_end).into()).to_string();
                        }
                    }
                    VbNetTokenType::Public
                    | VbNetTokenType::Private
                    | VbNetTokenType::Protected
                    | VbNetTokenType::Friend
                    | VbNetTokenType::Shared
                    | VbNetTokenType::Overrides
                    | VbNetTokenType::Overloads
                    | VbNetTokenType::Overridable
                    | VbNetTokenType::NotOverridable
                    | VbNetTokenType::MustOverride
                    | VbNetTokenType::Static
                    | VbNetTokenType::Partial
                    | VbNetTokenType::Async => {
                        let leaf_start = current_offset;
                        let leaf_end = current_offset + leaf.length() as usize;
                        modifiers.push(source.get_text_in((leaf_start..leaf_end).into()).to_string());
                    }
                    _ => {}
                },
                GreenTree::Node(child_node) => {
                    if child_node.kind() == VbNetElementType::Statement {
                        let mut statements = Vec::new();
                        self.build_statements(child_node, source, &mut statements);
                        body = Some(statements);
                    }
                }
            }
            current_offset += child.len() as usize;
        }

        Some(SubDeclaration { name, attributes, modifiers, parameters, body, span })
    }

    fn build_property<'a, S: Source + ?Sized>(&self, node: &GreenNode<'a, VbNetLanguage>, source: &'a S) -> Option<PropertyDeclaration> {
        let mut name = String::new();
        let mut attributes = Vec::new();
        let mut property_type = String::new();
        let mut modifiers = Vec::new();
        let mut get_accessor = None;
        let mut set_accessor = None;
        let span = (0..node.text_len() as usize).into();
        let mut current_offset = 0;
        let mut skipped_property = false;

        for child in node.children() {
            match child {
                GreenTree::Leaf(leaf) => match leaf.kind() {
                    VbNetTokenType::Property => {
                        skipped_property = true;
                    }
                    VbNetTokenType::Identifier if skipped_property => {
                        if name.is_empty() {
                            let leaf_start = current_offset;
                            let leaf_end = current_offset + leaf.length() as usize;
                            name = source.get_text_in((leaf_start..leaf_end).into()).to_string();
                        }
                    }
                    VbNetTokenType::Public
                    | VbNetTokenType::Private
                    | VbNetTokenType::Protected
                    | VbNetTokenType::Friend
                    | VbNetTokenType::Shared
                    | VbNetTokenType::Overrides
                    | VbNetTokenType::Overloads
                    | VbNetTokenType::Overridable
                    | VbNetTokenType::NotOverridable
                    | VbNetTokenType::MustOverride
                    | VbNetTokenType::ReadOnly
                    | VbNetTokenType::WriteOnly
                    | VbNetTokenType::Static
                    | VbNetTokenType::Partial => {
                        let leaf_start = current_offset;
                        let leaf_end = current_offset + leaf.length() as usize;
                        modifiers.push(source.get_text_in((leaf_start..leaf_end).into()).to_string());
                    }
                    _ => {}
                },
                GreenTree::Node(_) => {
                    // Handle get/set accessors
                }
            }
            current_offset += child.len() as usize;
        }

        Some(PropertyDeclaration { name, attributes, property_type, modifiers, get_accessor, set_accessor, span })
    }

    fn build_variable<'a, S: Source + ?Sized>(&self, node: &GreenNode<'a, VbNetLanguage>, source: &'a S) -> Option<VariableDeclaration> {
        let mut name = String::new();
        let mut attributes = Vec::new();
        let mut variable_type = String::new();
        let mut modifiers = Vec::new();
        let mut initializer = None;
        let span = (0..node.text_len() as usize).into();
        let mut current_offset = 0;
        let mut skipped_dim = false;

        for child in node.children() {
            match child {
                GreenTree::Leaf(leaf) => {
                    if leaf.kind() == VbNetTokenType::Dim {
                        skipped_dim = true;
                    }
                    if leaf.kind() == VbNetTokenType::Identifier && name.is_empty() && skipped_dim {
                        let leaf_start = current_offset;
                        let leaf_end = current_offset + leaf.length() as usize;
                        name = source.get_text_in((leaf_start..leaf_end).into()).to_string();
                    }
                }
                GreenTree::Node(child_node) => {
                    if child_node.kind() == VbNetElementType::Expression {
                        if let Some(expr) = self.build_expression(child_node, source) {
                            initializer = Some(expr);
                        }
                    }
                }
            }
            current_offset += child.len() as usize;
        }

        Some(VariableDeclaration { name, attributes, variable_type, modifiers, initializer, span })
    }

    fn build_members<'a, S: Source + ?Sized>(&self, node: &GreenNode<'a, VbNetLanguage>, source: &'a S, members: &mut Vec<Member>) {
        for child in node.children() {
            match child {
                GreenTree::Node(child_node) => match child_node.kind() {
                    VbNetElementType::Function => {
                        if let Some(function) = self.build_function(child_node, source) {
                            members.push(Member::Function(function));
                        }
                    }
                    VbNetElementType::Sub => {
                        if let Some(sub) = self.build_sub(child_node, source) {
                            members.push(Member::Sub(sub));
                        }
                    }
                    VbNetElementType::Property => {
                        if let Some(property) = self.build_property(child_node, source) {
                            members.push(Member::Property(property));
                        }
                    }
                    VbNetElementType::Dim => {
                        if let Some(variable) = self.build_variable(child_node, source) {
                            members.push(Member::Variable(variable));
                        }
                    }
                    _ => {
                        self.build_members(child_node, source, members);
                    }
                },
                GreenTree::Leaf(_) => {}
            }
        }
    }

    fn build_statements<'a, S: Source + ?Sized>(&self, node: &GreenNode<'a, VbNetLanguage>, source: &'a S, statements: &mut Vec<Statement>) {
        for child in node.children() {
            match child {
                GreenTree::Node(child_node) => {
                    match child_node.kind() {
                        VbNetElementType::Expression => {
                            if let Some(expr) = self.build_expression(child_node, source) {
                                statements.push(Statement::Expression(expr));
                            }
                        }
                        VbNetElementType::Return => {
                            let mut return_expr = None;
                            for grandchild in child_node.children() {
                                if let GreenTree::Node(grandchild_node) = grandchild {
                                    if grandchild_node.kind() == VbNetElementType::Expression {
                                        return_expr = self.build_expression(grandchild_node, source);
                                    }
                                }
                            }
                            statements.push(Statement::Return(return_expr));
                        }
                        VbNetElementType::If => {
                            // TODO: Implement If statement parsing
                        }
                        VbNetElementType::For => {
                            // TODO: Implement For loop parsing
                        }
                        VbNetElementType::While => {
                            // TODO: Implement While loop parsing
                        }
                        VbNetElementType::DoWhile => {
                            // TODO: Implement Do loop parsing
                        }
                        VbNetElementType::SelectCase => {
                            // TODO: Implement Select statement parsing
                        }
                        VbNetElementType::Try => {
                            // TODO: Implement Try statement parsing
                        }
                        VbNetElementType::Exit => {
                            // TODO: Implement Exit statement parsing
                        }
                        VbNetElementType::Continue => {
                            // TODO: Implement Continue statement parsing
                        }
                        VbNetElementType::Throw => {
                            // TODO: Implement Throw statement parsing
                        }
                        VbNetElementType::With => {
                            // TODO: Implement With statement parsing
                        }
                        VbNetElementType::Dim => {
                            if let Some(variable) = self.build_variable(child_node, source) {
                                statements.push(Statement::Dim(variable));
                            }
                        }
                        VbNetElementType::Const => {
                            // TODO: Implement Const statement parsing
                        }
                        _ => {
                            self.build_statements(child_node, source, statements);
                        }
                    }
                }
                GreenTree::Leaf(_) => {}
            }
        }
    }

    fn build_expression<'a, S: Source + ?Sized>(&self, node: &GreenNode<'a, VbNetLanguage>, source: &'a S) -> Option<Expression> {
        let mut current_offset = 0;
        for child in node.children() {
            match child {
                GreenTree::Leaf(leaf) => {
                    match leaf.kind() {
                        VbNetTokenType::Identifier => {
                            let leaf_start = current_offset;
                            let leaf_end = current_offset + leaf.length() as usize;
                            return Some(Expression::Identifier(source.get_text_in((leaf_start..leaf_end).into()).to_string()));
                        }
                        VbNetTokenType::IntegerLiteral => {
                            let leaf_start = current_offset;
                            let leaf_end = current_offset + leaf.length() as usize;
                            if let Ok(value) = source.get_text_in((leaf_start..leaf_end).into()).parse::<i64>() {
                                return Some(Expression::Literal(Literal::Integer(value)));
                            }
                        }
                        VbNetTokenType::FloatLiteral => {
                            let leaf_start = current_offset;
                            let leaf_end = current_offset + leaf.length() as usize;
                            if let Ok(value) = source.get_text_in((leaf_start..leaf_end).into()).parse::<f64>() {
                                return Some(Expression::Literal(Literal::Double(value)));
                            }
                        }
                        VbNetTokenType::StringLiteral => {
                            let leaf_start = current_offset;
                            let leaf_end = current_offset + leaf.length() as usize;
                            let text = source.get_text_in((leaf_start..leaf_end).into());
                            // Remove quotes
                            let value = text.trim_matches('"').replace("\"", "\"");
                            return Some(Expression::Literal(Literal::String(value)));
                        }
                        VbNetTokenType::BooleanLiteral => {
                            let leaf_start = current_offset;
                            let leaf_end = current_offset + leaf.length() as usize;
                            let text = source.get_text_in((leaf_start..leaf_end).into()).to_lowercase();
                            if text == "true" {
                                return Some(Expression::Literal(Literal::Boolean(true)));
                            }
                            else if text == "false" {
                                return Some(Expression::Literal(Literal::Boolean(false)));
                            }
                        }
                        VbNetTokenType::NothingLiteral => {
                            return Some(Expression::Literal(Literal::Nothing));
                        }
                        VbNetTokenType::Me => {
                            return Some(Expression::Me);
                        }
                        VbNetTokenType::MyBase => {
                            return Some(Expression::MyBase);
                        }
                        VbNetTokenType::MyClass => {
                            return Some(Expression::MyClass);
                        }
                        _ => {}
                    }
                }
                GreenTree::Node(child_node) => {
                    if child_node.kind() == VbNetElementType::ParenthesizedExpression {
                        // Parenthesized expression
                        for grandchild in child_node.children() {
                            if let GreenTree::Node(grandchild_node) = grandchild {
                                if let Some(expr) = self.build_expression(grandchild_node, source) {
                                    return Some(Expression::Parenthesized(Box::new(expr)));
                                }
                            }
                        }
                    }
                    else {
                        if let Some(expr) = self.build_expression(child_node, source) {
                            return Some(expr);
                        }
                    }
                }
            }
            current_offset += child.len() as usize;
        }
        None
    }
}
