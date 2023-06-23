use crate::{
    language::WatLanguage,
    lexer::token_type::WatTokenType,
    parser::{State, WatParser, element_type::WatElementType},
};
use oak_core::{OakError, source::Source, tree::GreenNode};

impl<'config> WatParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, WatLanguage>, OakError> {
        let cp = state.checkpoint();
        state.bump(); // Start node implicitly via first token or manually if needed, but here we just process tokens

        while state.not_at_end() {
            if state.at(WatTokenType::LeftParen) { self.parse_item(state) } else { state.bump() }
        }
        Ok(state.finish_at(cp, crate::parser::element_type::WatElementType::Root))
    }

    fn parse_item<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(WatTokenType::LeftParen).ok();

        if state.at(WatTokenType::ModuleKw) {
            state.expect(WatTokenType::ModuleKw).ok();
            while state.not_at_end() && !state.at(WatTokenType::RightParen) {
                if state.at(WatTokenType::LeftParen) { self.parse_item(state) } else { state.bump() }
            }
            state.expect(WatTokenType::RightParen).ok();
            state.finish_at(cp, crate::parser::element_type::WatElementType::Module);
        }
        else {
            // Simplified: just skip other items for now
            while state.not_at_end() && !state.at(WatTokenType::RightParen) {
                state.bump()
            }
            state.expect(WatTokenType::RightParen).ok();
            state.finish_at(cp, crate::parser::element_type::WatElementType::Item);
        }
    }
}
