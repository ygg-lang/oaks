use crate::{
    DejavuLanguage, DejavuParser,
    ast::{DejavuRoot, Item},
    lexer::token_type::DejavuSyntaxKind,
};
use oak_core::{GreenNode, OakError, RedNode, RedTree, source::SourceText};

impl<'config> DejavuParser<'config> {
    /// Builds a strongly-typed AST from a green tree.
    pub fn build_root(&self, green_tree: &GreenNode<DejavuLanguage>, source: &SourceText) -> Result<DejavuRoot, OakError> {
        println!("Building root from green tree: {:?}", green_tree.kind);
        let red_root = RedNode::<DejavuLanguage>::new(green_tree, 0);
        let mut items = Vec::new();
        for child in red_root.children() {
            match child {
                RedTree::Node(n) => match self.build_item(n, source) {
                    Ok(item) => items.push(item),
                    Err(err) => {
                        println!("Failed to build item in root: {:?} at {:?}: {:?}", n.green.kind, n.span(), err);
                        return Err(err);
                    }
                },
                RedTree::Token(t) => match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::Eof => continue,
                    _ => {
                        println!("Unexpected token in root: {:?} at {:?}", t.kind, t.span);
                        return Err(source.syntax_error(format!("Unexpected token in root: {:?}", t.kind), t.span.start));
                    }
                },
            }
        }
        Ok(DejavuRoot { items })
    }

    pub(crate) fn build_item(&self, n: RedNode<DejavuLanguage>, source: &SourceText) -> Result<Item, OakError> {
        use crate::ast::{Expr, Identifier, Statement};
        match n.green.kind {
            DejavuSyntaxKind::Namespace => {
                let ns = self.build_namespace(n, source)?;
                Ok(Item::Namespace(ns))
            }
            DejavuSyntaxKind::Class => {
                let class = self.build_class(n, source)?;
                Ok(Item::Class(class))
            }
            DejavuSyntaxKind::Flags => {
                let flags = self.build_flags(n, source)?;
                Ok(Item::Flags(flags))
            }
            DejavuSyntaxKind::Enums => {
                let enums = self.build_enums(n, source)?;
                Ok(Item::Enums(enums))
            }
            DejavuSyntaxKind::Trait => {
                let trait_node = self.build_trait(n, source)?;
                Ok(Item::Trait(trait_node))
            }
            DejavuSyntaxKind::Widget => {
                let widget = self.build_widget(n, source)?;
                Ok(Item::Widget(widget))
            }
            DejavuSyntaxKind::UsingStatement => {
                let us = self.build_using(n, source)?;
                Ok(Item::Using(us))
            }
            DejavuSyntaxKind::Micro => {
                let micro = self.build_micro(n, source)?;
                Ok(Item::Micro(micro))
            }
            DejavuSyntaxKind::Mezzo => {
                let mezzo = self.build_mezzo(n, source)?;
                Ok(Item::TypeFunction(mezzo))
            }
            DejavuSyntaxKind::LetStatement => {
                let stmt = self.build_let(n, source)?;
                Ok(Item::Statement(stmt))
            }
            DejavuSyntaxKind::ExpressionStatement => {
                let stmt = self.build_expr_stmt(n, source)?;
                Ok(Item::Statement(stmt))
            }
            DejavuSyntaxKind::Variant => {
                let variant = self.build_variant(n, source)?;
                Ok(Item::Variant(variant))
            }
            DejavuSyntaxKind::EffectDefinition => {
                let effect = self.build_effect(n, source)?;
                Ok(Item::Effect(effect))
            }
            DejavuSyntaxKind::Attribute => {
                let attr = self.build_attribute(n, source)?;
                Ok(Item::Statement(Statement::ExprStmt { annotations: vec![attr], expr: Expr::Ident(Identifier { name: "".to_string(), span: (0..0).into() }), semi: false, span: (0..0).into() }))
            }
            DejavuSyntaxKind::TemplateText => {
                let span = n.span();
                let content = source.slice(span.clone()).to_string();
                Ok(Item::TemplateText { content, span })
            }
            DejavuSyntaxKind::TemplateControl => {
                let span = n.span();
                let mut items = Vec::new();
                for child in n.children() {
                    if let RedTree::Node(child_node) = child {
                        items.push(self.build_item(child_node, source)?);
                    }
                }
                Ok(Item::TemplateControl { items, span })
            }
            DejavuSyntaxKind::Interpolation => {
                let span = n.span();
                let mut expr = None;
                for child in n.children() {
                    if let RedTree::Node(child_node) = child {
                        expr = Some(self.build_expr(child_node, source)?);
                    }
                }
                let expr = expr.ok_or_else(|| source.syntax_error("Empty interpolation", span.start))?;
                Ok(Item::TemplateInterpolation { expr, span })
            }
            _ => Err(source.syntax_error(format!("Unexpected item: {:?}", n.green.kind), n.span().start)),
        }
    }
}
