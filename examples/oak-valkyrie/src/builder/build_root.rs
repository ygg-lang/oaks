use crate::{
    ValkyrieLanguage, ValkyrieParser,
    ast::{Item, ValkyrieRoot},
    lexer::token_type::ValkyrieSyntaxKind,
};
use oak_core::{GreenNode, OakError, RedNode, RedTree, source::SourceText};

impl<'config> ValkyrieParser<'config> {
    /// Builds a strongly-typed AST from a green tree.
    pub fn build_root(&self, green_tree: &GreenNode<ValkyrieLanguage>, source: &SourceText) -> Result<ValkyrieRoot, OakError> {
        println!("Building root from green tree: {:?}", green_tree.kind);
        let red_root = RedNode::<ValkyrieLanguage>::new(green_tree, 0);
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
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    ValkyrieSyntaxKind::Eof => continue,
                    _ => {
                        println!("Unexpected token in root: {:?} at {:?}", t.kind, t.span);
                        return Err(source.syntax_error(format!("Unexpected token in root: {:?}", t.kind), t.span.start));
                    }
                },
            }
        }
        Ok(ValkyrieRoot { items })
    }

    pub(crate) fn build_item(&self, n: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Item, OakError> {
        use crate::ast::{Expr, Identifier, Statement};
        match n.green.kind {
            ValkyrieSyntaxKind::Namespace => {
                let ns = self.build_namespace(n, source)?;
                Ok(Item::Namespace(ns))
            }
            ValkyrieSyntaxKind::Class => {
                let class = self.build_class(n, source)?;
                Ok(Item::Class(class))
            }
            ValkyrieSyntaxKind::Flags => {
                let flags = self.build_flags(n, source)?;
                Ok(Item::Flags(flags))
            }
            ValkyrieSyntaxKind::Enums => {
                let enums = self.build_enums(n, source)?;
                Ok(Item::Enums(enums))
            }
            ValkyrieSyntaxKind::Trait => {
                let trait_node = self.build_trait(n, source)?;
                Ok(Item::Trait(trait_node))
            }
            ValkyrieSyntaxKind::Widget => {
                let widget = self.build_widget(n, source)?;
                Ok(Item::Widget(widget))
            }
            ValkyrieSyntaxKind::UsingStatement => {
                let us = self.build_using(n, source)?;
                Ok(Item::Using(us))
            }
            ValkyrieSyntaxKind::Micro => {
                let micro = self.build_micro(n, source)?;
                Ok(Item::Micro(micro))
            }
            ValkyrieSyntaxKind::Mezzo => {
                let mezzo = self.build_mezzo(n, source)?;
                Ok(Item::TypeFunction(mezzo))
            }
            ValkyrieSyntaxKind::LetStatement => {
                let stmt = self.build_let(n, source)?;
                Ok(Item::Statement(stmt))
            }
            ValkyrieSyntaxKind::ExpressionStatement => {
                let stmt = self.build_expr_stmt(n, source)?;
                Ok(Item::Statement(stmt))
            }
            ValkyrieSyntaxKind::Variant => {
                let variant = self.build_variant(n, source)?;
                Ok(Item::Variant(variant))
            }
            ValkyrieSyntaxKind::EffectDefinition => {
                let effect = self.build_effect(n, source)?;
                Ok(Item::Effect(effect))
            }
            ValkyrieSyntaxKind::Attribute => {
                let attr = self.build_attribute(n, source)?;
                Ok(Item::Statement(Statement::ExprStmt { annotations: vec![attr], expr: Expr::Ident(Identifier { name: "".to_string(), span: (0..0).into() }), semi: false, span: (0..0).into() }))
            }
            _ => Err(source.syntax_error(format!("Unexpected item: {:?}", n.green.kind), n.span().start)),
        }
    }
}
