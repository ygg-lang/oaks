use crate::{
    ast::*,
    language::JavaLanguage,
    lexer::token_type::JavaTokenType,
    parser::{JavaParser, element_type::JavaElementType},
};
use oak_core::{
    GreenNode, Parser, Source, TokenType,
    builder::{BuildOutput, Builder, BuilderCache},
    source::TextEdit,
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
                    JavaElementType::CompilationUnit => {
                        for sub_child in node.children() {
                            if let RedTree::Node(sub_node) = sub_child {
                                if let Some(item) = self.build_item(sub_node, source) {
                                    items.push(item)
                                }
                            }
                        }
                    }
                    _ => {
                        if let Some(item) = self.build_item(node, source) {
                            items.push(item)
                        }
                    }
                }
            }
        }

        JavaRoot { items }
    }

    fn build_item(&self, node: RedNode<JavaLanguage>, source: &str) -> Option<Item> {
        match node.green.kind {
            JavaElementType::ClassDeclaration => Some(Item::Class(self.build_class(node, source))),
            JavaElementType::InterfaceDeclaration => Some(Item::Interface(self.build_interface(node, source))),
            JavaElementType::EnumDeclaration => Some(Item::Enum(self.build_enum(node, source))),
            JavaElementType::StructDeclaration => Some(Item::Struct(self.build_struct(node, source))),
            JavaElementType::RecordDeclaration => Some(Item::Record(self.build_record(node, source))),
            JavaElementType::Package => Some(Item::Package(PackageDeclaration { name: self.extract_path(node, source), span: node.span() })),
            JavaElementType::Import => {
                let mut is_static = false;
                for child in node.children() {
                    if let RedTree::Leaf(leaf) = child {
                        if leaf.kind == JavaTokenType::Static {
                            is_static = true;
                            break;
                        }
                    }
                }
                Some(Item::Import(ImportDeclaration { path: self.extract_path(node, source), is_static, span: node.span() }))
            }
            _ => None,
        }
    }

    fn extract_path(&self, node: RedNode<JavaLanguage>, source: &str) -> String {
        let mut path = String::new();
        for child in node.children() {
            match child {
                RedTree::Node(sub_node) => {
                    let sub_path = self.extract_path(sub_node, source);
                    if !sub_path.is_empty() {
                        if !path.is_empty() && !sub_path.starts_with('.') {
                            path.push('.')
                        }
                        path.push_str(&sub_path)
                    }
                }
                RedTree::Leaf(leaf) => match leaf.kind {
                    JavaTokenType::Identifier | JavaTokenType::Asterisk => {
                        let text = self.get_text(leaf.span, source).trim();
                        if !path.is_empty() && !path.ends_with('.') {
                            path.push('.')
                        }
                        path.push_str(text)
                    }
                    JavaTokenType::Dot => {
                        if !path.is_empty() && !path.ends_with('.') {
                            path.push('.')
                        }
                    }
                    _ => {}
                },
            }
        }
        path
    }

    fn get_text<'a>(&self, span: core::range::Range<usize>, source: &'a str) -> &'a str {
        let start = span.start.min(source.len());
        let end = span.end.min(source.len());
        if start > end {
            return "";
        }
        &source[start..end]
    }

    fn extract_identifier(&self, node: RedNode<JavaLanguage>, source: &str) -> String {
        match node.green.kind {
            JavaElementType::Identifier => return self.get_text(node.span(), source).trim().to_string(),
            JavaElementType::MemberSelect => {
                let mut path = String::new();
                for child in node.children() {
                    match child {
                        RedTree::Node(sub_node) => path = self.extract_identifier(sub_node, source),
                        RedTree::Leaf(leaf) => {
                            if leaf.kind == JavaTokenType::Identifier {
                                if !path.is_empty() {
                                    path.push('.')
                                }
                                path.push_str(self.get_text(leaf.span, source).trim())
                            }
                        }
                    }
                }
                return path;
            }
            _ => {
                for child in node.children() {
                    match child {
                        RedTree::Leaf(leaf) => {
                            if leaf.kind == JavaTokenType::Identifier {
                                return self.get_text(leaf.span, source).trim().to_string();
                            }
                        }
                        RedTree::Node(sub_node) => {
                            let name = self.extract_identifier(sub_node, source);
                            if !name.is_empty() {
                                return name;
                            }
                        }
                    }
                }
            }
        }
        String::new()
    }

    fn extract_modifiers(&self, node: RedNode<JavaLanguage>, source: &str) -> Vec<String> {
        let mut modifiers = Vec::new();
        for child in node.children() {
            if let RedTree::Leaf(leaf) = child {
                match leaf.kind {
                    JavaTokenType::Public
                    | JavaTokenType::Private
                    | JavaTokenType::Protected
                    | JavaTokenType::Static
                    | JavaTokenType::Final
                    | JavaTokenType::Abstract
                    | JavaTokenType::Native
                    | JavaTokenType::Synchronized
                    | JavaTokenType::Transient
                    | JavaTokenType::Volatile => modifiers.push(self.get_text(leaf.span, source).trim().to_string()),
                    _ => {}
                }
            }
        }
        modifiers
    }

    fn build_class(&self, node: RedNode<JavaLanguage>, source: &str) -> ClassDeclaration {
        let name = self.extract_identifier(node.clone(), source);
        let mut members = Vec::new();
        let modifiers = self.extract_modifiers(node.clone(), source);
        let mut extends = None;
        let mut implements = Vec::new();

        for child in node.children() {
            if let RedTree::Node(sub_node) = child {
                match sub_node.green.kind {
                    JavaElementType::MethodDeclaration => members.push(Member::Method(self.build_method(sub_node, source))),
                    JavaElementType::FieldDeclaration => members.push(Member::Field(self.build_field(sub_node, source))),
                    JavaElementType::Identifier if extends.is_none() && self.get_text(sub_node.span(), source).trim() != name => {
                        // This is a bit weak, but if we have an identifier that's not the class name
                        // and we haven't found extends yet, it might be the superclass.
                        // Actually, the parser puts Extends/Implements in the GreenNode.
                    }
                    _ => self.collect_members(sub_node, source, &mut members),
                }
            }
            else if let RedTree::Leaf(leaf) = child {
                match leaf.kind {
                    JavaTokenType::Extends => {
                        // Next identifier should be the superclass
                    }
                    _ => {}
                }
            }
        }

        // Better way to find extends/implements:
        let mut found_extends = false;
        let mut found_implements = false;
        for child in node.children() {
            match child {
                RedTree::Leaf(leaf) => {
                    if leaf.kind == JavaTokenType::Extends {
                        found_extends = true;
                        found_implements = false
                    }
                    else if leaf.kind == JavaTokenType::Implements {
                        found_extends = false;
                        found_implements = true
                    }
                }
                RedTree::Node(sub_node) => {
                    if sub_node.green.kind == JavaElementType::Identifier {
                        let id = self.get_text(sub_node.span(), source).trim().to_string();
                        if id != name && !modifiers.contains(&id) {
                            if found_extends {
                                extends = Some(id);
                                found_extends = false
                            }
                            else if found_implements {
                                implements.push(id)
                            }
                        }
                    }
                }
            }
        }

        ClassDeclaration { modifiers, name, extends, implements, members, span: node.span() }
    }

    fn build_interface(&self, node: RedNode<JavaLanguage>, source: &str) -> InterfaceDeclaration {
        let name = self.extract_identifier(node.clone(), source);
        let mut members = Vec::new();
        let modifiers = self.extract_modifiers(node.clone(), source);
        let mut extends = Vec::new();

        let mut found_extends = false;
        for child in node.children() {
            match child {
                RedTree::Leaf(leaf) => {
                    if leaf.kind == JavaTokenType::Extends {
                        found_extends = true
                    }
                }
                RedTree::Node(sub_node) => {
                    if sub_node.green.kind == JavaElementType::Identifier {
                        let id = self.get_text(sub_node.span(), source).trim().to_string();
                        if id != name && !modifiers.contains(&id) {
                            if found_extends {
                                extends.push(id)
                            }
                        }
                    }
                    else if sub_node.green.kind == JavaElementType::MethodDeclaration {
                        members.push(Member::Method(self.build_method(sub_node, source)))
                    }
                    else if sub_node.green.kind == JavaElementType::FieldDeclaration {
                        members.push(Member::Field(self.build_field(sub_node, source)))
                    }
                    else {
                        self.collect_members(sub_node, source, &mut members)
                    }
                }
            }
        }

        InterfaceDeclaration { modifiers, name, extends, members, span: node.span() }
    }

    fn build_enum(&self, node: RedNode<JavaLanguage>, source: &str) -> EnumDeclaration {
        let name = self.extract_identifier(node.clone(), source);
        let mut members = Vec::new();
        let modifiers = self.extract_modifiers(node.clone(), source);

        let mut variants = Vec::new();

        for child in node.children() {
            if let RedTree::Node(sub_node) = child {
                match sub_node.green.kind {
                    JavaElementType::Identifier => variants.push(self.get_text(sub_node.span(), source).trim().to_string()),
                    JavaElementType::MethodDeclaration => members.push(Member::Method(self.build_method(sub_node, source))),
                    JavaElementType::FieldDeclaration => members.push(Member::Field(self.build_field(sub_node, source))),
                    _ => self.collect_members(sub_node, source, &mut members),
                }
            }
        }

        EnumDeclaration { modifiers, name, variants, members, span: node.span() }
    }

    fn build_struct(&self, node: RedNode<JavaLanguage>, source: &str) -> StructDeclaration {
        StructDeclaration { name: self.extract_identifier(node.clone(), source), modifiers: self.extract_modifiers(node.clone(), source), implements: Vec::new(), members: Vec::new(), span: node.span() }
    }

    fn build_record(&self, node: RedNode<JavaLanguage>, source: &str) -> RecordDeclaration {
        RecordDeclaration { name: self.extract_identifier(node.clone(), source), modifiers: self.extract_modifiers(node.clone(), source), parameters: Vec::new(), implements: Vec::new(), members: Vec::new(), span: node.span() }
    }

    fn collect_members(&self, node: RedNode<JavaLanguage>, source: &str, members: &mut Vec<Member>) {
        for child in node.children() {
            if let RedTree::Node(sub_node) = child {
                match sub_node.green.kind {
                    JavaElementType::MethodDeclaration => {
                        // Check if it's a constructor (name matches class name and no return type)
                        // Actually, let's look at the CST structure.
                        let method = self.build_method(sub_node.clone(), source);
                        // If it looks like a constructor, we should have a special check.
                        // But wait, our build_method might already be confused.
                        // In Java, constructors don't have return types.

                        // Let's check if the CST node is actually a constructor if we had that kind.
                        // If not, we check if return_type is empty or matches name?
                        // Actually, let's just use the MethodDeclaration for now but convert to Constructor if needed.

                        // Wait, I added ConstructorDeclaration to Member.
                        // Let's see if we can distinguish them.
                        let is_constructor = self.is_constructor(sub_node.clone(), source);
                        if is_constructor { members.push(Member::Constructor(self.build_constructor(sub_node, source))) } else { members.push(Member::Method(method)) }
                    }
                    JavaElementType::FieldDeclaration => members.push(Member::Field(self.build_field(sub_node, source))),
                    _ => self.collect_members(sub_node, source, members),
                }
            }
        }
    }

    fn is_constructor(&self, node: RedNode<JavaLanguage>, _source: &str) -> bool {
        // A constructor has no return type in the CST.
        // In our build_method, if return_type is "void" (default) but there was no void keyword,
        // it might be a constructor.

        // Let's check if there is any type before the name.
        let mut has_type = false;
        let mut name_found = false;
        for child in node.children() {
            match child {
                RedTree::Leaf(leaf) => match leaf.kind {
                    JavaTokenType::Identifier | JavaTokenType::Int | JavaTokenType::Boolean | JavaTokenType::Void | JavaTokenType::Long | JavaTokenType::Float | JavaTokenType::Double | JavaTokenType::Char | JavaTokenType::Byte | JavaTokenType::Short => {
                        if !name_found {
                            if has_type { name_found = true } else { has_type = true }
                        }
                    }
                    _ => {}
                },
                RedTree::Node(sub_node) => {
                    if sub_node.green.kind == JavaElementType::Identifier {
                        if !name_found {
                            if has_type { name_found = true } else { has_type = true }
                        }
                    }
                }
            }
        }
        !name_found && has_type
    }

    fn build_constructor(&self, node: RedNode<JavaLanguage>, source: &str) -> ConstructorDeclaration {
        let mut name = String::new();
        let mut parameters = Vec::new();
        let mut body = Vec::new();
        let modifiers = self.extract_modifiers(node.clone(), source);

        for child in node.children() {
            match child {
                RedTree::Leaf(leaf) => match leaf.kind {
                    JavaTokenType::Identifier => {
                        if name.is_empty() && !modifiers.contains(&self.get_text(leaf.span, source).trim().to_string()) {
                            name = self.get_text(leaf.span, source).trim().to_string()
                        }
                    }
                    _ => {}
                },
                RedTree::Node(sub_node) => match sub_node.green.kind {
                    JavaElementType::Identifier => {
                        if name.is_empty() {
                            name = self.extract_identifier(sub_node, source)
                        }
                    }
                    JavaElementType::Parameter => parameters.push(self.build_parameter(sub_node, source)),
                    JavaElementType::BlockStatement => body = self.build_block(sub_node, source),
                    _ => {}
                },
            }
        }

        ConstructorDeclaration { modifiers, name, parameters, body, span: node.span() }
    }

    fn build_method(&self, node: RedNode<JavaLanguage>, source: &str) -> MethodDeclaration {
        let mut name = String::new();
        let mut return_type = String::new();
        let mut parameters = Vec::new();
        let mut body = Vec::new();
        let mut throws = Vec::new();
        let mut is_static = false;
        let modifiers = self.extract_modifiers(node.clone(), source);
        if modifiers.contains(&"static".to_string()) {
            is_static = true
        }

        for child in node.children() {
            match child {
                RedTree::Leaf(leaf) => match leaf.kind {
                    JavaTokenType::Identifier => {
                        if !modifiers.contains(&self.get_text(leaf.span, source).trim().to_string()) {
                            if return_type.is_empty() { return_type = self.get_text(leaf.span, source).trim().to_string() } else { name = self.get_text(leaf.span, source).trim().to_string() }
                        }
                    }
                    JavaTokenType::Static => is_static = true,
                    JavaTokenType::Void | JavaTokenType::Int | JavaTokenType::Boolean => return_type = self.get_text(leaf.span, source).trim().to_string(),
                    _ => {}
                },
                RedTree::Node(sub_node) => match sub_node.green.kind {
                    JavaElementType::Identifier => {
                        if return_type.is_empty() {
                            return_type = self.extract_identifier(sub_node, source)
                        }
                        else if name.is_empty() {
                            name = self.extract_identifier(sub_node, source)
                        }
                        else {
                            throws.push(self.extract_identifier(sub_node, source))
                        }
                    }
                    JavaElementType::Parameter => parameters.push(self.build_parameter(sub_node, source)),
                    JavaElementType::BlockStatement => body = self.build_block(sub_node, source),
                    _ => {}
                },
            }
        }

        if return_type.is_empty() {
            return_type = "void".to_string()
        }

        MethodDeclaration { modifiers, name, return_type, parameters, body, throws, is_static, span: node.span() }
    }

    fn build_parameter(&self, node: RedNode<JavaLanguage>, source: &str) -> Parameter {
        let mut name = String::new();
        let mut r#type = String::new();
        let mut is_array = false;

        for child in node.children() {
            match child {
                RedTree::Leaf(leaf) => match leaf.kind {
                    JavaTokenType::Identifier => {
                        if r#type.is_empty() {
                            r#type = self.get_text(leaf.span, source).to_string()
                        }
                        else {
                            name = self.get_text(leaf.span, source).to_string()
                        }
                    }
                    JavaTokenType::Int | JavaTokenType::Boolean | JavaTokenType::Void | JavaTokenType::Long | JavaTokenType::Float | JavaTokenType::Double | JavaTokenType::Char | JavaTokenType::Byte | JavaTokenType::Short => {
                        r#type = self.get_text(leaf.span, source).to_string()
                    }
                    JavaTokenType::LeftBracket => is_array = true,
                    _ => {}
                },
                RedTree::Node(sub_node) => {
                    if sub_node.green.kind == JavaElementType::Identifier {
                        if r#type.is_empty() { r#type = self.extract_identifier(sub_node, source) } else { name = self.extract_identifier(sub_node, source) }
                    }
                }
            }
        }

        if is_array {
            r#type.push_str("[]")
        }

        Parameter { name, r#type }
    }

    fn build_field(&self, node: RedNode<JavaLanguage>, source: &str) -> FieldDeclaration {
        let mut name = String::new();
        let mut r#type = String::new();
        let modifiers = self.extract_modifiers(node.clone(), source);

        for child in node.children() {
            match child {
                RedTree::Leaf(leaf) => match leaf.kind {
                    JavaTokenType::Identifier => {
                        if !modifiers.contains(&self.get_text(leaf.span, source).trim().to_string()) {
                            if r#type.is_empty() { r#type = self.get_text(leaf.span, source).to_string() } else { name = self.get_text(leaf.span, source).to_string() }
                        }
                    }
                    JavaTokenType::Int | JavaTokenType::Boolean | JavaTokenType::Void | JavaTokenType::Long | JavaTokenType::Float | JavaTokenType::Double | JavaTokenType::Char | JavaTokenType::Byte | JavaTokenType::Short => {
                        r#type = self.get_text(leaf.span, source).to_string()
                    }
                    _ => {}
                },
                RedTree::Node(sub_node) => {
                    if sub_node.green.kind == JavaElementType::Identifier {
                        if r#type.is_empty() { r#type = self.extract_identifier(sub_node, source) } else { name = self.extract_identifier(sub_node, source) }
                    }
                }
            }
        }

        FieldDeclaration { modifiers, name, r#type, span: node.span() }
    }

    fn build_block(&self, node: RedNode<JavaLanguage>, source: &str) -> Vec<Statement> {
        let mut statements = Vec::new();
        for child in node.children() {
            if let RedTree::Node(sub_node) = child {
                statements.push(self.build_statement(sub_node, source))
            }
        }
        statements
    }

    fn build_catch_clause(&self, node: RedNode<JavaLanguage>, source: &str) -> CatchClause {
        let mut parameter = Parameter { name: "".to_string(), r#type: "".to_string() };
        let mut block = Vec::new();
        for child in node.children() {
            if let RedTree::Node(sub_node) = child {
                match sub_node.green.kind {
                    JavaElementType::Parameter => parameter = self.build_parameter(sub_node, source),
                    JavaElementType::BlockStatement => block = self.build_block(sub_node, source),
                    _ => {}
                }
            }
        }
        CatchClause { parameter, block }
    }

    fn build_switch_case(&self, node: RedNode<JavaLanguage>, source: &str) -> SwitchCase {
        let mut label = None;
        let mut body = Vec::new();
        for child in node.children() {
            if let RedTree::Node(sub_node) = child {
                if label.is_none() && sub_node.green.kind != JavaElementType::BlockStatement {
                    label = Some(self.build_expression(sub_node, source))
                }
                else if sub_node.green.kind == JavaElementType::BlockStatement {
                    body.extend(self.build_block(sub_node, source))
                }
                else {
                    body.push(self.build_statement(sub_node, source))
                }
            }
        }
        SwitchCase { label: label.unwrap_or(Expression::Literal(Literal::Integer(0))), body }
    }

    fn build_statement(&self, node: RedNode<JavaLanguage>, source: &str) -> Statement {
        match node.green.kind {
            JavaElementType::ExpressionStatement => {
                let mut expr_node = None;
                for child in node.children() {
                    if let RedTree::Node(sub_node) = child {
                        expr_node = Some(sub_node);
                        break;
                    }
                }
                if let Some(sub_node) = expr_node {
                    let expr = self.build_expression(sub_node.clone(), source);
                    return Statement::Expression(expr);
                }
                Statement::Block(vec![])
            }
            JavaElementType::BlockStatement => Statement::Block(self.build_block(node, source)),
            JavaElementType::ReturnStatement => {
                let mut expr = None;
                for child in node.children() {
                    if let RedTree::Node(sub_node) = child {
                        expr = Some(self.build_expression(sub_node, source));
                        break;
                    }
                }
                Statement::Return(expr)
            }
            JavaElementType::IfStatement => {
                let mut condition = None;
                let mut then_branch = None;
                let mut else_branch = None;
                for child in node.children() {
                    if let RedTree::Node(sub_node) = child {
                        if condition.is_none() {
                            condition = Some(self.build_expression(sub_node, source))
                        }
                        else if then_branch.is_none() {
                            then_branch = Some(Box::new(self.build_statement(sub_node, source)))
                        }
                        else {
                            else_branch = Some(Box::new(self.build_statement(sub_node, source)))
                        }
                    }
                }
                Statement::If { condition: condition.unwrap_or(Expression::Literal(Literal::Boolean(true))), then_branch: then_branch.unwrap_or(Box::new(Statement::Block(vec![]))), else_branch }
            }
            JavaElementType::WhileStatement => {
                let mut condition = None;
                let mut body = None;
                for child in node.children() {
                    if let RedTree::Node(sub_node) = child {
                        if condition.is_none() { condition = Some(self.build_expression(sub_node, source)) } else { body = Some(Box::new(self.build_statement(sub_node, source))) }
                    }
                }
                Statement::While { condition: condition.unwrap_or(Expression::Literal(Literal::Boolean(true))), body: body.unwrap_or(Box::new(Statement::Block(vec![]))) }
            }
            JavaElementType::DoWhileStatement => {
                let mut condition = None;
                let mut body = None;
                for child in node.children() {
                    if let RedTree::Node(sub_node) = child {
                        if body.is_none() { body = Some(Box::new(self.build_statement(sub_node, source))) } else { condition = Some(self.build_expression(sub_node, source)) }
                    }
                }
                Statement::DoWhile { condition: condition.unwrap_or(Expression::Literal(Literal::Boolean(true))), body: body.unwrap_or(Box::new(Statement::Block(vec![]))) }
            }
            JavaElementType::ForStatement => {
                let mut init = None;
                let mut condition = None;
                let mut update = None;
                let mut body = None;
                let mut semicolon_count = 0;
                let mut after_right_paren = false;

                for child in node.children() {
                    match child {
                        RedTree::Leaf(leaf) => {
                            if leaf.kind == JavaTokenType::Semicolon {
                                semicolon_count += 1
                            }
                            else if leaf.kind == JavaTokenType::RightParen {
                                after_right_paren = true
                            }
                        }
                        RedTree::Node(sub_node) => {
                            if after_right_paren {
                                if body.is_none() {
                                    body = Some(Box::new(self.build_statement(sub_node, source)))
                                }
                            }
                            else {
                                match semicolon_count {
                                    0 => init = Some(Box::new(self.build_statement(sub_node, source))),
                                    1 => condition = Some(self.build_expression(sub_node, source)),
                                    2 => update = Some(self.build_expression(sub_node, source)),
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                Statement::For { init, condition, update, body: body.unwrap_or(Box::new(Statement::Block(vec![]))) }
            }
            JavaElementType::SwitchStatement => {
                let mut selector = None;
                let mut cases = Vec::new();
                let mut default = None;
                for child in node.children() {
                    if let RedTree::Node(sub_node) = child {
                        match sub_node.green.kind {
                            JavaElementType::SwitchCase => cases.push(self.build_switch_case(sub_node, source)),
                            JavaElementType::DefaultCase => default = Some(self.build_block(sub_node, source)),
                            _ => {
                                if selector.is_none() {
                                    selector = Some(self.build_expression(sub_node, source))
                                }
                            }
                        }
                    }
                }
                Statement::Switch { selector: selector.unwrap_or(Expression::Identifier("unknown_selector".to_string())), cases, default }
            }
            JavaElementType::Break => Statement::Break,
            JavaElementType::Continue => Statement::Continue,
            JavaElementType::TryStatement => {
                let mut block = Vec::new();
                let mut catches = Vec::new();
                let mut finally = None;
                for child in node.children() {
                    if let RedTree::Node(sub_node) = child {
                        match sub_node.green.kind {
                            JavaElementType::BlockStatement => {
                                if block.is_empty() {
                                    block = self.build_block(sub_node, source);
                                }
                                else {
                                    finally = Some(self.build_block(sub_node, source));
                                }
                            }
                            JavaElementType::CatchClause => catches.push(self.build_catch_clause(sub_node, source)),
                            _ => {}
                        }
                    }
                }
                Statement::Try(TryStatement { block, catches, finally })
            }
            JavaElementType::ThrowStatement => {
                for child in node.children() {
                    if let RedTree::Node(sub_node) = child {
                        return Statement::Throw(self.build_expression(sub_node, source));
                    }
                }
                Statement::Throw(Expression::Identifier("unknown_throw".to_string()))
            }
            JavaElementType::VariableDeclaration => {
                let mut r#type = String::new();
                let mut name = String::new();
                let mut initializer = None;

                for child in node.children() {
                    match child {
                        RedTree::Leaf(leaf) => match leaf.kind {
                            JavaTokenType::Identifier => {
                                if r#type.is_empty() {
                                    r#type = self.get_text(leaf.span, source).to_string()
                                }
                                else if name.is_empty() {
                                    name = self.get_text(leaf.span, source).to_string()
                                }
                            }
                            JavaTokenType::Int | JavaTokenType::Boolean | JavaTokenType::Void | JavaTokenType::Long | JavaTokenType::Float | JavaTokenType::Double | JavaTokenType::Char | JavaTokenType::Byte | JavaTokenType::Short => {
                                if r#type.is_empty() {
                                    r#type = self.get_text(leaf.span, source).to_string()
                                }
                            }
                            JavaTokenType::LeftBracket => {
                                if !r#type.is_empty() {
                                    r#type.push_str("[]")
                                }
                            }
                            _ => {}
                        },
                        RedTree::Node(sub_node) => {
                            if sub_node.green.kind == JavaElementType::Identifier {
                                if r#type.is_empty() {
                                    r#type = self.extract_identifier(sub_node, source)
                                }
                                else if name.is_empty() {
                                    name = self.extract_identifier(sub_node, source)
                                }
                                else {
                                    initializer = Some(self.build_expression(sub_node, source))
                                }
                            }
                            else {
                                initializer = Some(self.build_expression(sub_node, source))
                            }
                        }
                    }
                }
                Statement::LocalVariable { r#type, name, initializer }
            }
            _ => Statement::Block(vec![]),
        }
    }

    fn build_expression(&self, node: RedNode<JavaLanguage>, source: &str) -> Expression {
        match node.green.kind {
            JavaElementType::AssignmentExpression => {
                let mut left = None;
                let mut right = None;
                let mut op = String::new();
                for child in node.children() {
                    match child {
                        RedTree::Node(sub_node) => {
                            if left.is_none() {
                                left = Some(Box::new(self.build_expression(sub_node, source)))
                            }
                            else {
                                right = Some(Box::new(self.build_expression(sub_node, source)))
                            }
                        }
                        RedTree::Leaf(leaf) => {
                            if leaf.kind.role() == oak_core::UniversalTokenRole::Operator || leaf.kind == JavaTokenType::Instanceof {
                                op = self.get_text(leaf.span, source).trim().to_string()
                            }
                        }
                    }
                }
                if let (Some(left), Some(right)) = (left, right) { Expression::Assignment { left, op, right } } else { Expression::Identifier("unknown_assignment".to_string()) }
            }
            JavaElementType::UnaryExpression => {
                let mut expr = None;
                let mut op = String::new();
                for child in node.children() {
                    match child {
                        RedTree::Node(sub_node) => expr = Some(Box::new(self.build_expression(sub_node, source))),
                        RedTree::Leaf(leaf) => {
                            if leaf.kind.role() == oak_core::UniversalTokenRole::Operator || leaf.kind == JavaTokenType::Instanceof {
                                op = self.get_text(leaf.span, source).trim().to_string();
                            }
                        }
                    }
                }
                if let Some(expression) = expr { if op == "++" || op == "--" { Expression::Update { expression, op, is_prefix: true } } else { Expression::Unary { op, expression } } } else { Expression::Identifier("unknown_unary".to_string()) }
            }
            JavaElementType::PostfixExpression => {
                let mut expr = None;
                let mut op = String::new();
                for child in node.children() {
                    match child {
                        RedTree::Node(sub_node) => expr = Some(Box::new(self.build_expression(sub_node, source))),
                        RedTree::Leaf(leaf) => {
                            if leaf.kind.role() == oak_core::UniversalTokenRole::Operator || leaf.kind == JavaTokenType::Instanceof {
                                op = self.get_text(leaf.span, source).trim().to_string();
                            }
                        }
                    }
                }
                if let Some(expression) = expr { Expression::Update { expression, op, is_prefix: false } } else { Expression::Identifier("unknown_postfix".to_string()) }
            }
            JavaElementType::TernaryExpression => {
                let mut condition = None;
                let mut then_branch = None;
                let mut else_branch = None;
                for child in node.children() {
                    if let RedTree::Node(sub_node) = child {
                        if condition.is_none() {
                            condition = Some(Box::new(self.build_expression(sub_node, source)));
                        }
                        else if then_branch.is_none() {
                            then_branch = Some(Box::new(self.build_expression(sub_node, source)));
                        }
                        else {
                            else_branch = Some(Box::new(self.build_expression(sub_node, source)));
                        }
                    }
                }
                if let (Some(condition), Some(then_branch), Some(else_branch)) = (condition, then_branch, else_branch) { Expression::Ternary { condition, then_branch, else_branch } } else { Expression::Identifier("unknown_ternary".to_string()) }
            }
            JavaElementType::CastExpression => {
                let mut target_type = String::new();
                let mut expr = None;
                for child in node.children() {
                    match child {
                        RedTree::Node(sub_node) => {
                            if target_type.is_empty() {
                                target_type = self.extract_identifier(sub_node, source);
                            }
                            else {
                                expr = Some(Box::new(self.build_expression(sub_node, source)));
                            }
                        }
                        RedTree::Leaf(leaf) => {
                            if target_type.is_empty() && (leaf.kind == JavaTokenType::Identifier || leaf.kind.role() == oak_core::UniversalTokenRole::Keyword) {
                                target_type = self.get_text(leaf.span, source).trim().to_string();
                            }
                        }
                    }
                }
                if let Some(expression) = expr { Expression::Cast { target_type, expression } } else { Expression::Identifier("unknown_cast".to_string()) }
            }
            JavaElementType::BinaryExpression => {
                let mut left = None;
                let mut right = None;
                let mut op = String::new();
                for child in node.children() {
                    match child {
                        RedTree::Node(sub_node) => {
                            if left.is_none() {
                                left = Some(Box::new(self.build_expression(sub_node, source)))
                            }
                            else {
                                right = Some(Box::new(self.build_expression(sub_node, source)))
                            }
                        }
                        RedTree::Leaf(leaf) => {
                            if leaf.kind.role() == oak_core::UniversalTokenRole::Operator || leaf.kind == JavaTokenType::Instanceof {
                                op = self.get_text(leaf.span, source).trim().to_string();
                            }
                        }
                    }
                }
                if let (Some(left), Some(right)) = (left, right) { Expression::Binary { left, op, right } } else { Expression::Identifier("unknown_binary".to_string()) }
            }
            JavaElementType::LiteralExpression => {
                for child in node.children() {
                    match child {
                        RedTree::Leaf(leaf) => match leaf.kind {
                            JavaTokenType::IntegerLiteral => {
                                let text = self.get_text(leaf.span, source).trim().to_string();
                                println!("DEBUG: Parsing IntegerLiteral: '{}'", text);
                                let val = text.parse().unwrap_or(0);
                                return Expression::Literal(Literal::Integer(val));
                            }
                            JavaTokenType::FloatingPointLiteral => {
                                let text = self.get_text(leaf.span, source).trim().to_string();
                                let val = text.parse().unwrap_or(0.0);
                                return Expression::Literal(Literal::Float(val));
                            }
                            JavaTokenType::StringLiteral => {
                                let text = self.get_text(leaf.span, source).trim();
                                let content = if text.len() >= 2 { &text[1..text.len() - 1] } else { text };
                                return Expression::Literal(Literal::String(content.to_string()));
                            }
                            JavaTokenType::BooleanLiteral => {
                                let val = self.get_text(leaf.span, source).trim() == "true";
                                return Expression::Literal(Literal::Boolean(val));
                            }
                            _ => {}
                        },
                        RedTree::Node(sub_node) => {
                            let expr = self.build_expression(sub_node, source);
                            if let Expression::Literal(_) = expr {
                                return expr;
                            }
                        }
                    }
                }
                // Fallback: try to parse the whole node text as a literal if it's a leaf-like node
                let text = self.get_text(node.span(), source).trim().to_string();
                if let Ok(val) = text.parse::<i64>() {
                    return Expression::Literal(Literal::Integer(val));
                }
                if let Ok(val) = text.parse::<f64>() {
                    return Expression::Literal(Literal::Float(val));
                }
                Expression::Identifier("unknown".to_string())
            }
            JavaElementType::MethodCall => {
                let mut callee = None;
                let mut arguments = Vec::new();

                for child in node.children() {
                    match child {
                        RedTree::Node(sub_node) => {
                            if callee.is_none() {
                                callee = Some(self.build_expression(sub_node, source));
                            }
                            else {
                                arguments.push(self.build_expression(sub_node, source));
                            }
                        }
                        _ => {}
                    }
                }

                if let Some(expr) = callee {
                    match expr {
                        Expression::FieldAccess(fa) => Expression::MethodCall(MethodCall { target: Some(fa.target), name: fa.name, arguments }),
                        Expression::Identifier(name) => Expression::MethodCall(MethodCall { target: None, name, arguments }),
                        _ => Expression::MethodCall(MethodCall { target: Some(Box::new(expr)), name: "unknown".to_string(), arguments }),
                    }
                }
                else {
                    Expression::Identifier("unknown".to_string())
                }
            }
            JavaElementType::MemberSelect => {
                let mut target = None;
                let mut name = String::new();
                for child in node.children() {
                    match child {
                        RedTree::Node(sub_node) => target = Some(Box::new(self.build_expression(sub_node, source))),
                        RedTree::Leaf(leaf) => {
                            if leaf.kind == JavaTokenType::Identifier {
                                name = self.get_text(leaf.span, source).trim().to_string();
                            }
                        }
                    }
                }
                if let Some(target) = target { Expression::FieldAccess(FieldAccess { target, name }) } else { Expression::Identifier(name) }
            }
            JavaElementType::ArrayAccess => {
                let mut target = None;
                let mut index = None;
                for child in node.children() {
                    if let RedTree::Node(sub_node) = child {
                        if target.is_none() {
                            target = Some(Box::new(self.build_expression(sub_node, source)));
                        }
                        else {
                            index = Some(Box::new(self.build_expression(sub_node, source)));
                        }
                    }
                }
                if let (Some(target), Some(index)) = (target, index) { Expression::ArrayAccess(ArrayAccess { target, index }) } else { Expression::Identifier("unknown_array_access".to_string()) }
            }
            JavaElementType::ArrayCreation => {
                let mut element_type = String::new();
                let mut dimensions = Vec::new();
                for child in node.children() {
                    match child {
                        RedTree::Leaf(leaf) => {
                            if leaf.kind == JavaTokenType::Identifier || leaf.kind.role() == oak_core::UniversalTokenRole::Keyword {
                                if element_type.is_empty() {
                                    element_type = self.get_text(leaf.span, source).to_string()
                                }
                            }
                        }
                        RedTree::Node(sub_node) => dimensions.push(self.build_expression(sub_node, source)),
                    }
                }
                Expression::ArrayCreation(ArrayCreation { element_type, dimensions })
            }
            JavaElementType::Identifier => {
                let name = self.get_text(node.span(), source).trim().to_string();
                match name.as_str() {
                    "this" => Expression::This,
                    "super" => Expression::Super,
                    _ => Expression::Identifier(name),
                }
            }
            JavaElementType::NewExpression => {
                let mut r#type = String::new();
                let mut arguments = Vec::new();
                for child in node.children() {
                    match child {
                        RedTree::Leaf(leaf) => {
                            if leaf.kind == JavaTokenType::Identifier && r#type.is_empty() {
                                r#type = self.get_text(leaf.span, source).to_string()
                            }
                        }
                        RedTree::Node(sub_node) => {
                            if r#type.is_empty() {
                                r#type = self.extract_identifier(sub_node, source)
                            }
                            else {
                                arguments.push(self.build_expression(sub_node, source));
                            }
                        }
                    }
                }
                Expression::New(NewExpression { r#type, arguments })
            }
            JavaElementType::ParenthesizedExpression => {
                for child in node.children() {
                    if let RedTree::Node(sub_node) = child {
                        return self.build_expression(sub_node, source);
                    }
                }
                Expression::Identifier("unknown_parenthesized".to_string())
            }
            _ => {
                for child in node.children() {
                    if let RedTree::Leaf(leaf) = child {
                        if leaf.kind == JavaTokenType::Identifier {
                            let name = self.get_text(leaf.span, source).trim().to_string();
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
