use crate::{kind::CoqSyntaxKind, language::CoqLanguage, parser::CoqParser};
use oak_core::{GreenNode, OakError, parser::ParserState, source::Source};

type State<'a, S> = ParserState<'a, CoqLanguage, S>;

impl<'p> CoqParser<'p> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, CoqLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() && !state.at(CoqSyntaxKind::Eof) {
            if state.at(CoqSyntaxKind::Theorem) || state.at(CoqSyntaxKind::Lemma) || state.at(CoqSyntaxKind::Definition) {
                self.parse_declaration(state)?;
            }
            else if state.at(CoqSyntaxKind::Proof) {
                self.parse_proof(state)?;
            }
            else {
                state.bump();
            }
        }

        Ok(state.finish_at(checkpoint, CoqSyntaxKind::Root))
    }

    fn parse_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.bump(); // Theorem/Lemma/Definition
        state.expect(CoqSyntaxKind::Identifier)?;

        if state.at(CoqSyntaxKind::Colon) {
            state.expect(CoqSyntaxKind::Colon)?;
            self.parse_term(state)?;
        }

        if state.at(CoqSyntaxKind::Dot) {
            state.expect(CoqSyntaxKind::Dot)?;
        }

        state.finish_at(checkpoint, CoqSyntaxKind::Declaration);
        Ok(())
    }

    fn parse_proof<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(CoqSyntaxKind::Proof)?;
        if state.at(CoqSyntaxKind::Dot) {
            state.expect(CoqSyntaxKind::Dot)?;
        }

        while state.not_at_end() && !state.at(CoqSyntaxKind::Qed) && !state.at(CoqSyntaxKind::Admitted) {
            state.bump();
        }

        if state.at(CoqSyntaxKind::Qed) {
            state.expect(CoqSyntaxKind::Qed)?;
        }
        else {
            state.expect(CoqSyntaxKind::Admitted)?;
        }

        if state.at(CoqSyntaxKind::Dot) {
            state.expect(CoqSyntaxKind::Dot)?;
        }

        state.finish_at(checkpoint, CoqSyntaxKind::Statement);
        Ok(())
    }

    fn parse_term<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        while state.not_at_end() && !state.at(CoqSyntaxKind::Dot) && !state.at(CoqSyntaxKind::Semicolon) {
            state.bump();
        }
        state.finish_at(checkpoint, CoqSyntaxKind::Expression);
        Ok(())
    }
}
