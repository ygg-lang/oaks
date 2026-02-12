use crate::{
    ast::{Program, PythonRoot},
    language::PythonLanguage,
};
use oak_core::{GreenNode, OakError, SourceText};

impl<'config> super::PythonBuilder<'config> {
    /// Builds a Python root from a green tree.
    pub(crate) fn build_root(&self, green_tree: &GreenNode<PythonLanguage>, source: &SourceText) -> Result<PythonRoot, OakError> {
        let statements = self.build_statement_list(green_tree.children(), 0, source)?;
        Ok(PythonRoot { program: Program { statements }, span: (0..green_tree.text_len() as usize).into() })
    }
}
