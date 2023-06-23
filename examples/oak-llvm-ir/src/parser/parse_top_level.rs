use crate::{
    language::LLvmLanguage,
    lexer::token_type::LLvmTokenType,
    parser::{LLirParser, element_type::LLvmElementType},
};
use oak_core::{GreenNode, OakError, parser::ParserState};

impl<'config> LLirParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, LLvmLanguage, S>) -> Result<&'a GreenNode<'a, LLvmLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            self.parse_item(state)?;
        }

        Ok(state.finish_at(checkpoint, LLvmElementType::Root))
    }

    fn parse_item<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, LLvmLanguage, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();

        // Skip whitespace/newline/comments
        self.skip_trivia(state);

        if !state.not_at_end() {
            return Ok(());
        }

        if state.at(LLvmTokenType::GlobalVar) {
            self.parse_global(state)?;
        }
        else if state.at(LLvmTokenType::Keyword) && state.current().map_or(false, |t| state.source.get_text_in(t.span) == "define") {
            self.parse_function(state)?;
        }
        else {
            // Unknown item, just skip one token to avoid infinite loop
            state.advance();
        }

        state.finish_at(checkpoint, LLvmElementType::Item);
        Ok(())
    }

    fn parse_global<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, LLvmLanguage, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(LLvmTokenType::GlobalVar)?;
        self.skip_trivia(state);
        state.expect(LLvmTokenType::Equal)?;
        self.skip_trivia(state);

        // [linkage] [preemption_specifier] [visibility] [DLL_storage_class] [thread_local] [(unnamed_addr|local_unnamed_addr)] [externally_initialized]
        // (global|constant)
        while state.not_at_end() {
            if state.at(LLvmTokenType::Keyword) {
                let text = state.current().map(|t| state.source.get_text_in(t.span)).unwrap_or_else(|| "".into());
                if text == "global" || text == "constant" {
                    state.bump();
                    break;
                }
            }
            state.bump();
            self.skip_trivia(state);
        }

        self.skip_trivia(state);

        // type
        let type_checkpoint = state.checkpoint();
        while state.not_at_end() && !state.at(LLvmTokenType::Newline) && !state.at(LLvmTokenType::Comment) {
            state.bump();
        }
        state.finish_at(type_checkpoint, LLvmElementType::Identifier);

        state.finish_at(checkpoint, LLvmElementType::Global);
        Ok(())
    }

    fn parse_function<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, LLvmLanguage, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(LLvmTokenType::Keyword)?; // define
        self.skip_trivia(state);

        // return type
        let ret_type_checkpoint = state.checkpoint();
        while state.not_at_end() && !state.at(LLvmTokenType::GlobalVar) {
            state.bump();
            self.skip_trivia(state);
        }
        state.finish_at(ret_type_checkpoint, LLvmElementType::Identifier);

        state.expect(LLvmTokenType::GlobalVar)?;
        self.skip_trivia(state);

        // parameters
        if state.at(LLvmTokenType::LParen) {
            let params_checkpoint = state.checkpoint();
            state.bump();
            while state.not_at_end() && !state.at(LLvmTokenType::RParen) {
                let param_checkpoint = state.checkpoint();
                // simple parameter parsing
                while state.not_at_end() && !state.at(LLvmTokenType::Comma) && !state.at(LLvmTokenType::RParen) {
                    state.bump();
                }
                state.finish_at(param_checkpoint, LLvmElementType::Parameter);
                if state.at(LLvmTokenType::Comma) {
                    state.bump();
                    self.skip_trivia(state);
                }
            }
            state.expect(LLvmTokenType::RParen)?;
            state.finish_at(params_checkpoint, LLvmElementType::Parameter); // Group all params
        }

        self.skip_trivia(state);

        // body
        if state.at(LLvmTokenType::LBrace) {
            state.bump();
            self.skip_trivia(state);

            let mut block_checkpoint = state.checkpoint();
            let mut in_block = false;

            while state.not_at_end() && !state.at(LLvmTokenType::RBrace) {
                // Check if it's a label: identifier followed by colon
                let is_label = if state.at(LLvmTokenType::Keyword) { state.peek_at(1).map(|t| t.kind) == Some(LLvmTokenType::Colon) } else { false };

                if is_label {
                    if in_block {
                        state.finish_at(block_checkpoint, LLvmElementType::Block);
                    }
                    block_checkpoint = state.checkpoint();
                    state.bump(); // keyword
                    state.bump(); // colon
                    in_block = true;
                }
                else if state.at(LLvmTokenType::LocalVar) || state.at(LLvmTokenType::Keyword) {
                    if !in_block {
                        block_checkpoint = state.checkpoint();
                        in_block = true;
                    }
                    let inst_checkpoint = state.checkpoint();
                    while state.not_at_end() && !state.at(LLvmTokenType::Newline) && !state.at(LLvmTokenType::RBrace) {
                        state.bump();
                    }
                    state.finish_at(inst_checkpoint, LLvmElementType::Instruction);
                }
                else {
                    state.bump();
                }
                self.skip_trivia(state);
            }

            if in_block {
                state.finish_at(block_checkpoint, LLvmElementType::Block);
            }

            state.expect(LLvmTokenType::RBrace)?;
        }

        state.finish_at(checkpoint, LLvmElementType::Function);
        Ok(())
    }

    fn skip_trivia<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, LLvmLanguage, S>) {
        while state.not_at_end() && (state.at(LLvmTokenType::Whitespace) || state.at(LLvmTokenType::Newline) || state.at(LLvmTokenType::Comment)) {
            state.bump();
        }
    }
}
