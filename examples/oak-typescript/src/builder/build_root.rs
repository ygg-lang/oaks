use crate::{ast::*, builder::TypeScriptBuilder, language::TypeScriptLanguage, parser::element_type::TypeScriptElementType};
use oak_core::{OakError, RedNode, RedTree, SourceText};

impl<'config> TypeScriptBuilder<'config> {
    pub(crate) fn build_root(&self, green_tree: &oak_core::GreenNode<TypeScriptLanguage>, source: &SourceText) -> Result<TypeScriptRoot, OakError> {
        let red_root = RedNode::new(green_tree, 0);
        let span = red_root.span();
        let mut statements = Vec::new();

        println!("Green tree root: {:?}", green_tree.kind);
        self.print_tree(&red_root, 0);

        self.collect_statements(&red_root, source, &mut statements)?;

        Ok(TypeScriptRoot { statements, span: span.into() })
    }

    pub(crate) fn print_tree(&self, node: &RedNode<TypeScriptLanguage>, indent: usize) {
        let span = node.span();
        println!("{:indent$}{:?} {:?}", "", node.green.kind, span, indent = indent);
        for child in node.children() {
            match child {
                RedTree::Node(child_node) => self.print_tree(&child_node, indent + 2),
                RedTree::Leaf(leaf) => println!("{:indent$}{:?} {:?}", "", leaf.kind, leaf.span, indent = indent + 2),
            }
        }
    }

    pub(crate) fn collect_statements(&self, node: &RedNode<TypeScriptLanguage>, source: &SourceText, statements: &mut Vec<Statement>) -> Result<(), OakError> {
        let kind = node.green.kind;

        if kind == TypeScriptElementType::SourceFile || kind == TypeScriptElementType::Root {
            for child in node.children() {
                if let RedTree::Node(child_node) = child {
                    self.collect_statements(&child_node, source, statements)?
                }
            }
        }
        else {
            if let Some(stmt) = self.build_statement(node, source)? {
                statements.push(stmt)
            }
        }
        Ok(())
    }
}
