use crate::parser::{CrystalParser, State};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> CrystalParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::language::CrystalLanguage>, OakError> {
        use crate::parser::CrystalElementType::*;
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.bump();
        }

        Ok(state.finish_at(checkpoint, SourceFile))
    }
}
