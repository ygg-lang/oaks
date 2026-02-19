use crate::{
    DejavuLanguage,
    ast::{BlockNode, ExpressionStatement, StatementNode},
    builder::DejavuBuilder,
    parser::element_type::DejavuElementType,
};
use oak_core::{OakError, RedNode, RedTree, Source};

impl<'config> DejavuBuilder<'config> {
    pub(crate) fn build_block<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<BlockNode, OakError> {
        let span = node.span();
        let mut statements = Vec::new();
        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    DejavuElementType::LetStatement => {
                        let stmt = self.build_let(n, source)?;
                        statements.push(stmt);
                    }
                    DejavuElementType::ExprStatement => {
                        let stmt = self.build_expr_stmt(n, source)?;
                        statements.push(stmt);
                    }
                    _ => {
                        // Handle expression as statement if not explicitly an ExprStatement
                        if let Ok(expr) = self.build_expr(n.clone(), source) {
                            statements.push(StatementNode::Expr(ExpressionStatement { annotations: vec![], expr, semi: false, span: n.span() }));
                        }
                    }
                },
                RedTree::Leaf(_) => {} // Ignore braces and whitespace
            }
        }
        Ok(BlockNode { statements, span })
    }
}
