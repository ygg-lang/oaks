use crate::{ast::*, language::NginxLanguage, lexer::token_type::NginxTokenType, parser::element_type::NginxElementType};
use oak_core::{
    Builder, BuilderCache, GreenNode, Lexer, OakDiagnostics, OakError, Parser, RedNode, RedTree, TextEdit,
    source::{Source, SourceText},
};

/// Builder for Nginx AST, coordinating lexing and parsing.
pub struct NginxBuilder<'config> {
    config: &'config NginxLanguage,
}

impl<'config> NginxBuilder<'config> {
    /// Creates a new `NginxBuilder` with the given configuration.
    pub fn new(config: &'config NginxLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<NginxLanguage> for NginxBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<NginxLanguage>) -> OakDiagnostics<NginxRoot> {
        let parser = crate::parser::NginxParser::new(self.config);
        let lexer = crate::lexer::NginxLexer::new(&self.config);

        let mut session = oak_core::parser::session::ParseSession::<NginxLanguage>::default();
        lexer.lex(source, edits, &mut session);
        let parse_result = parser.parse(source, edits, &mut session);

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

impl<'config> NginxBuilder<'config> {
    pub(crate) fn build_root(&self, green_tree: &GreenNode<NginxLanguage>, source: &SourceText) -> Result<NginxRoot, OakError> {
        let red_root = RedNode::new(green_tree, 0);
        let mut items = Vec::new();

        for child in red_root.children() {
            if let RedTree::Node(node) = child {
                match node.green.kind {
                    NginxElementType::Directive => {
                        items.push(NginxItem::Directive(self.build_directive(node, source)?));
                    }
                    NginxElementType::Block => {
                        items.push(NginxItem::Block(self.build_block(node, source)?));
                    }
                    NginxElementType::Comment => {
                        items.push(NginxItem::Comment(self.build_comment(node, source)?));
                    }
                    _ => {}
                }
            }
        }

        Ok(NginxRoot { range: red_root.span(), items })
    }

    fn build_directive(&self, node: RedNode<NginxLanguage>, source: &SourceText) -> Result<Directive, OakError> {
        let mut name = String::new();
        let mut parameters = Vec::new();
        let mut children = node.children();

        // The first non-whitespace/non-comment token should be the directive name
        while let Some(child) = children.next() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind != NginxTokenType::Whitespace && t.kind != NginxTokenType::CommentToken {
                        name = source.get_text_in(t.span()).as_ref().to_string();
                        break;
                    }
                }
                _ => {}
            }
        }

        // Subsequent children are parameters or punctuation
        for child in children {
            match child {
                RedTree::Node(n) if n.green.kind == NginxElementType::Parameter => {
                    parameters.push(source.get_text_in(n.span()).as_ref().trim().to_string());
                }
                RedTree::Leaf(t) if t.kind != NginxTokenType::Whitespace && t.kind != NginxTokenType::Semicolon && t.kind != NginxTokenType::CommentToken => {
                    parameters.push(source.get_text_in(t.span()).as_ref().to_string());
                }
                _ => {}
            }
        }

        Ok(Directive { name, parameters, range: node.span() })
    }

    fn build_block(&self, node: RedNode<NginxLanguage>, source: &SourceText) -> Result<Block, OakError> {
        let mut name = String::new();
        let mut parameters = Vec::new();
        let mut items = Vec::new();
        let mut children = node.children();

        // First non-whitespace/non-comment token is block name (http, server, etc.)
        while let Some(child) = children.next() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind != NginxTokenType::Whitespace && t.kind != NginxTokenType::CommentToken {
                        name = source.get_text_in(t.span()).as_ref().to_string();
                        break;
                    }
                }
                _ => {}
            }
        }

        // Parameters and items
        for child in children {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    NginxElementType::Parameter => {
                        parameters.push(source.get_text_in(n.span()).as_ref().trim().to_string());
                    }
                    NginxElementType::Directive => {
                        items.push(NginxItem::Directive(self.build_directive(n, source)?));
                    }
                    NginxElementType::Block => {
                        items.push(NginxItem::Block(self.build_block(n, source)?));
                    }
                    NginxElementType::Comment => {
                        items.push(NginxItem::Comment(self.build_comment(n, source)?));
                    }
                    _ => {}
                },
                RedTree::Leaf(t) if t.kind != NginxTokenType::Whitespace && t.kind != NginxTokenType::LeftBrace && t.kind != NginxTokenType::RightBrace && t.kind != NginxTokenType::CommentToken => {
                    parameters.push(source.get_text_in(t.span()).as_ref().to_string());
                }
                _ => {}
            }
        }

        Ok(Block { name, parameters, items, range: node.span() })
    }

    fn build_comment(&self, node: RedNode<NginxLanguage>, source: &SourceText) -> Result<Comment, OakError> {
        Ok(Comment { text: source.get_text_in(node.span()).as_ref().to_string(), range: node.span() })
    }
}
