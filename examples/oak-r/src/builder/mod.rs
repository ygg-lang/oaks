use crate::{
    RParser,
    ast::{Expr, Identifier, RRoot, Statement},
    language::RLanguage,
    lexer::token_type::RTokenType,
    parser::element_type::RElementType,
};
use oak_core::{Builder, BuilderCache, GreenNode, GreenTree, OakDiagnostics, OakError, Parser, SourceText, TextEdit, builder::BuildOutput, source::Source};

#[derive(Clone)]
pub struct RBuilder<'config> {
    config: &'config RLanguage,
}

impl<'config> RBuilder<'config> {
    pub fn new(config: &'config RLanguage) -> Self {
        Self { config }
    }

    pub fn build_root(&self, green_tree: &GreenNode<RLanguage>, source: &SourceText) -> Result<RRoot, OakError> {
        let mut statements = Vec::new();
        let mut current_offset = 0;

        for child in green_tree.children() {
            let child_len = child.len() as usize;
            if let GreenTree::Node(node) = child {
                if !node.kind.is_trivia() {
                    if let Some(stmt) = self.build_statement(node, current_offset, source)? {
                        statements.push(stmt);
                    }
                }
            }
            current_offset += child_len;
        }

        Ok(RRoot { statements })
    }

    fn build_statement(&self, node: &GreenNode<RLanguage>, offset: usize, source: &SourceText) -> Result<Option<Statement>, OakError> {
        match node.kind {
            RElementType::Assignment => {
                let mut name = None;
                let mut expr = None;
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Node(sub_node) => {
                            if sub_node.kind == RElementType::Identifier {
                                name = Some(Identifier { name: source.get_text_in((current_offset..current_offset + child_len).into()).to_string(), span: (current_offset..current_offset + child_len).into() });
                            }
                            else if sub_node.kind == RElementType::LiteralExpression || sub_node.kind == RElementType::IdentifierExpression || sub_node.kind == RElementType::CallExpression {
                                expr = Some(self.build_expression(sub_node, current_offset, source)?)
                            }
                        }
                        _ => {}
                    }
                    current_offset += child_len
                }

                if let (Some(n), Some(e)) = (name, expr) { Ok(Some(Statement::Assignment { name: n, expr: e, span: (offset..current_offset).into() })) } else { Ok(None) }
            }
            RElementType::Function => {
                let name = Identifier { name: "anonymous".to_string(), span: (offset..offset).into() };
                let mut params = Vec::new();
                let mut body = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(sub_node) = child {
                        match sub_node.kind {
                            RElementType::Identifier => params.push(Identifier { name: source.get_text_in((current_offset..current_offset + child_len).into()).to_string(), span: (current_offset..current_offset + child_len).into() }),
                            RElementType::BlockExpression => {
                                let mut block_offset = current_offset;
                                for block_child in sub_node.children() {
                                    let block_child_len = block_child.len() as usize;
                                    if let GreenTree::Node(stmt_node) = block_child {
                                        if let Some(s) = self.build_statement(stmt_node, block_offset, source)? {
                                            body.push(s)
                                        }
                                    }
                                    block_offset += block_child_len
                                }
                            }
                            _ => {}
                        }
                    }
                    current_offset += child_len
                }

                Ok(Some(Statement::FunctionDef { name, params, body, span: (offset..current_offset).into() }))
            }
            _ => {
                if let Ok(expr) = self.build_expression(node, offset, source) {
                    Ok(Some(Statement::ExprStmt { expr, span: (offset..offset + node.text_len() as usize).into() }))
                }
                else {
                    Ok(None)
                }
            }
        }
    }

    fn build_expression(&self, node: &GreenNode<RLanguage>, offset: usize, source: &SourceText) -> Result<Expr, OakError> {
        match node.kind {
            RElementType::IdentifierExpression | RElementType::Identifier => Ok(Expr::Ident(Identifier { name: source.get_text_in((offset..offset + node.text_len() as usize).into()).to_string(), span: (offset..offset + node.text_len() as usize).into() })),
            RElementType::LiteralExpression => {
                let text = source.get_text_in((offset..offset + node.text_len() as usize).into()).to_string();
                if text == "TRUE" || text == "FALSE" {
                    Ok(Expr::Bool { value: text == "TRUE", span: (offset..offset + node.text_len() as usize).into() })
                }
                else if text == "NULL" {
                    Ok(Expr::Null { span: (offset..offset + node.text_len() as usize).into() })
                }
                else {
                    Ok(Expr::Literal { value: text, span: (offset..offset + node.text_len() as usize).into() })
                }
            }
            RElementType::CallExpression => {
                let mut callee = None;
                let mut args = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(sub_node) = child {
                        if callee.is_none() && (sub_node.kind == RElementType::Identifier || sub_node.kind == RElementType::IdentifierExpression) {
                            callee = Some(Box::new(self.build_expression(sub_node, current_offset, source)?))
                        }
                        else if sub_node.kind != RElementType::LeftParen && sub_node.kind != RElementType::RightParen && sub_node.kind != RElementType::Comma && !sub_node.kind.is_trivia() {
                            args.push(self.build_expression(sub_node, current_offset, source)?)
                        }
                    }
                    current_offset += child_len
                }

                if let Some(c) = callee { Ok(Expr::Call { callee: c, args, span: (offset..offset + node.text_len() as usize).into() }) } else { Err(OakError::custom_error(format!("Unexpected token at offset {}", offset))) }
            }
            _ => Err(OakError::custom_error(format!("Unexpected token at offset {}", offset))),
        }
    }
}

impl<'config> Builder<RLanguage> for RBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<RLanguage>) -> BuildOutput<RLanguage> {
        let parser = RParser::new(self.config);
        let mut parse_cache = oak_core::parser::session::ParseSession::<RLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut parse_cache);

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
            Err(e) => OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
        }
    }
}
