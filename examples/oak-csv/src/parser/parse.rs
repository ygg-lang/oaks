use crate::{
    language::CsvLanguage,
    lexer::CsvTokenType,
    parser::{CsvElementType, CsvParser, State},
};
use oak_core::{GreenNode, OakError};

impl<'config> CsvParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: oak_core::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, CsvLanguage>, OakError> {
        use crate::{lexer::CsvTokenType::*, parser::CsvElementType::*};
        let cp = state.checkpoint();

        while state.not_at_end() && !state.at(Eof) {
            self.parse_record(state)?;
            if state.at(Newline) {
                state.bump();
            }
        }

        Ok(state.finish_at(cp, SourceFile))
    }

    fn parse_record<'a, S: oak_core::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::{lexer::CsvTokenType::*, parser::CsvElementType::*};
        let cp = state.checkpoint();

        self.parse_field(state)?;
        while state.at(Comma) {
            state.bump();
            self.parse_field(state)?;
        }

        state.finish_at(cp, Record);
        Ok(())
    }

    fn parse_field<'a, S: oak_core::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        if state.at(CsvTokenType::Field) {
            state.bump();
        }
        else {
            // Empty field
        }
        state.finish_at(cp, CsvElementType::Field);
        Ok(())
    }
}
