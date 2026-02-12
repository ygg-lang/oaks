/// Jasmin element type definition.
pub mod element_type;

use crate::{
    language::JasminLanguage,
    lexer::{JasminLexer, token_type::JasminTokenType},
    parser::element_type::JasminElementType,
};
use oak_core::{
    OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

/// Parser state for Jasmin.
pub(crate) type _State<'a, S> = ParserState<'a, JasminLanguage, S>;

/// Jasmin parser.
pub struct JasminParser<'config> {
    /// Language configuration.
    pub(crate) config: &'config JasminLanguage,
}

impl<'config> JasminParser<'config> {
    /// Creates a new `JasminParser` with the given configuration.
    pub fn new(config: &'config JasminLanguage) -> Self {
        Self { config }
    }

    /// Parses a top-level element.
    pub(crate) fn _parse_top_level<'a, S: Source + ?Sized>(&self, state: &mut _State<'a, S>) -> Result<(), OakError> {
        self.skip_trivia(state);
        if state.not_at_end() {
            state.bump();
        }
        Ok(())
    }

    /// Skips trivia (whitespace, newlines, comments).
    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut _State<'a, S>) {
        while let Some(kind) = state.peek_kind() {
            if matches!(kind, JasminTokenType::Whitespace | JasminTokenType::Newline | JasminTokenType::Comment) {
                state.bump();
            }
            else {
                break;
            }
        }
    }

    fn parse_class<'a, S: Source + ?Sized>(&self, state: &mut _State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(JasminTokenType::ClassKw).ok();
        self.skip_trivia(state);

        // Parse modifiers and class name
        while state.not_at_end() && !state.at(JasminTokenType::Newline) {
            match state.peek_kind() {
                Some(JasminTokenType::Public)
                | Some(JasminTokenType::Private)
                | Some(JasminTokenType::Protected)
                | Some(JasminTokenType::Static)
                | Some(JasminTokenType::Final)
                | Some(JasminTokenType::Abstract)
                | Some(JasminTokenType::Super)
                | Some(JasminTokenType::Synthetic)
                | Some(JasminTokenType::Deprecated) => {
                    state.bump();
                }
                Some(JasminTokenType::IdentifierToken) | Some(JasminTokenType::TypeDescriptor) => {
                    state.bump();
                }
                _ => {
                    state.bump();
                }
            }
            self.skip_trivia(state);
        }
        state.finish_at(cp, JasminElementType::Class);
    }

    fn parse_method<'a, S: Source + ?Sized>(&self, state: &mut _State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(JasminTokenType::MethodKw).ok();
        self.skip_trivia(state);

        // Parse modifiers, name and descriptor
        while state.not_at_end() && !state.at(JasminTokenType::Newline) {
            match state.peek_kind() {
                Some(JasminTokenType::Public)
                | Some(JasminTokenType::Private)
                | Some(JasminTokenType::Protected)
                | Some(JasminTokenType::Static)
                | Some(JasminTokenType::Final)
                | Some(JasminTokenType::Native)
                | Some(JasminTokenType::Synchronized)
                | Some(JasminTokenType::Abstract)
                | Some(JasminTokenType::Synthetic)
                | Some(JasminTokenType::Deprecated)
                | Some(JasminTokenType::Varargs) => {
                    state.bump();
                }
                Some(JasminTokenType::IdentifierToken) | Some(JasminTokenType::TypeDescriptor) => {
                    state.bump();
                }
                _ => {
                    state.bump();
                }
            }
            self.skip_trivia(state);
        }

        while state.not_at_end() {
            self.skip_trivia(state);
            if !state.not_at_end() {
                break;
            }

            if state.at(JasminTokenType::EndKw) {
                state.bump();
                self.skip_trivia(state);
                if state.at(JasminTokenType::MethodKw) {
                    state.bump();
                }
                break;
            }

            if state.at(JasminTokenType::StackKw) || state.at(JasminTokenType::LocalsKw) {
                state.bump();
                self.skip_trivia(state);
                while state.not_at_end() && !state.at(JasminTokenType::Newline) {
                    state.bump();
                }
            }
            else {
                // Parse instruction
                let inst_cp = state.checkpoint();
                while state.not_at_end() && !state.at(JasminTokenType::Newline) && !state.at(JasminTokenType::EndKw) {
                    state.bump();
                }
                state.finish_at(inst_cp, JasminElementType::Instruction);
            }
        }

        state.finish_at(cp, JasminElementType::Method);
    }

    fn parse_field<'a, S: Source + ?Sized>(&self, state: &mut _State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(JasminTokenType::FieldKw).ok();
        self.skip_trivia(state);

        // Parse modifiers, name and descriptor
        while state.not_at_end() && !state.at(JasminTokenType::Newline) {
            match state.peek_kind() {
                Some(JasminTokenType::Public) | Some(JasminTokenType::Private) | Some(JasminTokenType::Protected) | Some(JasminTokenType::Static) | Some(JasminTokenType::Final) | Some(JasminTokenType::Synthetic) | Some(JasminTokenType::Deprecated) => {
                    state.bump();
                }
                Some(JasminTokenType::IdentifierToken) | Some(JasminTokenType::TypeDescriptor) => {
                    state.bump();
                }
                _ => {
                    state.bump();
                }
            }
            self.skip_trivia(state);
        }
        state.finish_at(cp, JasminElementType::Field);
    }
}

impl<'config> Parser<JasminLanguage> for JasminParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<JasminLanguage>) -> ParseOutput<'a, JasminLanguage> {
        let lexer = JasminLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            while state.not_at_end() {
                self.skip_trivia(state);
                if !state.not_at_end() {
                    break;
                }

                if state.at(JasminTokenType::ClassKw) {
                    self.parse_class(state);
                }
                else if state.at(JasminTokenType::MethodKw) {
                    self.parse_method(state);
                }
                else if state.at(JasminTokenType::FieldKw) {
                    self.parse_field(state);
                }
                else {
                    state.bump();
                }
            }
            Ok(state.finish_at(checkpoint, JasminElementType::Root))
        })
    }
}
