#![doc = include_str!("readme.md")]
use crate::{CParser, ast::*, language::CLanguage, lexer::CTokenType, parser::CElementType};
use core::range::Range;
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, RedTree, SourceText, TextEdit, builder::BuildOutput, source::Source};

/// AST builder for the C language.
#[derive(Clone, Copy)]
pub struct CBuilder<'config> {
    /// Language configuration.
    config: &'config CLanguage,
}

impl<'config> CBuilder<'config> {
    /// Creates a new `CBuilder` with the given language configuration.
    pub fn new(config: &'config CLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<CLanguage> for CBuilder<'config> {
    /// Builds the C AST from the green tree.
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<CLanguage>) -> BuildOutput<CLanguage> {
        // Parse source code to get green tree.
        let parser = CParser::new(self.config);

        // TODO: Real incremental build should use BuilderCache.
        let mut cache = oak_core::parser::session::ParseSession::<CLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut cache);

        // Check if parsing succeeded.
        match parse_result.result {
            Ok(green_tree) => {
                // Build AST.
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

impl<'config> CBuilder<'config> {
    /// Builds the AST root from the green tree.
    pub(crate) fn build_root<'a>(&self, green_tree: &'a GreenNode<'a, CLanguage>, source: &SourceText) -> Result<CRoot, OakError> {
        let root_node = RedNode::new(green_tree, 0);
        let mut external_declarations = Vec::new();

        for child in root_node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    CElementType::FunctionDefinition => external_declarations.push(ExternalDeclaration::FunctionDefinition(self.build_function_definition(n, source)?)),
                    CElementType::DeclarationStatement => external_declarations.push(ExternalDeclaration::Declaration(self.build_declaration(n, source)?)),
                    _ => {}
                }
            }
        }

        Ok(CRoot { translation_unit: TranslationUnit { external_declarations, span: root_node.span() }, span: root_node.span() })
    }

    /// Builds a function definition from a red node.
    fn build_function_definition(&self, node: RedNode<CLanguage>, source: &SourceText) -> Result<FunctionDefinition, OakError> {
        let mut declaration_specifiers = Vec::new();
        let mut declarator = None;
        let mut compound_statement = None;

        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    CElementType::CompoundStatement => compound_statement = Some(self.build_compound_statement(n, source)?),
                    _ => {}
                },
                RedTree::Leaf(t) => match t.kind {
                    CTokenType::Int => declaration_specifiers.push(DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Int { span: t.span.clone() })),
                    CTokenType::Void => declaration_specifiers.push(DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Void { span: t.span.clone() })),
                    CTokenType::Identifier => {
                        let name = text(source, t.span.clone());
                        declarator = Some(Declarator { pointer: None, direct_declarator: DirectDeclarator::Identifier(name, t.span.clone()), span: t.span.clone() })
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        Ok(FunctionDefinition {
            declaration_specifiers,
            declarator: declarator.unwrap_or_else(|| Declarator { pointer: None, direct_declarator: DirectDeclarator::Identifier("main".to_string(), (0..0).into()), span: (0..0).into() }),
            compound_statement: compound_statement.unwrap_or_else(|| CompoundStatement { block_items: vec![], span: (0..0).into() }),
            span: node.span(),
        })
    }

    fn build_declaration(&self, node: RedNode<CLanguage>, source: &SourceText) -> Result<Declaration, OakError> {
        Ok(Declaration { declaration_specifiers: vec![], init_declarators: vec![], span: node.span() })
    }

    fn build_compound_statement(&self, node: RedNode<CLanguage>, source: &SourceText) -> Result<CompoundStatement, OakError> {
        let mut block_items = Vec::new();

        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    CElementType::ReturnStatement => block_items.push(BlockItem::Statement(Statement::Jump(self.build_return_statement(n, source)?))),
                    CElementType::ExpressionStatement => {
                        if let Some(expr) = self.build_expression(n, source)? {
                            block_items.push(BlockItem::Statement(Statement::Expression(ExpressionStatement { expression: Some(expr), span: n.span() })))
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(CompoundStatement { block_items, span: node.span() })
    }

    fn build_return_statement(&self, node: RedNode<CLanguage>, source: &SourceText) -> Result<JumpStatement, OakError> {
        let mut expression = None;
        for child in node.children() {
            match child {
                RedTree::Node(n) => {
                    if n.green.kind == CElementType::ExpressionStatement {
                        expression = self.build_expression(n, source)?
                    }
                }
                RedTree::Leaf(t) => {
                    if t.kind == CTokenType::IntegerLiteral {
                        let val = text(source, t.span.clone());
                        let int_val = val.parse::<i64>().unwrap_or(0);
                        expression = Some(Expression { kind: Box::new(ExpressionKind::Constant(Constant::Integer(int_val, t.span.clone()), t.span.clone())), span: t.span.clone() })
                    }
                }
                _ => {}
            }
        }
        Ok(JumpStatement::Return(expression, node.span()))
    }

    fn build_expression(&self, node: RedNode<CLanguage>, source: &SourceText) -> Result<Option<Expression>, OakError> {
        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    CTokenType::IntegerLiteral => {
                        let val = text(source, t.span.clone());
                        let int_val = val.parse::<i64>().unwrap_or(0);
                        return Ok(Some(Expression { kind: Box::new(ExpressionKind::Constant(Constant::Integer(int_val, t.span.clone()), t.span.clone())), span: t.span.clone() }));
                    }
                    CTokenType::Identifier => {
                        let name = text(source, t.span.clone());
                        return Ok(Some(Expression { kind: Box::new(ExpressionKind::Identifier(name, t.span.clone())), span: t.span.clone() }));
                    }
                    _ => {}
                },
                RedTree::Node(n) => {
                    // 递归处理表达式子节点
                    if let Some(expr) = self.build_expression(n, source)? {
                        return Ok(Some(expr));
                    }
                }
                _ => {}
            }
        }
        Ok(None)
    }
}

fn text(source: &SourceText, span: core::range::Range<usize>) -> String {
    source.get_text_in(span).to_string()
}
