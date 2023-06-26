use crate::DejavuLanguage;
use oak_core::{
    Builder, OakDiagnostics, Source, TextEdit,
    builder::{BuildOutput, BuilderCache},
};

/// Dejavu builder.
pub struct DejavuBuilder<'config> {
    language: &'config DejavuLanguage,
}

impl<'config> DejavuBuilder<'config> {
    /// Create a new Dejavu builder.
    pub fn new(language: &'config DejavuLanguage) -> Self {
        Self { language }
    }
}

impl<'config> Builder<DejavuLanguage> for DejavuBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, text: &'a S, _edits: &[TextEdit], cache: &'a mut impl BuilderCache<DejavuLanguage>) -> BuildOutput<DejavuLanguage> {
        let parse_output = oak_core::parser::parse_one_pass(&crate::DejavuParser::new(self.language), text, cache);

        let root = parse_output.result.and_then(|green_node| self.build_root(green_node, text));

        OakDiagnostics { result: root, diagnostics: parse_output.diagnostics }
    }
}

/// Block building utilities.
pub mod build_block;
/// Class building utilities.
pub mod build_class;
/// Control flow building utilities.
pub mod build_control_flow;
/// Expression building utilities.
pub mod build_expr;
/// Micro definition building utilities.
pub mod build_micro;
/// Namespace building utilities.
pub mod build_namespace;
/// Pattern building utilities.
pub mod build_pattern;
/// Root building utilities.
pub mod build_root;
/// Statement building utilities.
pub mod build_stmt;

pub(crate) fn text(source: &(impl Source + ?Sized), range: oak_core::Range<usize>) -> String {
    source.get_text_in(range).to_string()
}
