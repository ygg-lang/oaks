use crate::{kind::PowerShellSyntaxKind, language::PowerShellLanguage};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};
pub struct PowerShellParser<'a> {
    _config: &'a PowerShellLanguage,
}

impl<'a> PowerShellParser<'a> {
    pub fn new(config: &'a PowerShellLanguage) -> Self {
        Self { _config: config }
    }

    fn parse_program<'b, S: Source + ?Sized>(&self, state: &mut ParserState<'b, PowerShellLanguage, S>) {
        while state.not_at_end() {
            if state.at(PowerShellSyntaxKind::Function) {
                self.parse_function(state);
            }
            else if state.at(PowerShellSyntaxKind::Class) {
                self.parse_class(state);
            }
            else if state.at(PowerShellSyntaxKind::If) {
                self.parse_if(state);
            }
            else {
                self.parse_statement(state);
            }
        }
    }

    fn parse_function<'b, S: Source + ?Sized>(&self, state: &mut ParserState<'b, PowerShellLanguage, S>) {
        let checkpoint = state.checkpoint();
        state.expect(PowerShellSyntaxKind::Function).ok();
        state.expect(PowerShellSyntaxKind::Identifier).ok();
        if state.at(PowerShellSyntaxKind::LeftBrace) {
            self.parse_block(state);
        }
        state.finish_at(checkpoint, PowerShellSyntaxKind::FunctionDef);
    }

    fn parse_class<'b, S: Source + ?Sized>(&self, state: &mut ParserState<'b, PowerShellLanguage, S>) {
        let checkpoint = state.checkpoint();
        state.expect(PowerShellSyntaxKind::Class).ok();
        state.expect(PowerShellSyntaxKind::Identifier).ok();
        if state.at(PowerShellSyntaxKind::LeftBrace) {
            self.parse_block(state);
        }
        state.finish_at(checkpoint, PowerShellSyntaxKind::ClassDef);
    }

    fn parse_if<'b, S: Source + ?Sized>(&self, state: &mut ParserState<'b, PowerShellLanguage, S>) {
        let checkpoint = state.checkpoint();
        state.expect(PowerShellSyntaxKind::If).ok();
        state.expect(PowerShellSyntaxKind::LeftParen).ok();
        while !state.at(PowerShellSyntaxKind::RightParen) && state.not_at_end() {
            state.bump();
        }
        state.expect(PowerShellSyntaxKind::RightParen).ok();
        self.parse_block(state);
        state.finish_at(checkpoint, PowerShellSyntaxKind::IfStatement);
    }

    fn parse_block<'b, S: Source + ?Sized>(&self, state: &mut ParserState<'b, PowerShellLanguage, S>) {
        state.expect(PowerShellSyntaxKind::LeftBrace).ok();
        while !state.at(PowerShellSyntaxKind::RightBrace) && state.not_at_end() {
            self.parse_statement(state);
        }
        state.expect(PowerShellSyntaxKind::RightBrace).ok();
    }

    fn parse_statement<'b, S: Source + ?Sized>(&self, state: &mut ParserState<'b, PowerShellLanguage, S>) {
        let checkpoint = state.checkpoint();
        while !state.at(PowerShellSyntaxKind::Semicolon) && !state.at(PowerShellSyntaxKind::Newline) && state.not_at_end() {
            state.bump();
        }
        if state.at(PowerShellSyntaxKind::Semicolon) || state.at(PowerShellSyntaxKind::Newline) {
            state.bump();
        }
        state.finish_at(checkpoint, PowerShellSyntaxKind::ExpressionStatement);
    }
}

impl<'a> Parser<PowerShellLanguage> for PowerShellParser<'a> {
    fn parse<'b, S: Source + ?Sized>(&self, text: &'b S, edits: &[TextEdit], cache: &'b mut impl ParseCache<PowerShellLanguage>) -> ParseOutput<'b, PowerShellLanguage> {
        let lexer = crate::lexer::PowerShellLexer::new(self._config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            self.parse_program(state);
            Ok(state.finish_at(checkpoint, PowerShellSyntaxKind::Root))
        })
    }
}
