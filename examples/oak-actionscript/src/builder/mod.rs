use crate::parser::ActionScriptParser;
#[doc = include_str!("readme.md")]
use crate::{ast::*, language::ActionScriptLanguage, parser::ActionScriptElementType};
use oak_core::{Builder, BuilderCache, GreenNode, Lexer, OakDiagnostics, OakError, Parser, RedNode, RedTree, SourceText, TextEdit, source::Source};

/// ActionScript 语言的 AST 构建器
#[derive(Clone)]
pub struct ActionScriptBuilder<'config> {
    /// 语言配置
    config: &'config ActionScriptLanguage,
}

impl<'config> ActionScriptBuilder<'config> {
    /// 创建新的 ActionScript 构建器
    pub fn new(config: &'config ActionScriptLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<ActionScriptLanguage> for ActionScriptBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<ActionScriptLanguage>) -> oak_core::builder::BuildOutput<ActionScriptLanguage> {
        let parser = ActionScriptParser::new(self.config);
        let lexer = crate::lexer::ActionScriptLexer::new(&self.config);

        let mut cache = oak_core::parser::session::ParseSession::<ActionScriptLanguage>::default();
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

impl<'config> ActionScriptBuilder<'config> {
    /// 构建根节点
    pub(crate) fn build_root(&self, green_tree: GreenNode<ActionScriptLanguage>, _source: &SourceText) -> Result<ActionScriptRoot, OakError> {
        let red_root = RedNode::new(&green_tree, 0);
        let mut items = Vec::new();

        for child in red_root.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    ActionScriptElementType::Class => {
                        items.push(ActionScriptItem::Class);
                    }
                    ActionScriptElementType::Interface => {
                        items.push(ActionScriptItem::Interface);
                    }
                    ActionScriptElementType::Function => {
                        items.push(ActionScriptItem::Function);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        Ok(ActionScriptRoot { items })
    }
}
