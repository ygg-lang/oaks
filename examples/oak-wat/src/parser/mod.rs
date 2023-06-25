/// Element type definitions.
pub mod element_type;

use crate::{
    language::WatLanguage,
    lexer::{WatLexer, token_type::WatTokenType},
    parser::element_type::WatElementType,
};
use oak_core::{
    TextEdit,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::Source,
};

pub(crate) type State<'a, S> = ParserState<'a, WatLanguage, S>;

/// Parser for the WebAssembly Text (WAT) language.
pub struct WatParser<'config> {
    pub(crate) config: &'config WatLanguage,
}

impl<'config> WatParser<'config> {
    /// Creates a new instance of the WAT parser.
    pub fn new(config: &'config WatLanguage) -> Self {
        Self { config }
    }

    /// Parses a WAT item starting with a left parenthesis.
    fn parse_item<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(WatTokenType::LeftParen).ok();

        if state.at(WatTokenType::ModuleKw) {
            state.bump();
            while state.not_at_end() && !state.at(WatTokenType::RightParen) {
                if state.at(WatTokenType::LeftParen) {
                    self.parse_item(state);
                }
                else {
                    state.bump();
                }
            }
            state.expect(WatTokenType::RightParen).ok();
            state.finish_at(cp, WatElementType::Module);
        }
        else if state.at(WatTokenType::FuncKw) {
            state.bump();
            while state.not_at_end() && !state.at(WatTokenType::RightParen) {
                if state.at(WatTokenType::LeftParen) {
                    self.parse_item(state);
                }
                else {
                    state.bump();
                }
            }
            state.expect(WatTokenType::RightParen).ok();
            state.finish_at(cp, WatElementType::Func);
        }
        else if state.at(WatTokenType::ParamKw) {
            state.bump();
            while state.not_at_end() && !state.at(WatTokenType::RightParen) {
                state.bump();
            }
            state.expect(WatTokenType::RightParen).ok();
            state.finish_at(cp, WatElementType::Param);
        }
        else if state.at(WatTokenType::ResultKw) {
            state.bump();
            while state.not_at_end() && !state.at(WatTokenType::RightParen) {
                state.bump();
            }
            state.expect(WatTokenType::RightParen).ok();
            state.finish_at(cp, WatElementType::Result);
        }
        else if state.at(WatTokenType::LocalKw) {
            state.bump();
            while state.not_at_end() && !state.at(WatTokenType::RightParen) {
                state.bump();
            }
            state.expect(WatTokenType::RightParen).ok();
            state.finish_at(cp, WatElementType::Local);
        }
        else if state.at(WatTokenType::ExportKw) {
            state.bump();
            while state.not_at_end() && !state.at(WatTokenType::RightParen) {
                state.bump();
            }
            state.expect(WatTokenType::RightParen).ok();
            state.finish_at(cp, WatElementType::Export);
        }
        else if state.at(WatTokenType::ImportKw) {
            state.bump();
            while state.not_at_end() && !state.at(WatTokenType::RightParen) {
                state.bump();
            }
            state.expect(WatTokenType::RightParen).ok();
            state.finish_at(cp, WatElementType::Import);
        }
        else if state.at(WatTokenType::TypeKw) {
            state.bump();
            while state.not_at_end() && !state.at(WatTokenType::RightParen) {
                state.bump();
            }
            state.expect(WatTokenType::RightParen).ok();
            state.finish_at(cp, WatElementType::Type);
        }
        else if state.at(WatTokenType::TableKw) {
            state.bump();
            while state.not_at_end() && !state.at(WatTokenType::RightParen) {
                if state.at(WatTokenType::LeftParen) {
                    self.parse_item(state);
                }
                else {
                    state.bump();
                }
            }
            state.expect(WatTokenType::RightParen).ok();
            state.finish_at(cp, WatElementType::Table);
        }
        else if state.at(WatTokenType::MemoryKw) {
            state.bump();
            while state.not_at_end() && !state.at(WatTokenType::RightParen) {
                if state.at(WatTokenType::LeftParen) {
                    self.parse_item(state);
                }
                else {
                    state.bump();
                }
            }
            state.expect(WatTokenType::RightParen).ok();
            state.finish_at(cp, WatElementType::Memory);
        }
        else if state.at(WatTokenType::GlobalKw) {
            state.bump();
            while state.not_at_end() && !state.at(WatTokenType::RightParen) {
                state.bump();
            }
            state.expect(WatTokenType::RightParen).ok();
            state.finish_at(cp, WatElementType::Global);
        }
        else {
            // Check if it's an instruction with result (e.g. (i32.add (local.get 0) (local.get 1)))
            let is_instr = state.peek_kind().map_or(false, |k| {
                let kind_val = k as u8;
                kind_val >= WatTokenType::LocalGetKw as u8 && kind_val <= WatTokenType::I64RemUKw as u8
            });

            state.bump();
            while state.not_at_end() && !state.at(WatTokenType::RightParen) {
                if state.at(WatTokenType::LeftParen) {
                    self.parse_item(state);
                }
                else {
                    state.bump();
                }
            }
            state.expect(WatTokenType::RightParen).ok();

            if is_instr {
                state.finish_at(cp, WatElementType::Instruction);
            }
            else {
                state.finish_at(cp, WatElementType::Item);
            }
        }
    }
}

impl<'config> Parser<WatLanguage> for WatParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<WatLanguage>) -> ParseOutput<'a, WatLanguage> {
        let lexer = WatLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let cp = state.checkpoint();

            while state.not_at_end() {
                if state.at(WatTokenType::LeftParen) {
                    self.parse_item(state);
                }
                else {
                    state.bump();
                }
            }
            Ok(state.finish_at(cp, WatElementType::Root))
        })
    }
}
