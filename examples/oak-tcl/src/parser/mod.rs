/// Element type definitions for the Tcl parser.
pub mod element_type;

use crate::{
    language::TclLanguage,
    lexer::{TclLexer, token_type::TclTokenType},
    parser::element_type::TclElementType,
};
use oak_core::{
    GreenNode, OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, TclLanguage, S>;

/// A parser for the Tcl programming language.
///
/// This parser processes tokens produced by the lexer and constructs
/// an abstract syntax tree (AST) for Tcl source code.
pub struct TclParser<'config> {
    pub(crate) config: &'config TclLanguage,
}

impl<'config> TclParser<'config> {
    /// Creates a new TclParser with the given configuration.
    pub fn new(config: &'config TclLanguage) -> Self {
        Self { config }
    }

    fn parse_command<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();

        // Peek the first word to see if it's a special command
        let first_word_text = state.peek_text().map(|s| s.to_string());
        let element_kind = match first_word_text.as_deref() {
            Some("proc") => TclElementType::ProcDefinition,
            Some("if") => TclElementType::IfCommand,
            Some("while") => TclElementType::WhileCommand,
            Some("for") => TclElementType::ForCommand,
            Some("foreach") => TclElementType::ForEachCommand,
            Some("set") => TclElementType::SetCommand,
            _ => TclElementType::Command,
        };

        while state.not_at_end() && !state.at(TclTokenType::Newline) && !state.at(TclTokenType::Semicolon) {
            if state.at(TclTokenType::Whitespace) {
                state.bump();
                continue;
            }

            if state.at(TclTokenType::Comment) {
                break;
            }

            self.parse_word(state)?;
        }

        state.finish_at(checkpoint, element_kind);
        Ok(())
    }

    fn parse_word<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();

        if state.at(TclTokenType::Dollar) {
            state.bump(); // $
            state.expect(TclTokenType::Identifier)?;
            state.finish_at(checkpoint, TclElementType::VariableWord);
        }
        else if state.at(TclTokenType::LeftBracket) {
            state.bump(); // [
            // Internal script
            while state.not_at_end() && !state.at(TclTokenType::RightBracket) {
                if state.at(TclTokenType::Newline) || state.at(TclTokenType::Semicolon) || state.at(TclTokenType::Whitespace) {
                    state.bump();
                    continue;
                }
                self.parse_command(state)?;
            }
            state.expect(TclTokenType::RightBracket)?;
            state.finish_at(checkpoint, TclElementType::ScriptWord);
        }
        else {
            // Simple word or StringLiteral or BracedString (all currently tokens)
            // If it's a StringLiteral, it might be braced or quoted.
            let kind = state.current().map(|t| t.kind);
            state.bump();

            let node_kind = match kind {
                Some(TclTokenType::StringLiteral) => TclTokenType::SimpleWord,
                _ => TclTokenType::SimpleWord,
            };
            state.finish_at(checkpoint, TclElementType::from(node_kind));
        }

        Ok(())
    }
}

impl<'config> Parser<TclLanguage> for TclParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<TclLanguage>) -> ParseOutput<'a, TclLanguage> {
        let lexer = TclLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                if state.at(TclTokenType::Newline) || state.at(TclTokenType::Semicolon) {
                    state.bump();
                    continue;
                }

                if state.at(TclTokenType::Whitespace) {
                    state.bump();
                    continue;
                }

                if state.at(TclTokenType::Comment) {
                    state.bump();
                    continue;
                }

                self.parse_command(state)?
            }

            Ok(state.finish_at(checkpoint, TclElementType::Root))
        })
    }
}
