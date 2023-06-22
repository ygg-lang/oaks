use crate::{HandlebarsParser, ast::*, kind::HandlebarsSyntaxKind, language::HandlebarsLanguage};
use core::range::Range;
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, RedTree, SourceText, TextEdit, source::Source};

/// Handlebars 语言的 AST 构建器
#[derive(Clone)]
pub struct HandlebarsBuilder<'config> {
    config: &'config HandlebarsLanguage,
}

impl<'config> HandlebarsBuilder<'config> {
    pub fn new(config: &'config HandlebarsLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<HandlebarsLanguage> for HandlebarsBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<HandlebarsLanguage>) -> OakDiagnostics<HandlebarsRoot> {
        let parser = HandlebarsParser::new(self.config);

        let mut parse_cache = oak_core::parser::session::ParseSession::<HandlebarsLanguage>::default();
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
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}

impl<'config> HandlebarsBuilder<'config> {
    pub(crate) fn build_root<'a>(&self, green_tree: &'a GreenNode<'a, HandlebarsLanguage>, source: &SourceText) -> Result<HandlebarsRoot, OakError> {
        let red_root = RedNode::new(green_tree, 0);
        let mut nodes = Vec::new();

        for child in red_root.children() {
            if let RedTree::Node(n) = child {
                nodes.push(self.build_node(n, source)?);
            }
            else if let RedTree::Leaf(t) = child {
                if t.kind == HandlebarsSyntaxKind::Content {
                    nodes.push(TemplateNode::Content(Content { text: text(source, t.span.clone().into()), span: t.span.clone().into() }));
                }
            }
        }
        Ok(HandlebarsRoot { nodes })
    }

    fn build_node(&self, node: RedNode<HandlebarsLanguage>, source: &SourceText) -> Result<TemplateNode, OakError> {
        match node.green.kind {
            HandlebarsSyntaxKind::Mustache => {
                // Simplified Mustache building
                Ok(TemplateNode::Mustache(Mustache { expression: Expression::Path(text(source, node.span())), is_unescaped: false, span: node.span() }))
            }
            HandlebarsSyntaxKind::Block => Ok(TemplateNode::Block(Block { name: "todo".to_string(), params: Vec::new(), body: Vec::new(), inverse: None, span: node.span() })),
            HandlebarsSyntaxKind::CommentNode => Ok(TemplateNode::Comment(Comment { text: text(source, node.span()), span: node.span() })),
            _ => Ok(TemplateNode::Content(Content { text: text(source, node.span()), span: node.span() })),
        }
    }
}

fn text(source: &SourceText, range: Range<usize>) -> String {
    source.get_text_in(range).to_string()
}
