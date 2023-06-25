//! Parser implementation for the JASM language.

/// Element types for the JASM language.
pub mod element_type;

use crate::{
    language::JasmLanguage,
    lexer::{JasmLexer, token_type::JasmTokenType},
    parser::element_type::JasmElementType,
};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, JasmLanguage, S>;

/// Parser for the JASM language.
pub struct JasmParser<'config> {
    /// The language configuration.
    pub config: &'config JasmLanguage,
}

impl<'config> JasmParser<'config> {
    /// Creates a new `JasmParser`.
    pub fn new(config: &'config JasmLanguage) -> Self {
        Self { config }
    }

    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while state.not_at_end() && (state.at(JasmTokenType::Whitespace) || state.at(JasmTokenType::Newline) || state.at(JasmTokenType::Comment)) {
            state.bump();
        }
    }

    fn parse_class<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();

        // Parse modifiers
        while state.not_at_end()
            && matches!(
                state.current().map(|t| t.kind),
                Some(JasmTokenType::Public)
                    | Some(JasmTokenType::Private)
                    | Some(JasmTokenType::Protected)
                    | Some(JasmTokenType::Static)
                    | Some(JasmTokenType::Final)
                    | Some(JasmTokenType::Abstract)
                    | Some(JasmTokenType::Synthetic)
                    | Some(JasmTokenType::Deprecated)
            )
        {
            state.bump();
            self.skip_trivia(state);
        }

        state.expect(JasmTokenType::ClassKw).ok();
        self.skip_trivia(state);

        // Class name
        if state.at(JasmTokenType::Identifier) {
            state.bump();
        }
        self.skip_trivia(state);

        if state.eat(JasmTokenType::LeftBrace) {
            while state.not_at_end() && !state.at(JasmTokenType::RightBrace) {
                self.skip_trivia(state);
                if state.at(JasmTokenType::MethodKw)
                    || matches!(
                        state.current().map(|t| t.kind),
                        Some(JasmTokenType::Public)
                            | Some(JasmTokenType::Private)
                            | Some(JasmTokenType::Protected)
                            | Some(JasmTokenType::Static)
                            | Some(JasmTokenType::Final)
                            | Some(JasmTokenType::Abstract)
                            | Some(JasmTokenType::Synthetic)
                            | Some(JasmTokenType::Deprecated)
                    )
                {
                    // Check if it's a method or field by looking ahead or just trying both
                    // For JASM, both methods and fields can have modifiers.
                    // Usually method has MethodKw later, field has FieldKw.
                    // Simple heuristic: if we see MethodKw later, it's a method.
                    let mut lookahead = 0;
                    let mut is_method = false;
                    while let Some(t) = state.peek_at(lookahead) {
                        if t.kind == JasmTokenType::MethodKw {
                            is_method = true;
                            break;
                        }
                        if t.kind == JasmTokenType::Newline || t.kind == JasmTokenType::Semicolon || t.kind == JasmTokenType::LeftBrace {
                            break;
                        }
                        lookahead += 1;
                    }

                    if is_method {
                        self.parse_method(state);
                    }
                    else {
                        self.parse_field(state);
                    }
                }
                else if state.at(JasmTokenType::FieldKw) {
                    self.parse_field(state);
                }
                else if state.at(JasmTokenType::MethodKw) {
                    self.parse_method(state);
                }
                else {
                    state.advance();
                }
                self.skip_trivia(state);
            }
            state.eat(JasmTokenType::RightBrace);
        }

        state.finish_at(cp, JasmElementType::Class);
    }

    fn parse_field<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();

        // Parse modifiers
        while state.not_at_end()
            && matches!(
                state.current().map(|t| t.kind),
                Some(JasmTokenType::Public) | Some(JasmTokenType::Private) | Some(JasmTokenType::Protected) | Some(JasmTokenType::Static) | Some(JasmTokenType::Final) | Some(JasmTokenType::Synthetic) | Some(JasmTokenType::Deprecated)
            )
        {
            state.bump();
            self.skip_trivia(state);
        }

        state.expect(JasmTokenType::FieldKw).ok();
        self.skip_trivia(state);

        // Field name
        if state.at(JasmTokenType::Identifier) {
            state.bump();
        }
        self.skip_trivia(state);

        // Field descriptor
        if state.at(JasmTokenType::Identifier) || state.at(JasmTokenType::String) {
            state.bump();
        }

        while state.not_at_end() && !state.at(JasmTokenType::Newline) && !state.at(JasmTokenType::Semicolon) {
            state.bump();
        }
        state.eat(JasmTokenType::Semicolon);
        state.finish_at(cp, JasmElementType::Field);
    }

    fn parse_method<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();

        // Parse modifiers
        while state.not_at_end()
            && matches!(
                state.current().map(|t| t.kind),
                Some(JasmTokenType::Public)
                    | Some(JasmTokenType::Private)
                    | Some(JasmTokenType::Protected)
                    | Some(JasmTokenType::Static)
                    | Some(JasmTokenType::Final)
                    | Some(JasmTokenType::Abstract)
                    | Some(JasmTokenType::Native)
                    | Some(JasmTokenType::Synchronized)
                    | Some(JasmTokenType::Synthetic)
                    | Some(JasmTokenType::Deprecated)
                    | Some(JasmTokenType::Varargs)
            )
        {
            state.bump();
            self.skip_trivia(state);
        }

        state.expect(JasmTokenType::MethodKw).ok();
        self.skip_trivia(state);

        // Method name
        if state.at(JasmTokenType::Identifier) {
            state.bump();
        }
        self.skip_trivia(state);

        // Method descriptor
        if state.at(JasmTokenType::Identifier) || state.at(JasmTokenType::String) {
            state.bump();
        }
        self.skip_trivia(state);

        if state.eat(JasmTokenType::LeftBrace) {
            while state.not_at_end() && !state.at(JasmTokenType::RightBrace) {
                self.skip_trivia(state);
                if !state.not_at_end() || state.at(JasmTokenType::RightBrace) {
                    break;
                }

                if state.at(JasmTokenType::StackKw) || state.at(JasmTokenType::LocalsKw) {
                    state.bump();
                    self.skip_trivia(state);
                    if state.at(JasmTokenType::Number) {
                        state.bump();
                    }
                    continue;
                }

                let inst_cp = state.checkpoint();

                // Instructions or directives
                while state.not_at_end() && !state.at(JasmTokenType::Newline) && !state.at(JasmTokenType::RightBrace) {
                    state.bump();
                }

                state.finish_at(inst_cp, JasmElementType::Instruction);
                self.skip_trivia(state);
            }
            state.eat(JasmTokenType::RightBrace);
        }

        state.finish_at(cp, JasmElementType::Method);
    }
}

impl<'config> Parser<JasmLanguage> for JasmParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<JasmLanguage>) -> ParseOutput<'a, JasmLanguage> {
        let lexer = JasmLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                self.skip_trivia(state);
                if state.at(JasmTokenType::ClassKw) {
                    self.parse_class(state);
                }
                else {
                    state.advance();
                }
                self.skip_trivia(state);
            }

            Ok(state.finish_at(checkpoint, JasmElementType::Root))
        })
    }
}
