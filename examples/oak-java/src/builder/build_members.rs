use crate::{ast::*, builder::JavaBuilder, language::JavaLanguage, lexer::token_type::JavaTokenType, parser::element_type::JavaElementType};
use oak_core::tree::red_tree::{RedNode, RedTree};

impl<'config> JavaBuilder<'config> {
    pub(crate) fn collect_members(&self, node: RedNode<JavaLanguage>, source: &str, members: &mut Vec<Member>) {
        for child in node.children() {
            if let RedTree::Node(sub_node) = child {
                match sub_node.green.kind {
                    JavaElementType::MethodDeclaration => {
                        let is_constructor = self.is_constructor(sub_node.clone(), source);
                        if is_constructor { members.push(Member::Constructor(self.build_constructor(sub_node, source))) } else { members.push(Member::Method(self.build_method(sub_node, source))) }
                    }
                    JavaElementType::ConstructorDeclaration => members.push(Member::Constructor(self.build_constructor(sub_node, source))),
                    JavaElementType::FieldDeclaration => members.push(Member::Field(self.build_field(sub_node, source))),
                    _ => self.collect_members(sub_node, source, members),
                }
            }
        }
    }

    pub(crate) fn is_constructor(&self, node: RedNode<JavaLanguage>, _source: &str) -> bool {
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

    pub(crate) fn build_constructor(&self, node: RedNode<JavaLanguage>, source: &str) -> ConstructorDeclaration {
        let mut name = String::new();
        let mut parameters = Vec::new();
        let mut body = Vec::new();
        let modifiers = self.extract_modifiers(node.clone(), source);
        let annotations = self.extract_annotations(node.clone(), source);

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

        let constructor = ConstructorDeclaration { modifiers, annotations, name, parameters, body, span: node.span() };

        constructor
    }

    pub(crate) fn build_method(&self, node: RedNode<JavaLanguage>, source: &str) -> MethodDeclaration {
        let mut name = String::new();
        let mut return_type = String::new();
        let mut parameters = Vec::new();
        let mut body = Vec::new();
        let mut throws = Vec::new();
        let mut is_static = false;
        let modifiers = self.extract_modifiers(node.clone(), source);
        let annotations = self.extract_annotations(node.clone(), source);
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
                    _ => {}
                },
                RedTree::Node(sub_node) => match sub_node.green.kind {
                    JavaElementType::Identifier => {
                        let id = self.extract_identifier(sub_node, source);
                        if !modifiers.contains(&id) {
                            if return_type.is_empty() { return_type = id } else { name = id }
                        }
                    }
                    JavaElementType::Parameter => parameters.push(self.build_parameter(sub_node, source)),
                    JavaElementType::BlockStatement => body = self.build_block(sub_node, source),
                    JavaElementType::ThrowsClause => {
                        for throw_child in sub_node.children() {
                            if let RedTree::Node(throw_node) = throw_child {
                                if throw_node.green.kind == JavaElementType::Identifier {
                                    throws.push(self.extract_identifier(throw_node, source))
                                }
                            }
                        }
                    }
                    _ => {}
                },
            }
        }

        let type_parameters = Vec::new(); // TODO: Extract type parameters
        let method = MethodDeclaration { type_parameters, modifiers, annotations, name, return_type, parameters, body, throws, is_static, span: node.span() };

        method
    }

    pub(crate) fn build_parameter(&self, node: RedNode<JavaLanguage>, source: &str) -> Parameter {
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

    pub(crate) fn build_field(&self, node: RedNode<JavaLanguage>, source: &str) -> FieldDeclaration {
        let mut name = String::new();
        let mut field_type = String::new();
        let modifiers = self.extract_modifiers(node.clone(), source);
        let annotations = self.extract_annotations(node.clone(), source);

        for child in node.children() {
            if let RedTree::Node(sub_node) = child {
                if sub_node.green.kind == JavaElementType::Identifier {
                    let id = self.extract_identifier(sub_node, source);
                    if !modifiers.contains(&id) {
                        if field_type.is_empty() { field_type = id } else { name = id }
                    }
                }
            }
        }

        let field = FieldDeclaration { modifiers, annotations, name, r#type: field_type, span: node.span() };
        field
    }
}
