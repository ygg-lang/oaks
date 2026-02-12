use crate::{AsciiDocParser, ast::*, language::AsciiDocLanguage, lexer::AsciiDocTokenType, parser::AsciiDocElementType};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, RedTree, SourceText, TextEdit, builder::BuildOutput, source::Source};

/// AST builder for the AsciiDoc language.
#[derive(Clone, Copy)]
pub struct AsciiDocBuilder<'config> {
    /// Language configuration.
    config: &'config AsciiDocLanguage,
}

impl<'config> AsciiDocBuilder<'config> {
    /// Creates a new `AsciiDocBuilder` with the given language configuration.
    pub fn new(config: &'config AsciiDocLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<AsciiDocLanguage> for AsciiDocBuilder<'config> {
    /// Builds the AsciiDoc AST from the green tree.
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<AsciiDocLanguage>) -> BuildOutput<AsciiDocLanguage> {
        let parser = AsciiDocParser::new(self.config);

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

impl<'config> AsciiDocBuilder<'config> {
    /// Builds the AST root from the green tree.
    pub(crate) fn build_root<'a>(&self, green_tree: &'a GreenNode<'a, AsciiDocLanguage>, source: &SourceText) -> Result<AsciiDocRoot, OakError> {
        let root_node = RedNode::new(green_tree, 0);
        let mut elements = Vec::new();

        for child in root_node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    AsciiDocElementType::Header1 => elements.push(self.build_header(n, source, 1)?),
                    AsciiDocElementType::Header2 => elements.push(self.build_header(n, source, 2)?),
                    AsciiDocElementType::Header3 => elements.push(self.build_header(n, source, 3)?),
                    AsciiDocElementType::Header4 => elements.push(self.build_header(n, source, 4)?),
                    AsciiDocElementType::Header5 => elements.push(self.build_header(n, source, 5)?),
                    AsciiDocElementType::Header6 => elements.push(self.build_header(n, source, 6)?),
                    AsciiDocElementType::Text => elements.push(Element::Text { content: text(source, n.span()), span: n.span() }),
                    AsciiDocElementType::BoldMarker => elements.push(Element::Bold { content: text(source, n.span()), span: n.span() }),
                    AsciiDocElementType::ItalicMarker => elements.push(Element::Italic { content: text(source, n.span()), span: n.span() }),
                    AsciiDocElementType::MonospaceMarker => elements.push(Element::Monospace { content: text(source, n.span()), span: n.span() }),
                    AsciiDocElementType::CodeBlockMarker => elements.push(Element::CodeBlock { content: text(source, n.span()), span: n.span() }),
                    AsciiDocElementType::LinkMarker => elements.push(Element::Link { url: text(source, n.span()), text: None, span: n.span() }),
                    AsciiDocElementType::ListMarker => elements.push(Element::ListItem { content: text(source, n.span()), span: n.span() }),
                    AsciiDocElementType::Comment => elements.push(Element::Comment { content: text(source, n.span()), span: n.span() }),
                    _ => {}
                }
            }
            else if let RedTree::Leaf(t) = child {
                match t.kind {
                    AsciiDocTokenType::Text => elements.push(Element::Text { content: text(source, t.span.clone()), span: t.span.clone() }),
                    _ => {}
                }
            }
        }

        Ok(AsciiDocRoot { elements })
    }

    fn build_header(&self, node: RedNode<AsciiDocLanguage>, source: &SourceText, level: u8) -> Result<Element, OakError> {
        Ok(Element::Header { level, text: text(source, node.span()), span: node.span() })
    }
}

fn text(source: &SourceText, span: core::range::Range<usize>) -> String {
    source.get_text_in(span).to_string()
}
