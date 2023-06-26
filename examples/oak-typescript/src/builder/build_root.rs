use crate::{ast::*, builder::TypeScriptBuilder, language::TypeScriptLanguage, parser::element_type::TypeScriptElementType};
use oak_core::{OakError, RedNode, RedTree, SourceText};

impl<'config> TypeScriptBuilder<'config> {
    pub(crate) fn build_root(&self, green_tree: &oak_core::GreenNode<TypeScriptLanguage>, source: &SourceText) -> Result<TypeScriptRoot, OakError> {
        let red_root = RedNode::new(green_tree, 0);
        let span = red_root.span();
        let mut statements = Vec::new();

        self.collect_statements(&red_root, source, &mut statements)?;

        Ok(TypeScriptRoot { statements, span: span.into() })
    }

    pub(crate) fn collect_statements(&self, node: &RedNode<TypeScriptLanguage>, source: &SourceText, statements: &mut Vec<Statement>) -> Result<(), OakError> {
        let kind = node.green.kind;

        if kind == TypeScriptElementType::SourceFile || kind == TypeScriptElementType::Root {
            for child in node.children() {
                if let RedTree::Node(child_node) = child {
                    if let Some(stmt) = self.build_statement(&child_node, source)? {
                        statements.push(stmt);
                    }
                }
            }
        }
        else {
            if let Some(stmt) = self.build_statement(node, source)? {
                // If erase_types is true, we skip type-only statements.
                if self.erase_types {
                    match &stmt {
                        Statement::Interface(_) | Statement::TypeAlias(_) => return Ok(()),
                        _ => {}
                    }
                }
                statements.push(stmt)
            }
        }
        Ok(())
    }
}
