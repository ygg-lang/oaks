#![doc = include_str!("readme.md")]
use crate::{
    ast::*,
    language::ZigLanguage,
    parser::{ZigParser, element_type::ZigElementType},
};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, Parser, RedNode, RedTree, SourceText, TextEdit, source::Source};
use std::sync::Arc;

/// AST builder for the Zig language
#[derive(Clone)]
pub struct ZigBuilder<'config> {
    /// Language configuration
    config: &'config ZigLanguage,
}

impl<'config> ZigBuilder<'config> {
    /// Creates a new Zig builder
    pub fn new(config: &'config ZigLanguage) -> Self {
        Self { config }
    }

    /// Builds the AST root node from the syntax tree
    pub fn build_root(&self, green: &GreenNode<ZigLanguage>, source: &SourceText) -> Result<ZigRoot, oak_core::OakError> {
        let mut items = Vec::new();
        let red = RedNode::new(green, 0);

        for child in red.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    ZigElementType::Comptime => {
                        // In Zig, comptime can be followed by a block or an expression.
                        // For simplicity, we check if any child is a Block.
                        let mut is_block = false;
                        for child in n.children() {
                            if let RedTree::Node(cn) = child {
                                if cn.green.kind == ZigElementType::Block {
                                    items.push(Item::Comptime(self.build_block(cn, source)?));
                                    is_block = true;
                                    break;
                                }
                            }
                        }
                        if !is_block {
                            // If not a block, it might be an expression.
                            // In a real implementation, we'd build a Statement or Expression node.
                            // For Item::Comptime, we currently only support Arc<Block>.
                        }
                    }
                    ZigElementType::Var | ZigElementType::Const => {
                        let decl = self.build_declaration(n, source)?;
                        items.push(Item::Declaration(decl));
                    }
                    ZigElementType::ContainerField => {
                        let field = self.build_container_field(n, source)?;
                        items.push(Item::ContainerField(field));
                    }
                    _ => {}
                }
            }
        }

        Ok(ZigRoot { items })
    }

    fn build_block(&self, node: RedNode<ZigLanguage>, source: &SourceText) -> Result<Arc<Block>, oak_core::OakError> {
        let mut statements = Vec::new();

        for child in node.children() {
            if let RedTree::Node(n) = child {
                let stmt = self.build_statement(n, source)?;
                statements.push(stmt);
            }
        }

        Ok(Arc::new(Block { statements, span: node.span().into() }))
    }

    fn build_statement(&self, node: RedNode<ZigLanguage>, source: &SourceText) -> Result<Statement, oak_core::OakError> {
        let kind = node.green.kind;
        match kind {
            ZigElementType::Block => Ok(Statement::Block(self.build_block(node, source)?)),
            ZigElementType::IfStatement => {
                // Simplified
                Ok(Statement::Expression(Expression::Identifier("if".to_string())))
            }
            ZigElementType::WhileStatement => {
                // Simplified
                Ok(Statement::Expression(Expression::Identifier("while".to_string())))
            }
            ZigElementType::ForStatement => {
                // Simplified
                Ok(Statement::Expression(Expression::Identifier("for".to_string())))
            }
            ZigElementType::ReturnStatement => {
                // Simplified
                Ok(Statement::Expression(Expression::Identifier("return".to_string())))
            }
            ZigElementType::BreakStatement => {
                // Simplified
                Ok(Statement::Expression(Expression::Identifier("break".to_string())))
            }
            ZigElementType::ContinueStatement => {
                // Simplified
                Ok(Statement::Expression(Expression::Identifier("continue".to_string())))
            }
            ZigElementType::DeferStatement => {
                // Simplified
                Ok(Statement::Expression(Expression::Identifier("defer".to_string())))
            }
            ZigElementType::Comptime => {
                // Check if it's a block
                for child in node.children() {
                    if let RedTree::Node(n) = child {
                        if n.green.kind == ZigElementType::Block {
                            return Ok(Statement::Comptime(self.build_block(n, source)?));
                        }
                    }
                }
                // Fallback or handle expression-based comptime
                Ok(Statement::Expression(Expression::Identifier("comptime_expr".to_string())))
            }
            ZigElementType::Var | ZigElementType::Const => {
                let decl = self.build_declaration(node, source)?;
                Ok(Statement::Declaration(decl))
            }
            _ => {
                // Simplified
                Ok(Statement::Expression(Expression::Identifier("stmt".to_string())))
            }
        }
    }

    fn build_declaration(&self, node: RedNode<ZigLanguage>, _source: &SourceText) -> Result<Arc<Declaration>, oak_core::OakError> {
        let span = node.span();
        let decl = Arc::new(Declaration { name: "decl".to_string(), is_comptime: false, span: span.into() });
        Ok(decl)
    }

    fn build_container_field(&self, node: RedNode<ZigLanguage>, _source: &SourceText) -> Result<Arc<ContainerField>, oak_core::OakError> {
        let span = node.span();
        let field = Arc::new(ContainerField { name: "field".to_string(), span: span.into() });
        Ok(field)
    }
}

impl<'config> Builder<ZigLanguage> for ZigBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<ZigLanguage>) -> oak_core::builder::BuildOutput<ZigLanguage> {
        let parser = ZigParser::new(self.config);
        let parse_result = parser.parse(source, edits, cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                match self.build_root(green_tree, &source_text) {
                    Ok(ast_root) => OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics },
                    Err(build_error) => {
                        let mut diagnostics = parse_result.diagnostics;
                        diagnostics.push(build_error.clone());
                        OakDiagnostics { result: Err(build_error), diagnostics }
                    }
                }
            }
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}
