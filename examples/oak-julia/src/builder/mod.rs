use crate::{
    ast::{JuliaExpression, JuliaFunction, JuliaRoot, JuliaStatement},
    language::JuliaLanguage,
    lexer::token_type::JuliaTokenType,
    parser::{JuliaParser, element_type::JuliaElementType},
};
use oak_core::{Builder, BuilderCache, GreenNode, GreenTree, OakDiagnostics, OakError, Parser, Source, SourceText, TextEdit, builder::BuildOutput};

pub struct JuliaBuilder<'config> {
    config: &'config JuliaLanguage,
}

impl<'config> JuliaBuilder<'config> {
    pub fn new(config: &'config JuliaLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<JuliaLanguage> for JuliaBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<JuliaLanguage>) -> BuildOutput<JuliaLanguage> {
        let parser = JuliaParser::new(self.config);

        let mut parse_cache = oak_core::parser::session::ParseSession::<JuliaLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut parse_cache);

        match parse_result.result {
            Ok(green_tree) => {
                let text = source.get_text_in((0..source.length()).into()).into_owned();
                let source_text = SourceText::new(text);
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

impl<'config> JuliaBuilder<'config> {
    pub fn build_root(&self, green_tree: &GreenNode<JuliaLanguage>, source: &SourceText) -> Result<JuliaRoot, OakError> {
        let mut statements = Vec::new();
        let mut current_offset = 0;

        for child in green_tree.children() {
            let child_len = child.len() as usize;
            match child {
                GreenTree::Node(node) => {
                    if !node.kind.is_trivia() {
                        if let Some(stmt) = self.build_statement(node, current_offset, source)? {
                            statements.push(stmt)
                        }
                    }
                }
                _ => {}
            }
            current_offset += child_len
        }

        Ok(JuliaRoot { statements })
    }

    fn build_statement(&self, node: &GreenNode<JuliaLanguage>, offset: usize, source: &SourceText) -> Result<Option<JuliaStatement>, OakError> {
        match node.kind {
            JuliaElementType::Function => {
                let mut name = String::new();
                let mut body = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Node(inner_node) if inner_node.kind == JuliaElementType::Identifier => name = source.get_text_in((current_offset..current_offset + child_len).into()).to_string(),
                        GreenTree::Leaf(leaf) if leaf.kind == JuliaTokenType::Identifier => name = source.get_text_in((current_offset..current_offset + child_len).into()).to_string(),
                        GreenTree::Node(inner_node) => {
                            if !inner_node.kind.is_trivia() {
                                if let Some(stmt) = self.build_statement(inner_node, current_offset, source)? {
                                    body.push(stmt)
                                }
                            }
                        }
                        _ => {}
                    }
                    current_offset += child_len
                }
                Ok(Some(JuliaStatement::Function(JuliaFunction { name, body })))
            }
            JuliaElementType::Call => {
                if let Some(expr) = self.build_expression(node, offset, source)? {
                    Ok(Some(JuliaStatement::Expression(expr)))
                }
                else {
                    Ok(None)
                }
            }
            _ => {
                if let Some(expr) = self.build_expression(node, offset, source)? {
                    Ok(Some(JuliaStatement::Expression(expr)))
                }
                else {
                    Ok(None)
                }
            }
        }
    }

    fn build_expression(&self, node: &GreenNode<JuliaLanguage>, offset: usize, source: &SourceText) -> Result<Option<JuliaExpression>, OakError> {
        match node.kind {
            JuliaElementType::Identifier => {
                let text = source.get_text_in((offset..offset + node.text_len() as usize).into()).to_string();
                Ok(Some(JuliaExpression::Identifier(text)))
            }
            JuliaElementType::StringLiteral => {
                let text = source.get_text_in((offset..offset + node.text_len() as usize).into()).to_string();
                Ok(Some(JuliaExpression::Literal(text)))
            }
            JuliaElementType::Call => {
                let mut callee = None;
                let mut arguments = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Node(inner_node) if inner_node.kind == JuliaElementType::Identifier => callee = Some(Box::new(JuliaExpression::Identifier(source.get_text_in((current_offset..current_offset + child_len).into()).to_string()))),
                        GreenTree::Leaf(leaf) if leaf.kind == JuliaTokenType::Identifier => callee = Some(Box::new(JuliaExpression::Identifier(source.get_text_in((current_offset..current_offset + child_len).into()).to_string()))),
                        GreenTree::Node(inner_node) if inner_node.kind == JuliaElementType::ArgumentList => {
                            let mut arg_offset = current_offset;
                            for arg_child in inner_node.children() {
                                let arg_child_len = arg_child.len() as usize;
                                if let GreenTree::Node(arg_node) = arg_child {
                                    if !arg_node.kind.is_trivia() {
                                        if let Some(expr) = self.build_expression(arg_node, arg_offset, source)? {
                                            arguments.push(expr)
                                        }
                                    }
                                }
                                arg_offset += arg_child_len
                            }
                        }
                        _ => {}
                    }
                    current_offset += child_len
                }

                if let Some(callee) = callee { Ok(Some(JuliaExpression::Call { callee, arguments })) } else { Ok(None) }
            }
            _ => Ok(None),
        }
    }
}
