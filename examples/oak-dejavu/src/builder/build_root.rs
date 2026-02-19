use crate::{
    DejavuLanguage,
    ast::{DejavuRoot, ItemNode},
    builder::DejavuBuilder,
    lexer::token_type::DejavuTokenType,
    parser::element_type::DejavuElementType,
};
use oak_core::{GreenNode, OakError, RedNode, RedTree, Source};

impl<'config> DejavuBuilder<'config> {
    /// Builds a strongly-typed AST from a green tree.
    pub fn build_root<S: Source + ?Sized>(&self, green_tree: &GreenNode<DejavuLanguage>, source: &S) -> Result<DejavuRoot, OakError> {
        // println!("Building root from green tree: {:?}", green_tree.kind);
        let red_root = RedNode::<DejavuLanguage>::new(green_tree, 0);
        let mut items = Vec::new();
        for child in red_root.children() {
            match child {
                RedTree::Node(n) => match self.build_item(n, source) {
                    Ok(item) => items.push(item),
                    Err(err) => {
                        // println!("Failed to build item in root: {:?} at {:?}: {:?}", n.green.kind, n.span(), err);
                        return Err(err);
                    }
                },
                RedTree::Leaf(t) => match t.kind {
                    DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                    DejavuTokenType::Eof => continue,
                    _ => {
                        // println!("Unexpected token in root: {:?} at {:?}", t.kind, t.span);
                        return Err(source.syntax_error(format!("Unexpected token in root: {:?}", t.kind), t.span.start));
                    }
                },
            }
        }
        Ok(DejavuRoot { items })
    }

    pub(crate) fn build_item<S: Source + ?Sized>(&self, n: RedNode<DejavuLanguage>, source: &S) -> Result<ItemNode, OakError> {
        use crate::ast::{ExpressionNode, ExpressionStatement, IdentifierNode, StatementNode, TemplateControlNode, TemplateInterpolationNode};
        match n.green.kind {
            DejavuElementType::Namespace => {
                let ns = self.build_namespace(n, source)?;
                Ok(ItemNode::Namespace(ns))
            }
            DejavuElementType::Class => {
                let class = self.build_class(n, source)?;
                Ok(ItemNode::Class(class))
            }
            DejavuElementType::Attribute => {
                let attr = self.build_attribute(n, source)?;
                Ok(ItemNode::Statement(StatementNode::Expr(ExpressionStatement { annotations: vec![attr], expr: ExpressionNode::Ident(IdentifierNode { name: "".to_string(), span: (0..0).into() }), semi: false, span: (0..0).into() })))
            }
            DejavuElementType::Micro => {
                let micro = self.build_micro(n, source)?;
                Ok(ItemNode::Micro(micro))
            }
            DejavuElementType::Mezzo => {
                let mezzo = self.build_mezzo(n, source)?;
                Ok(ItemNode::TypeFunction(mezzo))
            }
            DejavuElementType::LetStatement => {
                let stmt = self.build_let(n, source)?;
                Ok(ItemNode::Statement(stmt))
            }
            DejavuElementType::ExprStatement => {
                let stmt = self.build_expr_stmt(n, source)?;
                Ok(ItemNode::Statement(stmt))
            }
            DejavuElementType::Variant => {
                let variant = self.build_variant(n, source)?;
                Ok(ItemNode::Variant(variant))
            }
            DejavuElementType::TemplateControl => {
                let span = n.span();
                let mut items = Vec::new();
                for child in n.children() {
                    if let RedTree::Node(child_node) = child {
                        items.push(self.build_item(child_node, source)?);
                    }
                }
                Ok(ItemNode::TemplateControl(TemplateControlNode { items, span }))
            }
            DejavuElementType::Interpolation => {
                let span = n.span();
                let mut expr = None;
                for child in n.children() {
                    if let RedTree::Node(child_node) = child {
                        expr = Some(self.build_expr(child_node, source)?);
                    }
                }
                let expr = expr.ok_or_else(|| source.syntax_error("Empty interpolation".to_string(), span.start))?;
                Ok(ItemNode::TemplateInterpolation(TemplateInterpolationNode { expr, span }))
            }
            _ => Err(source.syntax_error(format!("Unexpected node kind in item: {:?}", n.green.kind), n.span().start)),
        }
    }
}
