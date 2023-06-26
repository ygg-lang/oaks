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

        // Error recovery: if no class keyword, skip until we find one or end of file
        if !state.at(JasmTokenType::ClassKw) {
            while state.not_at_end() && !state.at(JasmTokenType::ClassKw) {
                state.advance();
            }
        }

        state.expect(JasmTokenType::ClassKw).ok();
        self.skip_trivia(state);

        // Class name
        if state.at(JasmTokenType::Identifier) {
            state.bump();
        }
        else {
            // Error recovery: skip until identifier or next keyword
            while state.not_at_end() && !state.at(JasmTokenType::Identifier) && !state.at(JasmTokenType::ExtendsKw) && !state.at(JasmTokenType::ImplementsKw) && !state.at(JasmTokenType::LeftBrace) {
                state.advance();
            }
        }
        self.skip_trivia(state);

        // Parse extends clause
        if state.at(JasmTokenType::ExtendsKw) {
            state.bump();
            self.skip_trivia(state);
            if state.at(JasmTokenType::Identifier) {
                state.bump();
            }
            else {
                // Error recovery: skip until identifier or next keyword
                while state.not_at_end() && !state.at(JasmTokenType::Identifier) && !state.at(JasmTokenType::ImplementsKw) && !state.at(JasmTokenType::LeftBrace) {
                    state.advance();
                }
            }
            self.skip_trivia(state);
        }

        // Parse implements clause
        if state.at(JasmTokenType::ImplementsKw) {
            state.bump();
            self.skip_trivia(state);
            // Parse interface list
            while state.not_at_end() && !state.at(JasmTokenType::LeftBrace) {
                if state.at(JasmTokenType::Identifier) {
                    state.bump();
                    self.skip_trivia(state);
                    if state.at(JasmTokenType::Comma) {
                        state.bump();
                        self.skip_trivia(state);
                    }
                    else {
                        break;
                    }
                }
                else {
                    // Error recovery: skip until identifier or left brace
                    while state.not_at_end() && !state.at(JasmTokenType::Identifier) && !state.at(JasmTokenType::LeftBrace) {
                        state.advance();
                    }
                }
            }
        }

        if state.eat(JasmTokenType::LeftBrace) {
            while state.not_at_end() && !state.at(JasmTokenType::RightBrace) {
                self.skip_trivia(state);
                if state.not_at_end() && !state.at(JasmTokenType::RightBrace) {
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
                        // Error recovery: skip until next meaningful token
                        state.advance();
                    }
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

        // Error recovery: if no field keyword, skip until we find one or end of file
        if !state.at(JasmTokenType::FieldKw) {
            while state.not_at_end() && !state.at(JasmTokenType::FieldKw) && !state.at(JasmTokenType::MethodKw) && !state.at(JasmTokenType::RightBrace) {
                state.advance();
            }
        }

        state.expect(JasmTokenType::FieldKw).ok();
        self.skip_trivia(state);

        // Field name
        if state.at(JasmTokenType::Identifier) {
            state.bump();
        }
        else {
            // Error recovery: skip until identifier or descriptor
            while state.not_at_end() && !state.at(JasmTokenType::Identifier) && !state.at(JasmTokenType::String) && !state.at(JasmTokenType::Newline) && !state.at(JasmTokenType::Semicolon) {
                state.advance();
            }
        }
        self.skip_trivia(state);

        // Field descriptor
        if state.at(JasmTokenType::Identifier) || state.at(JasmTokenType::String) {
            state.bump();
        }

        // Skip until end of line or semicolon
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

        // Error recovery: if no method keyword, skip until we find one or end of file
        if !state.at(JasmTokenType::MethodKw) {
            while state.not_at_end() && !state.at(JasmTokenType::MethodKw) && !state.at(JasmTokenType::FieldKw) && !state.at(JasmTokenType::RightBrace) {
                state.advance();
            }
        }

        state.expect(JasmTokenType::MethodKw).ok();
        self.skip_trivia(state);

        // Method name
        if state.at(JasmTokenType::Identifier) {
            state.bump();
        }
        else {
            // Error recovery: skip until identifier or descriptor
            while state.not_at_end() && !state.at(JasmTokenType::Identifier) && !state.at(JasmTokenType::String) && !state.at(JasmTokenType::LeftBrace) {
                state.advance();
            }
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
                if state.at(JasmTokenType::At) {
                    state.bump();
                    self.skip_trivia(state);
                    // Parse annotation
                    if state.at(JasmTokenType::Identifier) {
                        state.bump();
                        // Parse annotation arguments
                        if state.at(JasmTokenType::LeftParen) {
                            state.bump();
                            self.skip_trivia(state);
                            // Parse arguments
                            while state.not_at_end() && !state.at(JasmTokenType::RightParen) {
                                if state.at(JasmTokenType::Identifier) || state.at(JasmTokenType::String) || state.at(JasmTokenType::Number) {
                                    state.bump();
                                }
                                else if state.at(JasmTokenType::Comma) {
                                    state.bump();
                                }
                                else {
                                    state.advance();
                                }
                                self.skip_trivia(state);
                            }
                            if state.at(JasmTokenType::RightParen) {
                                state.bump();
                            }
                        }
                    }
                }
                else if state.at(JasmTokenType::Dot) {
                    state.bump();
                    self.skip_trivia(state);
                    if state.at(JasmTokenType::SourceKw) {
                        state.bump();
                        self.skip_trivia(state);
                        // Parse source file path
                        if state.at(JasmTokenType::String) {
                            state.bump();
                        }
                        else if state.at(JasmTokenType::Identifier) {
                            state.bump();
                        }
                    }
                    else if state.at(JasmTokenType::SuperKw) {
                        state.bump();
                        self.skip_trivia(state);
                        // Parse super class
                        if state.at(JasmTokenType::Identifier) {
                            state.bump();
                        }
                    }
                    else if state.at(JasmTokenType::InterfaceKw) {
                        state.bump();
                        self.skip_trivia(state);
                        // Parse interface
                        if state.at(JasmTokenType::Identifier) {
                            state.bump();
                        }
                    }
                    else if state.at(JasmTokenType::CatchKw) {
                        state.bump();
                        self.skip_trivia(state);
                        // Parse exception handler
                        if state.at(JasmTokenType::Identifier) {
                            state.bump();
                        }
                        self.skip_trivia(state);
                        if state.at(JasmTokenType::Identifier) {
                            state.bump();
                        }
                    }
                    else if state.at(JasmTokenType::AttributeKw) {
                        state.bump();
                        self.skip_trivia(state);
                        // Parse attribute
                        if state.at(JasmTokenType::Identifier) {
                            state.bump();
                        }
                        self.skip_trivia(state);
                        if state.at(JasmTokenType::String) || state.at(JasmTokenType::Identifier) {
                            state.bump();
                        }
                    }
                    else if state.at(JasmTokenType::StackMapKw) {
                        state.bump();
                        self.skip_trivia(state);
                        // Parse stack map frame
                        while state.not_at_end() && !state.at(JasmTokenType::Newline) {
                            if state.at(JasmTokenType::Identifier) || state.at(JasmTokenType::Number) {
                                state.bump();
                            }
                            else {
                                state.advance();
                            }
                            self.skip_trivia(state);
                        }
                    }
                }
                else if state.at(JasmTokenType::ClassKw) {
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
