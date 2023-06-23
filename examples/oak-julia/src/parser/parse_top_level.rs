use crate::{
    language::JuliaLanguage,
    lexer::token_type::JuliaTokenType,
    parser::{JuliaParser, State, element_type::JuliaElementType},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> JuliaParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, JuliaLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            self.skip_trivia(state);
            if !state.not_at_end() {
                break;
            }
            self.parse_statement(state)?;
        }

        Ok(state.finish_at(checkpoint, crate::parser::element_type::JuliaElementType::Root))
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.skip_trivia(state);
        if state.at(JuliaTokenType::Function) {
            self.parse_function(state)?;
        }
        else if state.at(JuliaTokenType::If) {
            self.parse_if(state)?;
        }
        else if state.at(JuliaTokenType::For) {
            self.parse_for(state)?;
        }
        else if state.at(JuliaTokenType::End) {
            // End should be handled by the caller (e.g. parse_function)
            // If it's here, it might be an error or we just consume it
            state.bump();
        }
        else {
            self.parse_expression(state)?;
        }
        Ok(())
    }

    fn parse_if<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // if
        self.skip_trivia(state);

        self.parse_expression(state)?;
        self.skip_trivia(state);

        // Parse then part
        while state.not_at_end() && !state.at(JuliaTokenType::Else) && !state.at(JuliaTokenType::ElseIf) && !state.at(JuliaTokenType::End) {
            self.parse_statement(state)?;
            self.skip_trivia(state);
        }

        if state.at(JuliaTokenType::Else) || state.at(JuliaTokenType::ElseIf) {
            state.bump();
            self.skip_trivia(state);
            while state.not_at_end() && !state.at(JuliaTokenType::End) {
                self.parse_statement(state)?;
                self.skip_trivia(state);
            }
        }

        if state.at(JuliaTokenType::End) {
            state.bump();
        }

        state.finish_at(cp, JuliaElementType::If);
        Ok(())
    }

    fn parse_for<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // for
        self.skip_trivia(state);

        if state.at(JuliaTokenType::Identifier) {
            state.bump();
        }
        self.skip_trivia(state);

        if state.at(JuliaTokenType::Equal) || state.at(JuliaTokenType::In) {
            state.bump();
        }
        self.skip_trivia(state);

        self.parse_expression(state)?;
        self.skip_trivia(state);

        // Parse body
        while state.not_at_end() && !state.at(JuliaTokenType::End) {
            self.parse_statement(state)?;
            self.skip_trivia(state);
        }

        if state.at(JuliaTokenType::End) {
            state.bump();
        }

        state.finish_at(cp, JuliaElementType::For);
        Ok(())
    }

    fn parse_function<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // function
        self.skip_trivia(state);

        if state.at(JuliaTokenType::Identifier) {
            let name_cp = state.checkpoint();
            state.bump(); // function name
            state.finish_at(name_cp, JuliaElementType::Identifier);
        }

        if state.at(JuliaTokenType::LeftParen) {
            state.bump();
            self.skip_trivia(state);
            // Parse arguments (ignored for now)
            while state.not_at_end() && !state.at(JuliaTokenType::RightParen) {
                state.bump();
            }
            if state.at(JuliaTokenType::RightParen) {
                state.bump();
            }
        }

        self.skip_trivia(state);

        // Parse body
        while state.not_at_end() && !state.at(JuliaTokenType::End) {
            self.parse_statement(state)?;
            self.skip_trivia(state);
        }

        if state.at(JuliaTokenType::End) {
            state.bump();
        }

        state.finish_at(cp, JuliaElementType::Function);
        Ok(())
    }

    fn parse_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.skip_trivia(state);
        let cp = state.checkpoint();

        if state.at(JuliaTokenType::Identifier) {
            state.bump();
            let id_node = state.finish_at(cp, JuliaElementType::Identifier);
            self.skip_trivia(state);
            if state.at(JuliaTokenType::LeftParen) {
                // Function call
                let call_cp = state.checkpoint_before(id_node);
                state.bump(); // (
                self.skip_trivia(state);
                let arg_cp = state.checkpoint();
                while state.not_at_end() && !state.at(JuliaTokenType::RightParen) {
                    self.parse_expression(state)?;
                    self.skip_trivia(state);
                    if state.at(JuliaTokenType::Comma) {
                        state.bump();
                        self.skip_trivia(state);
                    }
                }
                state.finish_at(arg_cp, JuliaElementType::ArgumentList);
                if state.at(JuliaTokenType::RightParen) {
                    state.bump();
                }
                state.finish_at(call_cp, JuliaElementType::Call);
            }
        }
        else if state.at(JuliaTokenType::StringLiteral) {
            state.bump();
            state.finish_at(cp, JuliaElementType::StringLiteral);
        }
        else if state.not_at_end() {
            state.bump();
        }

        Ok(())
    }
}
