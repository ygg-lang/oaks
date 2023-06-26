/// MSIL element type definition.
pub mod element_type;

use crate::{
    language::MsilLanguage,
    lexer::{MsilLexer, token_type::MsilTokenType},
};
use oak_core::{
    TextEdit,
    parser::{ParseCache, Parser, ParserState, parse_with_lexer},
    source::Source,
};

pub(crate) type State<'a, S> = ParserState<'a, MsilLanguage, S>;

/// MSIL language parser.
pub struct MsilParser<'config> {
    pub(crate) config: &'config MsilLanguage,
}

impl<'config> MsilParser<'config> {
    /// Creates a new MSIL parser.
    pub fn new(config: &'config MsilLanguage) -> Self {
        Self { config }
    }

    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while let Some(kind) = state.peek_kind() {
            if kind == MsilTokenType::Whitespace || kind == MsilTokenType::CommentToken {
                state.bump();
            }
            else {
                break;
            }
        }
    }

    fn parse_assembly<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.bump(); // .assembly
        self.skip_trivia(state);

        let is_extern = if state.at(MsilTokenType::ExternKeyword) {
            state.bump(); // extern
            self.skip_trivia(state);
            true
        }
        else {
            false
        };

        if state.at(MsilTokenType::IdentifierToken) {
            let id_cp = state.checkpoint();
            state.bump();
            state.finish_at(id_cp, crate::parser::element_type::MsilElementType::Identifier);
            self.skip_trivia(state);
        }

        if state.at(MsilTokenType::LeftBrace) {
            state.bump();
            while state.not_at_end() && !state.at(MsilTokenType::RightBrace) {
                state.bump();
            }
            if state.at(MsilTokenType::RightBrace) {
                state.bump();
            }
        }

        if is_extern {
            state.finish_at(cp, crate::parser::element_type::MsilElementType::AssemblyExtern);
        }
        else {
            state.finish_at(cp, crate::parser::element_type::MsilElementType::Assembly);
        }
    }

    fn parse_module<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.bump(); // .module
        self.skip_trivia(state);
        while state.not_at_end() && !state.at(MsilTokenType::Semicolon) && !state.at(MsilTokenType::Eof) {
            if state.at(MsilTokenType::IdentifierToken) {
                let id_cp = state.checkpoint();
                state.bump();
                state.finish_at(id_cp, crate::parser::element_type::MsilElementType::Identifier);
                self.skip_trivia(state);
            }
            else {
                state.bump();
            }
        }
        state.finish_at(cp, crate::parser::element_type::MsilElementType::Module);
    }

    fn parse_class<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.bump(); // .class
        self.skip_trivia(state);

        // Skip modifiers
        while state.at(MsilTokenType::PublicKeyword) || state.at(MsilTokenType::PrivateKeyword) || state.at(MsilTokenType::StaticKeyword) || state.at(MsilTokenType::Keyword) {
            state.bump();
            self.skip_trivia(state);
        }

        if state.at(MsilTokenType::IdentifierToken) {
            let id_cp = state.checkpoint();
            state.bump();
            state.finish_at(id_cp, crate::parser::element_type::MsilElementType::Identifier);
            self.skip_trivia(state);
        }

        // Handle extends
        if (state.at(MsilTokenType::IdentifierToken) || state.at(MsilTokenType::Keyword)) && state.peek_text().as_deref() == Some("extends") {
            state.bump();
            self.skip_trivia(state);
            while state.not_at_end() && !state.at(MsilTokenType::LeftBrace) {
                state.bump();
            }
        }

        if state.at(MsilTokenType::LeftBrace) {
            state.bump();
            while state.not_at_end() && !state.at(MsilTokenType::RightBrace) {
                self.skip_trivia(state);
                if !state.not_at_end() || state.at(MsilTokenType::RightBrace) {
                    break;
                }

                if state.at(MsilTokenType::MethodKeyword) {
                    self.parse_method(state);
                }
                else {
                    state.bump();
                }
            }
            if state.at(MsilTokenType::RightBrace) {
                state.bump();
            }
        }

        state.finish_at(cp, crate::parser::element_type::MsilElementType::Class);
    }

    fn parse_method<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.bump(); // .method
        self.skip_trivia(state);

        // Parse method modifiers
        while state.not_at_end() && !state.at(MsilTokenType::LeftBrace) {
            if state.at(MsilTokenType::PublicKeyword) || state.at(MsilTokenType::PrivateKeyword) || state.at(MsilTokenType::StaticKeyword) || state.at(MsilTokenType::Keyword) {
                state.bump();
                self.skip_trivia(state);
            }
            else if state.at(MsilTokenType::IdentifierToken) {
                let id_cp = state.checkpoint();
                state.bump();
                state.finish_at(id_cp, crate::parser::element_type::MsilElementType::Identifier);
                self.skip_trivia(state);
            }
            else if state.at(MsilTokenType::LeftParen) {
                // Parse method parameters
                state.bump();
                self.skip_trivia(state);
                while state.not_at_end() && !state.at(MsilTokenType::RightParen) {
                    if state.at(MsilTokenType::IdentifierToken) {
                        let id_cp = state.checkpoint();
                        state.bump();
                        state.finish_at(id_cp, crate::parser::element_type::MsilElementType::Identifier);
                        self.skip_trivia(state);
                    }
                    else if state.at(MsilTokenType::Comma) {
                        state.bump();
                        self.skip_trivia(state);
                    }
                    else {
                        state.bump();
                    }
                }
                if state.at(MsilTokenType::RightParen) {
                    state.bump();
                    self.skip_trivia(state);
                }
            }
            else {
                state.bump();
            }
        }

        if state.at(MsilTokenType::LeftBrace) {
            state.bump();
            while state.not_at_end() && !state.at(MsilTokenType::RightBrace) {
                self.skip_trivia(state);
                if !state.not_at_end() || state.at(MsilTokenType::RightBrace) {
                    break;
                }

                // Parse method body elements
                if state.at(MsilTokenType::IdentifierToken) {
                    let peeked = state.peek_text();
                    let text = peeked.as_deref().unwrap_or("");
                    if text.starts_with(".") {
                        // Parse directives like .maxstack, .locals, etc.
                        let dir_cp = state.checkpoint();
                        state.bump();
                        self.skip_trivia(state);
                        while state.not_at_end() && !state.at(MsilTokenType::Semicolon) && !state.at(MsilTokenType::LeftBrace) && !state.at(MsilTokenType::RightBrace) {
                            state.bump();
                        }
                        if state.at(MsilTokenType::Semicolon) {
                            state.bump();
                        }
                        state.finish_at(dir_cp, crate::parser::element_type::MsilElementType::Directive);
                    }
                    else if text.starts_with("IL_") {
                        // Parse instruction labels
                        let label_cp = state.checkpoint();
                        state.bump();
                        if state.at(MsilTokenType::Colon) {
                            state.bump();
                        }
                        state.finish_at(label_cp, crate::parser::element_type::MsilElementType::Label);
                    }
                    else {
                        // Parse instructions
                        let inst_cp = state.checkpoint();
                        state.bump();
                        self.skip_trivia(state);
                        // Parse instruction operands
                        while state.not_at_end() && !state.at(MsilTokenType::Semicolon) && !state.at(MsilTokenType::RightBrace) {
                            if state.at(MsilTokenType::IdentifierToken) || state.at(MsilTokenType::NumberToken) || state.at(MsilTokenType::StringToken) || state.at(MsilTokenType::LeftBracket) {
                                state.bump();
                            }
                            else {
                                break;
                            }
                            self.skip_trivia(state);
                        }
                        state.finish_at(inst_cp, crate::parser::element_type::MsilElementType::Instruction);
                    }
                }
                else {
                    state.bump();
                }
            }
            if state.at(MsilTokenType::RightBrace) {
                state.bump();
            }
        }

        state.finish_at(cp, crate::parser::element_type::MsilElementType::Method);
    }
}

impl<'config> Parser<MsilLanguage> for MsilParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<MsilLanguage>) -> oak_core::ParseOutput<'a, MsilLanguage> {
        let lexer = MsilLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let cp = state.checkpoint();
            while state.not_at_end() {
                self.skip_trivia(state);
                if !state.not_at_end() {
                    break;
                }

                if state.at(MsilTokenType::AssemblyKeyword) {
                    self.parse_assembly(state);
                }
                else if state.at(MsilTokenType::ModuleKeyword) {
                    self.parse_module(state);
                }
                else if state.at(MsilTokenType::ClassKeyword) {
                    self.parse_class(state);
                }
                else {
                    state.bump();
                }
            }
            Ok(state.finish_at(cp, crate::parser::element_type::MsilElementType::Root))
        })
    }
}
