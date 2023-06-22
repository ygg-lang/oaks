use crate::parser::{AsciiDocElementType, AsciiDocParser, State};
use oak_core::{OakError, source::Source, tree::GreenNode};

impl<'config> AsciiDocParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::language::AsciiDocLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.bump();
        }

        let node = state.finish_at(checkpoint, AsciiDocElementType::SourceFile.into());
        Ok(node)
    }
}
