use crate::{ast::Program, language::PythonLanguage, parser::PythonParser};
use oak_core::{OakError, Parser, parser::session::ParseSession, source::Source};

/// Python language frontend.
pub struct PythonFrontend<'a, S: Source + ?Sized> {
    source: &'a S,
}

impl<'a, S: Source + ?Sized> PythonFrontend<'a, S> {
    /// Creates a new frontend instance.
    pub fn new(source: &'a S) -> Self {
        Self { source }
    }

    /// Parses Python source code into an AST.
    pub fn parse_to_ast(&self) -> Result<Program, OakError> {
        let config = PythonLanguage {};
        let parser = PythonParser::new(&config);
        let mut cache = ParseSession::<PythonLanguage>::default();

        let output = parser.parse(self.source, &[], &mut cache);

        match output.result {
            Ok(_green_node) => {
                // TODO: Implement GreenNode to AST conversion.
                // Currently returning an empty Program as PythonBuilder integration is pending.
                Ok(Program { statements: vec![] })
            }
            Err(e) => Err(e),
        }
    }
}
