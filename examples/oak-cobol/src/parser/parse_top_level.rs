use crate::{
    lexer::CobolTokenType,
    parser::{CobolElementType, CobolParser, State},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> CobolParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::language::CobolLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            if state.at(CobolTokenType::Identification) {
                self.parse_identification_division(state)?;
            }
            else if state.at(CobolTokenType::Environment) {
                self.parse_environment_division(state)?;
            }
            else if state.at(CobolTokenType::Data) {
                self.parse_data_division(state)?;
            }
            else if state.at(CobolTokenType::Procedure) {
                self.parse_procedure_division(state)?;
            }
            else {
                state.bump();
            }
        }

        Ok(state.finish_at(checkpoint, CobolElementType::SourceFile))
    }

    fn parse_identification_division<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(CobolTokenType::Identification)?;
        state.expect(CobolTokenType::Division)?;
        // Optional period
        self.consume_period(state);

        while state.not_at_end() && !self.is_division_start(state) {
            if state.at(CobolTokenType::Program) {
                self.parse_program_id(state)?;
            }
            else {
                state.bump();
            }
        }

        state.finish_at(checkpoint, CobolElementType::IdentificationDivision);
        Ok(())
    }

    fn parse_program_id<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(CobolTokenType::Program)?;
        if state.at(CobolTokenType::Identifier) {
            state.bump();
        }
        self.consume_period(state);

        state.finish_at(checkpoint, CobolElementType::ProgramIdParagraph);
        Ok(())
    }

    fn parse_environment_division<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(CobolTokenType::Environment)?;
        state.expect(CobolTokenType::Division)?;
        self.consume_period(state);

        while state.not_at_end() && !self.is_division_start(state) {
            state.bump();
        }

        state.finish_at(checkpoint, CobolElementType::EnvironmentDivision);
        Ok(())
    }

    fn parse_data_division<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(CobolTokenType::Data)?;
        state.expect(CobolTokenType::Division)?;
        self.consume_period(state);

        while state.not_at_end() && !self.is_division_start(state) {
            if state.at(CobolTokenType::WorkingStorage) {
                self.parse_working_storage_section(state)?;
            }
            else {
                state.bump();
            }
        }

        state.finish_at(checkpoint, CobolElementType::DataDivision);
        Ok(())
    }

    fn parse_working_storage_section<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(CobolTokenType::WorkingStorage)?;
        state.expect(CobolTokenType::Section)?;
        self.consume_period(state);

        while state.not_at_end() && !self.is_division_start(state) && !state.at(CobolTokenType::FileSection) && !state.at(CobolTokenType::LinkageSection) {
            if state.at(CobolTokenType::NumberLiteral) {
                self.parse_data_item(state)?;
            }
            else {
                state.bump();
            }
        }

        state.finish_at(checkpoint, CobolElementType::WorkingStorageSection);
        Ok(())
    }

    fn parse_data_item<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(CobolTokenType::NumberLiteral)?; // Level
        state.expect(CobolTokenType::Identifier)?; // Name

        while state.not_at_end() && !state.at(CobolTokenType::Period) && !state.at(CobolTokenType::Newline) {
            if state.at(CobolTokenType::Pic) || state.at(CobolTokenType::Picture) {
                state.bump();
                state.expect(CobolTokenType::Identifier)?;
            }
            else if state.at(CobolTokenType::Value) {
                state.bump();
                state.bump(); // The value literal
            }
            else {
                state.bump();
            }
        }
        self.consume_period(state);

        state.finish_at(checkpoint, CobolElementType::DataItem);
        Ok(())
    }

    fn parse_procedure_division<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(CobolTokenType::Procedure)?;
        state.expect(CobolTokenType::Division)?;
        self.consume_period(state);

        while state.not_at_end() && !self.is_division_start(state) {
            if state.at(CobolTokenType::Display) {
                self.parse_display_statement(state)?;
            }
            else if state.at(CobolTokenType::Stop) {
                self.parse_stop_statement(state)?;
            }
            else if state.at(CobolTokenType::Move) {
                self.parse_move_statement(state)?;
            }
            else if state.at(CobolTokenType::Accept) {
                self.parse_accept_statement(state)?;
            }
            else if state.at(CobolTokenType::Add) {
                self.parse_add_statement(state)?;
            }
            else {
                state.bump();
            }
        }

        state.finish_at(checkpoint, CobolElementType::ProcedureDivision);
        Ok(())
    }

    fn parse_display_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(CobolTokenType::Display)?;

        while state.not_at_end() && !state.at(CobolTokenType::Newline) && !self.is_statement_end(state) {
            if state.at(CobolTokenType::StringLiteral) || state.at(CobolTokenType::NumberLiteral) || state.at(CobolTokenType::Identifier) {
                state.bump();
            }
            else {
                break;
            }
        }
        self.consume_period(state);

        state.finish_at(checkpoint, CobolElementType::DisplayStatement);
        Ok(())
    }

    fn parse_stop_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(CobolTokenType::Stop)?;
        if state.at(CobolTokenType::Identifier) && state.peek_text().map(|t| t == "RUN").unwrap_or(false) {
            state.bump();
        }
        self.consume_period(state);

        state.finish_at(checkpoint, CobolElementType::StopStatement);
        Ok(())
    }

    fn parse_move_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(CobolTokenType::Move)?;
        // Simplified MOVE literal TO identifier
        state.bump(); // Literal or Identifier
        if state.at(CobolTokenType::Identifier) && state.peek_text().map(|t| t == "TO").unwrap_or(false) {
            state.bump();
        }
        state.expect(CobolTokenType::Identifier)?;
        self.consume_period(state);

        state.finish_at(checkpoint, CobolElementType::MoveStatement);
        Ok(())
    }

    fn parse_accept_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(CobolTokenType::Accept)?;
        state.expect(CobolTokenType::Identifier)?;
        if state.at(CobolTokenType::Identifier) && state.peek_text().map(|t| t == "FROM").unwrap_or(false) {
            state.bump();
            state.expect(CobolTokenType::Identifier)?;
        }
        self.consume_period(state);

        state.finish_at(checkpoint, CobolElementType::AcceptStatement);
        Ok(())
    }

    fn parse_add_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(CobolTokenType::Add)?;
        while state.not_at_end() && (state.at(CobolTokenType::Identifier) || state.at(CobolTokenType::NumberLiteral)) {
            state.bump();
        }
        if state.at(CobolTokenType::Identifier) && state.peek_text().map(|t| t == "TO").unwrap_or(false) {
            state.bump();
            state.expect(CobolTokenType::Identifier)?;
        }
        self.consume_period(state);

        state.finish_at(checkpoint, CobolElementType::AddStatement);
        Ok(())
    }

    fn consume_period<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        if state.at(CobolTokenType::Period) {
            state.bump();
        }
    }

    fn is_division_start<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        state.at(CobolTokenType::Identification) || state.at(CobolTokenType::Environment) || state.at(CobolTokenType::Data) || state.at(CobolTokenType::Procedure)
    }

    fn is_statement_end<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        state.at(CobolTokenType::Display) || state.at(CobolTokenType::Stop) || state.at(CobolTokenType::Move) || state.at(CobolTokenType::Accept) || state.at(CobolTokenType::Add) || self.is_division_start(state)
    }
}
