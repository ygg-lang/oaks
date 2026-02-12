/// Element type module
pub mod element_type;

use crate::{
    language::FSharpLanguage,
    lexer::{FSharpLexer, token_type::FSharpTokenType},
    parser::element_type::FSharpElementType,
};
use oak_core::{
    GreenNode, OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, FSharpLanguage, S>;

/// F# parser
pub struct FSharpParser<'config> {
    pub(crate) config: &'config FSharpLanguage,
}

impl<'config> FSharpParser<'config> {
    /// Creates a new FSharpParser
    pub fn new(config: &'config FSharpLanguage) -> Self {
        Self { config }
    }

    fn parse_namespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(FSharpTokenType::Namespace)?;

        // Parse namespace name (e.g. System.Collections)
        while state.not_at_end() && state.at(FSharpTokenType::Identifier) {
            state.bump();
            if state.at(FSharpTokenType::Dot) { state.bump() } else { break }
        }

        state.finish_at(checkpoint, FSharpElementType::Namespace);
        Ok(())
    }

    fn parse_module<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(FSharpTokenType::Module)?;

        if state.at(FSharpTokenType::Identifier) {
            state.bump()
        }

        state.finish_at(checkpoint, FSharpElementType::Module);
        Ok(())
    }

    fn parse_open<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(FSharpTokenType::Open)?;

        // Parse namespace/module name to open
        while state.not_at_end() && state.at(FSharpTokenType::Identifier) {
            state.bump();
            if state.at(FSharpTokenType::Dot) { state.bump() } else { break }
        }

        state.finish_at(checkpoint, FSharpElementType::Open);
        Ok(())
    }

    fn parse_binding<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(FSharpTokenType::Let)?;

        if state.eat(FSharpTokenType::Rec) {
            // optional rec
        }

        // Name
        state.expect(FSharpTokenType::Identifier)?;

        // Parameters (optional)
        while state.not_at_end() && state.at(FSharpTokenType::Identifier) {
            state.bump()
        }

        // Equals
        state.expect(FSharpTokenType::Equal)?;

        // Expression
        self.parse_expression(state)?;

        state.finish_at(checkpoint, FSharpElementType::Let);
        Ok(())
    }

    fn parse_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        let kind = state.peek_kind();
        match kind {
            Some(FSharpTokenType::If) => {
                state.expect(FSharpTokenType::If)?;
                self.parse_expression(state)?;
                state.expect(FSharpTokenType::Then)?;
                self.parse_expression(state)?;
                if state.eat(FSharpTokenType::Else) {
                    self.parse_expression(state)?
                }
                state.finish_at(checkpoint, FSharpElementType::If);
            }
            _ => {
                // Simple expression: consume until end of line or specific delimiters
                while state.not_at_end() {
                    let kind = state.peek_kind();
                    if matches!(kind, Some(FSharpTokenType::Newline | FSharpTokenType::Then | FSharpTokenType::Else)) {
                        break;
                    }
                    state.bump()
                }
                // If we didn't consume anything, just bump one to avoid infinite loop
                if state.checkpoint().0 == checkpoint.0 {
                    state.bump()
                }
                state.finish_at(checkpoint, FSharpElementType::Expression);
            }
        }
        Ok(())
    }
}

impl<'config> Parser<FSharpLanguage> for FSharpParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<FSharpLanguage>) -> ParseOutput<'a, FSharpLanguage> {
        let lexer = FSharpLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let cp = (0, 0); // Ensure the root node includes initial trivia skipped during state initialization
            while state.not_at_end() {
                let kind = state.peek_kind();
                match kind {
                    Some(FSharpTokenType::Namespace) => self.parse_namespace(state)?,
                    Some(FSharpTokenType::Module) => self.parse_module(state)?,
                    Some(FSharpTokenType::Open) => self.parse_open(state)?,
                    Some(FSharpTokenType::Let) => self.parse_binding(state)?,
                    _ => state.bump(),
                }
            }

            Ok(state.finish_at(cp, FSharpElementType::Root))
        })
    }
}
