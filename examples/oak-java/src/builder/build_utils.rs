use crate::{builder::JavaBuilder, language::JavaLanguage, lexer::token_type::JavaTokenType, parser::element_type::JavaElementType};
use oak_core::tree::red_tree::{RedNode, RedTree};

impl<'config> JavaBuilder<'config> {
    pub(crate) fn extract_path(&self, node: RedNode<JavaLanguage>, source: &str) -> String {
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

    pub(crate) fn get_text<'a>(&self, span: core::range::Range<usize>, source: &'a str) -> &'a str {
        let start = span.start.min(source.len());
        let end = span.end.min(source.len());
        if start > end {
            return "";
        }
        &source[start..end]
    }

    pub(crate) fn extract_identifier(&self, node: RedNode<JavaLanguage>, source: &str) -> String {
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

    pub(crate) fn extract_modifiers(&self, node: RedNode<JavaLanguage>, source: &str) -> Vec<String> {
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

    pub(crate) fn extract_annotations(&self, node: RedNode<JavaLanguage>, source: &str) -> Vec<crate::ast::Annotation> {
        let mut annotations = Vec::new();
        for child in node.children() {
            if let RedTree::Node(sub_node) = child {
                if sub_node.green.kind == JavaElementType::Annotation {
                    let mut name = String::new();
                    let mut arguments = Vec::new();
                    for attr_child in sub_node.children() {
                        match attr_child {
                            RedTree::Node(n) if n.green.kind == JavaElementType::Identifier => {
                                name = self.extract_identifier(n, source);
                            }
                            RedTree::Node(n) if n.green.kind != JavaElementType::Identifier && n.green.kind != JavaElementType::Annotation => {
                                // Extract expression if possible
                                arguments.push(self.build_expression(n, source));
                            }
                            _ => {}
                        }
                    }
                    annotations.push(crate::ast::Annotation { name, arguments, span: sub_node.span() });
                }
            }
        }
        annotations
    }
}
