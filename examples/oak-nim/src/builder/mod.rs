#![doc = include_str!("readme.md")]
use crate::{
    ast::*,
    language::NimLanguage,
    parser::{NimElementType, NimParser},
};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, RedTree, TextEdit, builder::BuildOutput, source::Source};
use std::sync::Arc;

/// Root structure for Nim build output.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NimRoot {
    /// List of items in the root.
    pub items: Vec<Item>,
}

/// A builder for Nim language.
pub struct NimBuilder<'config> {
    /// Language configuration.
    pub config: &'config NimLanguage,
}

impl<'config> NimBuilder<'config> {
    /// Creates a new Nim builder.
    pub fn new(config: &'config NimLanguage) -> Self {
        Self { config }
    }

    /// Builds the AST root node.
    pub fn build_root<'a, S: Source + ?Sized>(&self, green: &'a GreenNode<'a, NimLanguage>, _source: &S) -> Result<NimRoot, OakError> {
        let mut items = Vec::new();
        let red = RedNode::new(green, 0);

        for child in red.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    NimElementType::ProcDecl | NimElementType::VarDecl | NimElementType::ConstDecl | NimElementType::LetDecl | NimElementType::TypeDecl => {
                        let decl = self.build_declaration(n)?;
                        items.push(Item::Declaration(decl));
                    }
                    NimElementType::WhenStmt | NimElementType::StaticStmt | NimElementType::IfStmt | NimElementType::WhileStmt | NimElementType::ForStmt | NimElementType::BlockStmt => {
                        let stmt = self.build_statement(n)?;
                        items.push(Item::Statement(stmt));
                    }
                    _ => {}
                }
            }
        }

        Ok(NimRoot { items })
    }

    fn build_declaration<'a>(&self, node: RedNode<'a, NimLanguage>) -> Result<Arc<Declaration>, OakError> {
        let span = node.span();
        let decl = Arc::new(Declaration { name: "decl".to_string(), span });
        Ok(decl)
    }

    fn build_statement<'a>(&self, node: RedNode<'a, NimLanguage>) -> Result<Statement, OakError> {
        match node.green.kind {
            NimElementType::StaticStmt => {
                // Find the block in the static statement
                for child in node.children() {
                    if let RedTree::Node(n) = child {
                        if n.green.kind == NimElementType::BlockStmt {
                            return Ok(Statement::Static(self.build_block(n)?));
                        }
                    }
                }
                // Fallback: use the node itself if it's already a block or similar
                Ok(Statement::Static(self.build_block(node)?))
            }
            NimElementType::WhenStmt => {
                let mut branches = Vec::new();
                let else_branch = None;

                for child in node.children() {
                    if let RedTree::Node(n) = child {
                        match n.green.kind {
                            NimElementType::Expression => {
                                // This would be the condition for the first branch
                                // Simplified: we need a better way to pair condition and body
                            }
                            NimElementType::BlockStmt => {
                                // This would be a branch body or else body
                                branches.push(WhenBranch { condition: Expression::Identifier("when_cond".to_string()), body: self.build_block(n)? });
                            }
                            _ => {}
                        }
                    }
                }
                Ok(Statement::When { branches, else_branch })
            }
            _ => {
                // Simplified: other statements
                Ok(Statement::Expression(Expression::Identifier("stmt".to_string())))
            }
        }
    }

    fn build_block<'a>(&self, node: RedNode<'a, NimLanguage>) -> Result<Arc<Block>, OakError> {
        let span = node.span();
        let mut statements = Vec::new();

        for child in node.children() {
            if let RedTree::Node(n) = child {
                statements.push(self.build_statement(n)?);
            }
        }

        let block = Arc::new(Block { statements, span });
        Ok(block)
    }
}

impl<'config> Builder<NimLanguage> for NimBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<NimLanguage>) -> BuildOutput<NimLanguage> {
        let parser = NimParser::new(self.config);
        let mut session = oak_core::parser::session::ParseSession::<NimLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut session);

        match parse_result.result {
            Ok(green_tree) => match self.build_root(green_tree, source) {
                Ok(ast_root) => OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics },
                Err(e) => {
                    let mut diagnostics = parse_result.diagnostics;
                    diagnostics.push(e.clone());
                    OakDiagnostics { result: Err(e), diagnostics }
                }
            },
            Err(e) => OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
        }
    }
}
