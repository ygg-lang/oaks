use crate::{ast::*, language::MermaidLanguage, parser::MermaidParser};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, Source, SourceText, TextEdit};

#[derive(Clone)]
pub struct MermaidBuilder<'config> {
    config: &'config MermaidLanguage,
}

impl<'config> MermaidBuilder<'config> {
    pub fn new(config: &'config MermaidLanguage) -> Self {
        Self { config }
    }

    pub fn build_root(&self, green_tree: &GreenNode<MermaidLanguage>, _source: &SourceText) -> Result<MermaidRoot, oak_core::OakError> {
        // 简化实现，实际逻辑需要根据 GreenTree 节点类型递归构建 AST
        Ok(MermaidRoot { elements: Vec::new(), span: (0..green_tree.text_len as usize).into() })
    }
}

impl<'config> Builder<MermaidLanguage> for MermaidBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<MermaidLanguage>) -> oak_core::builder::BuildOutput<MermaidLanguage> {
        let parser = MermaidParser::new(self.config);
        let lexer = crate::lexer::MermaidLexer::new(self.config);
        let mut cache = oak_core::parser::session::ParseSession::<MermaidLanguage>::default();
        let parse_result = oak_core::parser::parse(&parser, &lexer, source, edits, &mut cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                OakDiagnostics { result: self.build_root(&green_tree, &source_text), diagnostics: parse_result.diagnostics }
            }
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}
