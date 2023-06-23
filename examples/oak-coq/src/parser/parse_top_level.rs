use crate::{
    language::CoqLanguage,
    lexer::token_type::CoqTokenType,
    parser::{CoqElementType, CoqParser},
};
use oak_core::{GreenNode, OakError, parser::ParserState, source::Source};

type State<'a, S> = ParserState<'a, CoqLanguage, S>;

impl<'p> CoqParser<'p> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, CoqLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() && !state.at(CoqTokenType::Eof) {
            if state.at(CoqTokenType::Theorem) || state.at(CoqTokenType::Lemma) || state.at(CoqTokenType::Definition) {
                self.parse_declaration(state)?
            }
            else if state.at(CoqTokenType::Proof) {
                self.parse_proof(state)?
            }
            else {
                state.bump()
            }
        }

        Ok(state.finish_at(checkpoint, crate::parser::element_type::CoqElementType::Root))
    }

    fn parse_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.bump(); // Theorem/Lemma/Definition
        state.expect(CoqTokenType::Identifier)?;

        if state.at(CoqTokenType::Colon) {
            state.expect(CoqTokenType::Colon)?;
            self.parse_term(state)?
        }

        if state.at(CoqTokenType::Dot) {
            state.expect(CoqTokenType::Dot)?
        }

        state.finish_at(checkpoint, crate::parser::element_type::CoqElementType::Declaration);
        Ok(())
    }

    fn parse_proof<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(CoqTokenType::Proof)?;
        if state.at(CoqTokenType::Dot) {
            state.expect(CoqTokenType::Dot)?
        }

        while state.not_at_end() && !state.at(CoqTokenType::Qed) && !state.at(CoqTokenType::Admitted) {
            state.bump()
        }

        if state.at(CoqTokenType::Qed) {
            state.expect(CoqTokenType::Qed)?
        }
        else {
            state.expect(CoqTokenType::Admitted)?
        }

        if state.at(CoqTokenType::Dot) {
            state.expect(CoqTokenType::Dot)?
        }

        state.finish_at(checkpoint, crate::parser::element_type::CoqElementType::Statement);
        Ok(())
    }

    fn parse_term<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        while state.not_at_end() && !state.at(CoqTokenType::Dot) && !state.at(CoqTokenType::Semicolon) {
            state.bump()
        }
        state.finish_at(checkpoint, crate::parser::element_type::CoqElementType::Expression);
        Ok(())
    }
}
