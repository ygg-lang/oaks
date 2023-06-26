use oak_core::{
    builder::{BuildOutput, Builder as CoreBuilder, BuilderCache},
    source::{Source, TextEdit},
};

use crate::language::RacketLanguage;

/// AST builder for Racket source code.
pub struct Builder;

impl CoreBuilder<RacketLanguage> for Builder {
    fn build<'a, S: Source + ?Sized>(&self, _text: &S, _edits: &[TextEdit], _cache: &'a mut impl BuilderCache<RacketLanguage>) -> BuildOutput<RacketLanguage> {
        BuildOutput::<RacketLanguage>::new(Ok(()))
    }
}
