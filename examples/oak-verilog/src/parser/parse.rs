use crate::{
    kind::VerilogKind,
    language::VerilogLanguage,
    parser::{State, VerilogParser},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> VerilogParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, VerilogLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance();
        }

        Ok(state.finish_at(checkpoint, VerilogKind::Module))
    }
}
