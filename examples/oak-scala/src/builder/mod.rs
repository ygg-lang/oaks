#![doc = include_str!("readme.md")]
use crate::{ast::ScalaRoot, language::ScalaLanguage, parser::ScalaParser};
use oak_core::{Builder, BuilderCache, Lexer, OakDiagnostics, Parser, RedNode, TextEdit, source::Source};

/// AST builder for the Scala language
#[derive(Clone)]
pub struct ScalaBuilder<'config> {
    config: &'config ScalaLanguage,
}

impl<'config> ScalaBuilder<'config> {
    /// Creates a new ScalaBuilder with the given language configuration.
    pub fn new(config: &'config ScalaLanguage) -> Self {
        Self { config }
    }

    pub(crate) fn build_root(&self, green_tree: oak_core::GreenNode<ScalaLanguage>, source: &oak_core::SourceText) -> Result<ScalaRoot, oak_core::OakError> {
        let mut declarations = Vec::new();
        let mut offset = 0;

        for child in green_tree.children() {
            if let oak_core::GreenTree::Node(node) = child {
                if let Some(decl) = self.build_declaration(node, source, offset) {
                    declarations.push(decl)
                }
            }
            offset += child.len() as usize
        }

        Ok(ScalaRoot { span: (0..source.length()).into(), declarations })
    }

    fn build_declaration(&self, node: &oak_core::GreenNode<ScalaLanguage>, source: &oak_core::SourceText, offset: usize) -> Option<crate::ast::Declaration> {
        use crate::{lexer::token_type::ScalaTokenType, parser::element_type::ScalaElementType};
        let kind: ScalaElementType = node.kind;
        let start = offset;

        match kind {
            ScalaElementType::Class => {
                let (name, members) = self.parse_container(node, source, offset);
                Some(crate::ast::Declaration::Class { name, members, span: (start..start + node.byte_length as usize).into() })
            }
            ScalaElementType::Object => {
                let (name, members) = self.parse_container(node, source, offset);
                Some(crate::ast::Declaration::Object { name, members, span: (start..start + node.byte_length as usize).into() })
            }
            ScalaElementType::Trait => {
                let (name, members) = self.parse_container(node, source, offset);
                Some(crate::ast::Declaration::Trait { name, members, span: (start..start + node.byte_length as usize).into() })
            }
            ScalaElementType::Def => {
                let name = self.find_identifier(node, source, offset).unwrap_or_else(|| "method".to_string());
                Some(crate::ast::Declaration::Def { name, span: (start..start + node.byte_length as usize).into() })
            }
            ScalaElementType::Val | ScalaElementType::Var => {
                let name = self.find_identifier(node, source, offset).unwrap_or_else(|| "v".to_string());
                Some(crate::ast::Declaration::Val { name, span: (start..start + node.byte_length as usize).into() })
            }
            _ => None,
        }
    }

    fn parse_container(&self, node: &oak_core::GreenNode<ScalaLanguage>, source: &oak_core::SourceText, offset: usize) -> (String, Vec<crate::ast::Declaration>) {
        use crate::parser::element_type::ScalaElementType;
        let mut name = "InferredName".to_string();
        let mut members = vec![];
        let mut inner_offset = offset;
        let mut found_name = false;

        for child in node.children() {
            match child {
                oak_core::GreenTree::Node(child_node) => {
                    let child_kind: ScalaElementType = child_node.kind;
                    if child_kind == ScalaElementType::SourceFile { // Assuming container body is parsed similarly
                        // Logic for nested members
                    }
                    if let Some(decl) = self.build_declaration(child_node, source, inner_offset) {
                        members.push(decl);
                    }
                }
                oak_core::GreenTree::Leaf(leaf) => {
                    if leaf.kind == crate::lexer::token_type::ScalaTokenType::Identifier && !found_name {
                        name = source.get_text_in((inner_offset..inner_offset + leaf.length as usize).into()).to_string();
                        found_name = true;
                    }
                }
            }
            inner_offset += child.len() as usize;
        }
        (name, members)
    }

    fn find_identifier(&self, node: &oak_core::GreenNode<ScalaLanguage>, source: &oak_core::SourceText, offset: usize) -> Option<String> {
        let mut inner_offset = offset;
        for child in node.children() {
            if let oak_core::GreenTree::Leaf(leaf) = child {
                if leaf.kind == crate::lexer::token_type::ScalaTokenType::Identifier {
                    return Some(source.get_text_in((inner_offset..inner_offset + leaf.length as usize).into()).to_string());
                }
            }
            inner_offset += child.len() as usize;
        }
        None
    }
}

impl<'config> Builder<ScalaLanguage> for ScalaBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<ScalaLanguage>) -> OakDiagnostics<ScalaRoot> {
        let parser = ScalaParser::new(self.config);
        let lexer = crate::lexer::ScalaLexer::new(&self.config);

        let mut session = oak_core::parser::session::ParseSession::<ScalaLanguage>::default();
        lexer.lex(source, edits, &mut session);
        let parse_result = parser.parse(source, edits, &mut session);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = oak_core::SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                match self.build_root(green_tree.clone(), &source_text) {
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
