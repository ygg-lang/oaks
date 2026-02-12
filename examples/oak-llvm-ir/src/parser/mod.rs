//! Parser implementation for LLVM IR.

/// Element types for the LLVM IR parser.
pub mod element_type;

use crate::{
    language::LLvmLanguage,
    lexer::{LLvmLexer, token_type::LLvmTokenType},
    parser::element_type::LLvmElementType,
};
use oak_core::{
    OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, LLvmLanguage, S>;

/// Parser for LLVM IR.
pub struct LLirParser<'config> {
    pub(crate) config: &'config LLvmLanguage,
}

impl<'config> LLirParser<'config> {
    /// Creates a new LLVM IR parser with the given language configuration.
    pub fn new(config: &'config LLvmLanguage) -> Self {
        Self { config }
    }

    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while state.not_at_end() && (state.at(LLvmTokenType::Whitespace) || state.at(LLvmTokenType::Newline) || state.at(LLvmTokenType::Comment)) {
            state.bump();
        }
    }

    fn parse_type<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();

        // Basic types: i8, i16, i32, i64, float, double, void, etc.
        // Pointers: T*
        // Arrays: [N x T]
        // Structs: { T1, T2, ... }
        // Function pointers: T1 (T2, T3)*

        match state.peek_kind() {
            Some(LLvmTokenType::Keyword) | Some(LLvmTokenType::Identifier) => {
                state.bump();
            }
            Some(LLvmTokenType::Number) => {
                // Could be an integer type like i32 if the lexer doesn't distinguish
                state.bump();
            }
            Some(LLvmTokenType::LBracket) => {
                state.bump(); // [
                self.skip_trivia(state);
                // Parse array size
                if state.at(LLvmTokenType::Number) {
                    state.bump();
                    self.skip_trivia(state);
                }
                // Parse 'x'
                if state.at(LLvmTokenType::Identifier) || state.at(LLvmTokenType::Keyword) {
                    state.bump();
                    self.skip_trivia(state);
                }
                // Parse element type
                self.parse_type(state)?;
                self.skip_trivia(state);
                state.expect(LLvmTokenType::RBracket)?;
            }
            Some(LLvmTokenType::LBrace) => {
                state.bump(); // {
                self.skip_trivia(state);
                while state.not_at_end() && !state.at(LLvmTokenType::RBrace) {
                    self.parse_type(state)?;
                    self.skip_trivia(state);
                    if state.at(LLvmTokenType::Comma) {
                        state.bump();
                        self.skip_trivia(state);
                    }
                }
                state.expect(LLvmTokenType::RBrace)?;
            }
            Some(LLvmTokenType::LParen) => {
                // Function type: T1 (T2, T3)
                state.bump();
                self.skip_trivia(state);
                while state.not_at_end() && !state.at(LLvmTokenType::RParen) {
                    self.parse_type(state)?;
                    self.skip_trivia(state);
                    if state.at(LLvmTokenType::Comma) {
                        state.bump();
                        self.skip_trivia(state);
                    }
                }
                state.expect(LLvmTokenType::RParen)?;
            }
            _ => {
                state.advance();
            }
        }

        // Handle pointers (multiple levels possible)
        self.skip_trivia(state);
        while state.at(LLvmTokenType::Star) {
            state.bump();
            self.skip_trivia(state);
        }

        state.finish_at(checkpoint, LLvmElementType::Type);
        Ok(())
    }

    fn parse_item<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
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

    fn parse_global<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
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
        self.parse_type(state)?;

        state.finish_at(checkpoint, LLvmElementType::Global);
        Ok(())
    }

    fn parse_function<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(LLvmTokenType::Keyword)?; // define
        self.skip_trivia(state);

        // return type
        self.parse_type(state)?;
        self.skip_trivia(state);

        state.expect(LLvmTokenType::GlobalVar)?;
        self.skip_trivia(state);

        // parameters
        if state.at(LLvmTokenType::LParen) {
            let params_checkpoint = state.checkpoint();
            state.bump();
            while state.not_at_end() && !state.at(LLvmTokenType::RParen) {
                let param_checkpoint = state.checkpoint();
                // Parse type
                self.parse_type(state)?;
                self.skip_trivia(state);

                // Parse optional name
                if state.at(LLvmTokenType::LocalVar) {
                    state.bump();
                }

                state.finish_at(param_checkpoint, LLvmElementType::Parameter);

                self.skip_trivia(state);
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
                // Check if it's a label: identifier/localvar/number followed by colon
                let is_label = match state.peek_kind() {
                    Some(LLvmTokenType::Identifier) | Some(LLvmTokenType::LocalVar) | Some(LLvmTokenType::Number) | Some(LLvmTokenType::Keyword) => state.peek_at(1).map(|t| t.kind) == Some(LLvmTokenType::Colon),
                    _ => false,
                };

                if is_label {
                    if in_block {
                        state.finish_at(block_checkpoint, LLvmElementType::Block);
                    }
                    block_checkpoint = state.checkpoint();
                    state.bump(); // label
                    if state.at(LLvmTokenType::Colon) {
                        state.bump(); // colon
                    }
                    in_block = true;
                }
                else if state.at(LLvmTokenType::LocalVar) || state.at(LLvmTokenType::Keyword) || state.at(LLvmTokenType::Identifier) {
                    if !in_block {
                        block_checkpoint = state.checkpoint();
                        in_block = true;
                    }
                    let inst_checkpoint = state.checkpoint();

                    // Parse instruction structure: [result =] opcode operands
                    if state.at(LLvmTokenType::LocalVar) && state.peek_at(1).map(|t| t.kind) == Some(LLvmTokenType::Equal) {
                        state.bump(); // result
                        self.skip_trivia(state);
                        state.bump(); // =
                        self.skip_trivia(state);
                    }

                    if state.at(LLvmTokenType::Keyword) || state.at(LLvmTokenType::Identifier) {
                        state.bump(); // opcode
                    }

                    while state.not_at_end() && !state.at(LLvmTokenType::Newline) && !state.at(LLvmTokenType::RBrace) {
                        // If we see a colon, it might be a label, so we should stop instruction parsing
                        if state.peek_at(1).map(|t| t.kind) == Some(LLvmTokenType::Colon) {
                            let _next_text = state.current().map(|t| state.source.get_text_in(t.span)).unwrap_or_else(|| "".into());
                            // If the current token looks like a label name (identifier/number/localvar), stop
                            if matches!(state.peek_kind(), Some(LLvmTokenType::Identifier) | Some(LLvmTokenType::LocalVar) | Some(LLvmTokenType::Number) | Some(LLvmTokenType::Keyword)) {
                                break;
                            }
                        }
                        else {
                            // Instruction arguments
                            let arg_checkpoint = state.checkpoint();
                            if matches!(state.peek_kind(), Some(LLvmTokenType::Identifier) | Some(LLvmTokenType::LocalVar) | Some(LLvmTokenType::Number) | Some(LLvmTokenType::Keyword)) {
                                state.bump();
                                state.finish_at(arg_checkpoint, LLvmElementType::Operand);
                            }
                            else {
                                state.advance();
                                // state.done(LLvmElementType::Error);
                            }
                        }
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
}

impl<'config> Parser<LLvmLanguage> for LLirParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<LLvmLanguage>) -> ParseOutput<'a, LLvmLanguage> {
        let lexer = LLvmLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                self.parse_item(state)?;
            }

            Ok(state.finish_at(checkpoint, LLvmElementType::Root))
        })
    }
}
