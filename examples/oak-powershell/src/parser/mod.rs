pub mod element_type;

use crate::{language::PowerShellLanguage, lexer::token_type::PowerShellTokenType, parser::element_type::PowerShellElementType};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

#[derive(Debug)]
pub struct PowerShellParser<'a> {
    _config: &'a PowerShellLanguage,
}

impl<'a> PowerShellParser<'a> {
    pub fn new(config: &'a PowerShellLanguage) -> Self {
        Self { _config: config }
    }

    fn parse_program<'b, S: Source + ?Sized>(&self, state: &mut ParserState<'b, PowerShellLanguage, S>) {
        while state.not_at_end() {
            if state.at(PowerShellTokenType::Function) {
                self.parse_function(state);
            }
            else if state.at(PowerShellTokenType::Class) {
                self.parse_class(state);
            }
            else if state.at(PowerShellTokenType::If) {
                self.parse_if(state);
            }
            else {
                self.parse_statement(state);
            }
        }
    }

    fn parse_function<'b, S: Source + ?Sized>(&self, state: &mut ParserState<'b, PowerShellLanguage, S>) {
        let checkpoint = state.checkpoint();
        state.expect(PowerShellTokenType::Function).ok();
        state.expect(PowerShellTokenType::Identifier).ok();
        if state.at(PowerShellTokenType::LeftBrace) {
            self.parse_block(state);
        }
        state.finish_at(checkpoint, PowerShellElementType::FunctionDef);
    }

    fn parse_class<'b, S: Source + ?Sized>(&self, state: &mut ParserState<'b, PowerShellLanguage, S>) {
        let checkpoint = state.checkpoint();
        state.expect(PowerShellTokenType::Class).ok();
        state.expect(PowerShellTokenType::Identifier).ok();
        if state.at(PowerShellTokenType::LeftBrace) {
            self.parse_block(state);
        }
        state.finish_at(checkpoint, PowerShellElementType::ClassDef);
    }

    fn parse_if<'b, S: Source + ?Sized>(&self, state: &mut ParserState<'b, PowerShellLanguage, S>) {
        let checkpoint = state.checkpoint();
        state.expect(PowerShellTokenType::If).ok();
        state.expect(PowerShellTokenType::LeftParen).ok();
        while !state.at(PowerShellTokenType::RightParen) && state.not_at_end() {
            state.bump();
        }
        state.expect(PowerShellTokenType::RightParen).ok();
        self.parse_block(state);
        state.finish_at(checkpoint, PowerShellElementType::IfStatement);
    }

    fn parse_block<'b, S: Source + ?Sized>(&self, state: &mut ParserState<'b, PowerShellLanguage, S>) {
        state.expect(PowerShellTokenType::LeftBrace).ok();
        while !state.at(PowerShellTokenType::RightBrace) && state.not_at_end() {
            self.parse_statement(state);
        }
        state.expect(PowerShellTokenType::RightBrace).ok();
    }

    fn parse_statement<'b, S: Source + ?Sized>(&self, state: &mut ParserState<'b, PowerShellLanguage, S>) {
        let checkpoint = state.checkpoint();
        while !state.at(PowerShellTokenType::Semicolon) && !state.at(PowerShellTokenType::Newline) && state.not_at_end() {
            state.bump();
        }
        if state.at(PowerShellTokenType::Semicolon) || state.at(PowerShellTokenType::Newline) {
            state.bump();
        }
        state.finish_at(checkpoint, PowerShellElementType::ExpressionStatement);
    }
}

impl<'a> Parser<PowerShellLanguage> for PowerShellParser<'a> {
    fn parse<'b, S: Source + ?Sized>(&self, text: &'b S, edits: &[TextEdit], cache: &'b mut impl ParseCache<PowerShellLanguage>) -> ParseOutput<'b, PowerShellLanguage> {
        let lexer = crate::lexer::PowerShellLexer::new(self._config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            self.parse_program(state);
            Ok(state.finish_at(checkpoint, PowerShellElementType::Root))
        })
    }
}
