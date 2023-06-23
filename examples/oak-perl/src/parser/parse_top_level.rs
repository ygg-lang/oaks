use crate::{
    PerlLanguage,
    parser::{PerlParser, State, element_type::PerlElementType},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> PerlParser<'config> {
    /// Internal method to parse the root of a Perl program.
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, PerlLanguage>, OakError> {
        let checkpoint = state.checkpoint();
        while state.not_at_end() {
            if self.parse_statement(state).is_err() {
                break;
            }
        }
        Ok(state.finish_at(checkpoint, PerlElementType::Root))
    }
}
