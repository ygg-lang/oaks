#![doc = include_str!("readme.md")]
use crate::{ast::*, language::GroovyLanguage, parser::GroovyParser};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, Parser, SourceText, TextEdit, source::Source};

/// AST builder for the Groovy language
#[derive(Clone)]
pub struct GroovyBuilder<'config> {
    /// Language configuration
    config: &'config GroovyLanguage,
}

impl<'config> GroovyBuilder<'config> {
    /// Creates a new Groovy builder
    pub fn new(config: &'config GroovyLanguage) -> Self {
        Self { config }
    }

    /// Builds the AST root node from the syntax tree
    pub fn build_root(&self, green: &GreenNode<GroovyLanguage>, source: &SourceText) -> Result<GroovyRoot, oak_core::OakError> {
        let mut declarations = Vec::new();
        let mut offset = 0;

        for child in green.children() {
            if let oak_core::GreenTree::Node(node) = child {
                if let Some(decl) = self.build_declaration(node, source, offset) {
                    declarations.push(decl)
                }
            }
            offset += child.len() as usize
        }

        Ok(GroovyRoot { declarations, span: (0..source.length()).into() })
    }

    fn build_declaration(&self, node: &oak_core::GreenNode<GroovyLanguage>, source: &SourceText, offset: usize) -> Option<crate::ast::Declaration> {
        use crate::parser::element_type::GroovyElementType;
        let kind: GroovyElementType = node.kind;
        let start = offset;

        match kind {
            GroovyElementType::ClassKeyword => {
                let mut name = "GroovyClass".to_string();
                let mut inner_offset = offset;
                for child in node.children() {
                    if let oak_core::GreenTree::Leaf(leaf) = child {
                        if leaf.kind == crate::lexer::token_type::GroovyTokenType::Identifier {
                            name = source.get_text_in((inner_offset..inner_offset + leaf.length as usize).into()).to_string();
                            break;
                        }
                    }
                    inner_offset += child.len() as usize;
                }
                Some(crate::ast::Declaration::Class { name, members: vec![], span: (start..start + node.byte_length as usize).into() })
            }
            GroovyElementType::DefKeyword => {
                let mut name = "groovyMethod".to_string();
                let mut inner_offset = offset;
                for child in node.children() {
                    if let oak_core::GreenTree::Leaf(leaf) = child {
                        if leaf.kind == crate::lexer::token_type::GroovyTokenType::Identifier {
                            name = source.get_text_in((inner_offset..inner_offset + leaf.length as usize).into()).to_string();
                            break;
                        }
                    }
                    inner_offset += child.len() as usize;
                }
                Some(crate::ast::Declaration::Method { name, params: vec![], span: (start..start + node.byte_length as usize).into() })
            }
            _ => {
                // Check if it's a SourceFile or Root and recurse
                if kind == GroovyElementType::Root || kind == GroovyElementType::SourceFile {
                    // Logic to handle root children is already in build_root
                }
                None
            }
        }
    }
}

impl<'config> Builder<GroovyLanguage> for GroovyBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<GroovyLanguage>) -> oak_core::builder::BuildOutput<GroovyLanguage> {
        let parser = GroovyParser::new(self.config);
        let mut cache = oak_core::parser::ParseSession::<GroovyLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut cache);

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
