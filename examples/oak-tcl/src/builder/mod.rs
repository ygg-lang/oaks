use crate::{
    ast::*,
    language::TclLanguage,
    lexer::{TclLexer, token_type::TclTokenType},
    parser::{TclParser, element_type::TclElementType},
};
use oak_core::{Builder, BuilderCache, GreenNode, Lexer, OakDiagnostics, OakError, Parser, RedNode, RedTree, SourceText, TextEdit, source::Source};

/// Tcl 语言的 AST 构建器
#[derive(Clone)]
pub struct TclBuilder<'config> {
    /// 语言配置
    config: &'config TclLanguage,
}

impl<'config> TclBuilder<'config> {
    /// 创建新的 Tcl 构建器
    pub fn new(config: &'config TclLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<TclLanguage> for TclBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<TclLanguage>) -> oak_core::builder::BuildOutput<TclLanguage> {
        let parser = TclParser::new(self.config);
        let lexer = TclLexer::new(&self.config);

        let mut cache = oak_core::parser::session::ParseSession::<TclLanguage>::default();
        lexer.lex(source, edits, &mut cache);
        let parse_result = parser.parse(source, edits, &mut cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
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

impl<'config> TclBuilder<'config> {
    /// 构建根节点
    pub(crate) fn build_root(&self, green_tree: GreenNode<TclLanguage>, source: &SourceText) -> Result<TclRoot, OakError> {
        let red_root = RedNode::new(&green_tree, 0);
        self.build_root_from_red(red_root, source)
    }

    fn build_root_from_red(&self, red_root: RedNode<TclLanguage>, source: &SourceText) -> Result<TclRoot, OakError> {
        let span = red_root.span();
        let mut items = Vec::new();

        for child in red_root.children() {
            let kind = child.kind::<TclTokenType>();
            if kind == TclElementType::Command.into() {
                if let Some(node) = child.as_node() {
                    items.push(TclItem::Command(self.build_command(node, source)?));
                }
            }
            else if kind == TclTokenType::Comment {
                items.push(TclItem::Comment(TclComment { span: child.span().into(), text: source.get_text_in(child.span()).to_string() }));
            }
        }

        Ok(TclRoot { span: span.into(), items })
    }

    fn build_command(&self, node: RedNode<TclLanguage>, source: &SourceText) -> Result<TclCommand, OakError> {
        let span = node.span();
        let mut words = Vec::new();

        for child in node.children() {
            let kind = child.kind::<TclTokenType>();
            if kind == TclElementType::SimpleWord.into() || kind == TclElementType::VariableWord.into() || kind == TclElementType::ScriptWord.into() || kind == TclElementType::BracedWord.into() {
                if let Some(node) = child.as_node() {
                    words.push(self.build_word(node, source)?);
                }
            }
        }

        Ok(TclCommand { span: span.into(), words })
    }

    fn build_word(&self, node: RedNode<TclLanguage>, source: &SourceText) -> Result<TclWord, OakError> {
        let kind = node.kind::<TclTokenType>();
        if kind == TclElementType::SimpleWord.into() {
            let text = source.get_text_in(node.span()).trim().to_string();
            Ok(TclWord::Simple(text))
        }
        else if kind == TclElementType::VariableWord.into() {
            // $Identifier -> skip $
            let text = source.get_text_in(node.span()).trim().to_string();
            let var_name = if text.starts_with('$') { &text[1..] } else { &text };
            Ok(TclWord::Variable(var_name.to_string()))
        }
        else if kind == TclElementType::ScriptWord.into() {
            // ScriptWord usually contains [ ... ]
            // We want to parse the content inside brackets as a root.
            let root = self.build_root_from_red(node, source)?;
            Ok(TclWord::Script(root))
        }
        else if kind == TclElementType::BracedWord.into() {
            let text = source.get_text_in(node.span()).trim().to_string();
            Ok(TclWord::Braced(text))
        }
        else {
            Err(source.syntax_error(format!("Unknown word type: {:?}", kind), node.span().start))
        }
    }
}
