use crate::{ast::*, language::FortranLanguage};
use core::range::Range;
use oak_core::{
    GreenNode, OakError, Parser, Source, TextEdit,
    builder::{BuildOutput, Builder, BuilderCache},
    tree::red_tree::{RedNode, RedTree},
};

pub struct FortranBuilder<'config> {
    language: &'config FortranLanguage,
}

impl<'config> FortranBuilder<'config> {
    pub fn new(language: &'config FortranLanguage) -> Self {
        Self { language }
    }

    pub fn build_root(&self, green: &GreenNode<FortranLanguage>, _source: &str) -> FortranRoot {
        let red = RedNode::new(green, 0);
        let units = Vec::new();

        for child in red.children() {
            if let RedTree::Node(_node) = child {
                // TODO: 实现真正的 AST 构建逻辑
            }
        }

        FortranRoot { name: None, units, span: Range::from(0..green.byte_length as usize) }
    }
}

impl<'config> Builder<FortranLanguage> for FortranBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<FortranLanguage>) -> BuildOutput<FortranLanguage> {
        let mut session = oak_core::parser::ParseSession::<FortranLanguage>::default();
        let parser = crate::parser::FortranParser::new(self.language);
        let output = parser.parse(source, edits, &mut session);

        let mut result = Err(oak_core::OakError::custom_error("Build failed"));
        if let Ok(green) = &output.result {
            let root = self.build_root(green, source.get_text_in((0..source.length()).into()).as_ref());
            result = Ok(root);
        }

        oak_core::errors::OakDiagnostics { result, diagnostics: output.diagnostics }
    }
}
