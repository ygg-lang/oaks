use crate::{builder::JavaBuilder, language::JavaLanguage, lexer::token_type::JavaTokenType, parser::element_type::JavaElementType};
use oak_core::tree::red_tree::{RedNode, RedTree};

impl<'config> JavaBuilder<'config> {
    pub(crate) fn extract_inheritance(&self, node: RedNode<JavaLanguage>, source: &str, class_name: &str, modifiers: &[String]) -> (Option<String>, Vec<String>) {
        let mut extends = None;
        let mut implements = Vec::new();

        let mut found_extends = false;
        let mut found_implements = false;

        for child in node.children() {
            match child {
                RedTree::Leaf(leaf) => {
                    if leaf.kind == JavaTokenType::Extends {
                        found_extends = true;
                        found_implements = false;
                    }
                    else if leaf.kind == JavaTokenType::Implements {
                        found_extends = false;
                        found_implements = true;
                    }
                }
                RedTree::Node(sub_node) => {
                    if sub_node.green.kind == JavaElementType::Identifier {
                        let id = self.get_text(sub_node.span(), source).trim().to_string();
                        if id != class_name && !modifiers.contains(&id) {
                            if found_extends {
                                extends = Some(id);
                                found_extends = false;
                            }
                            else if found_implements {
                                implements.push(id);
                            }
                        }
                    }
                }
            }
        }
        (extends, implements)
    }
}
