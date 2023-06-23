use crate::{
    language::TailwindLanguage,
    lexer::token_type::TailwindTokenType,
    parser::{State, element_type::TailwindElementType},
};
use oak_core::{errors::OakError, tree::GreenNode};

impl<'config> super::TailwindParser<'config> {
    /// Parses the root of the Tailwind document.
    pub(crate) fn parse_root_internal<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, TailwindLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            if state.at(TailwindTokenType::DoubleLeftBrace) {
                self.parse_variable(state)?;
            }
            else if state.at(TailwindTokenType::LeftBracePercent) {
                self.parse_tag(state)?;
            }
            else {
                state.advance();
            }
        }

        Ok(state.finish_at(checkpoint, crate::parser::element_type::TailwindElementType::Root))
    }

    /// Parses a variable reference `{{ ... }}`.
    fn parse_variable<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(TailwindTokenType::DoubleLeftBrace)?;

        while state.not_at_end() && !state.at(TailwindTokenType::DoubleRightBrace) {
            state.advance();
        }

        state.expect(TailwindTokenType::DoubleRightBrace)?;
        state.finish_at(checkpoint, crate::parser::element_type::TailwindElementType::Variable);
        Ok(())
    }

    /// Parses a tag `{% ... %}`.
    fn parse_tag<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(TailwindTokenType::LeftBracePercent)?;

        while state.not_at_end() && !state.at(TailwindTokenType::PercentRightBrace) {
            state.advance();
        }

        state.expect(TailwindTokenType::PercentRightBrace)?;
        state.finish_at(checkpoint, crate::parser::element_type::TailwindElementType::Tag);
        Ok(())
    }
}
