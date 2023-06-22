use crate::{
    JsonLanguage,
    ast::{JsonNull, JsonRoot, JsonValue},
    lexer::JsonLexer,
    parser::JsonParser,
};
use oak_core::{Builder, BuilderCache, Lexer, OakDiagnostics, Parser, SourceText, TextEdit, parser::session::ParseSession, source::Source};

/// JSON AST 构建器
#[derive(Clone)]
pub struct JsonBuilder<'config> {
    config: &'config JsonLanguage,
}

impl<'config> JsonBuilder<'config> {
    pub fn new(config: &'config JsonLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<JsonLanguage> for JsonBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<JsonLanguage>) -> OakDiagnostics<JsonRoot> {
        let parser = JsonParser::new(self.config);
        let lexer = JsonLexer::new(self.config);

        let mut cache = ParseSession::<JsonLanguage>::default();
        lexer.lex(source, edits, &mut cache);
        let parse_result = parser.parse(source, edits, &mut cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()));
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

impl<'config> JsonBuilder<'config> {
    fn build_root(&self, _green_tree: oak_core::GreenNode<JsonLanguage>, _source: &SourceText) -> Result<JsonRoot, oak_core::OakError> {
        // TODO: 从 GreenNode 构建 AST
        Ok(JsonRoot { value: JsonValue::Null(JsonNull { span: (0..0).into() }) })
    }
}
