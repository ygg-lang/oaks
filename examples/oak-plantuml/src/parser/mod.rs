/// Element types and categories for the PlantUML language.
pub mod element_type;

use crate::{language::PlantUmlLanguage, lexer::token_type::PlantUmlTokenType, parser::element_type::PlantUmlElementType as ET};
use oak_core::{Parser, source::Source};

/// A parser for the PlantUML language.
pub struct PlantUmlParser<'config> {
    pub(crate) config: &'config PlantUmlLanguage,
}

impl<'config> PlantUmlParser<'config> {
    /// Creates a new parser with the given configuration.
    pub fn new(config: &'config PlantUmlLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<PlantUmlLanguage> for PlantUmlParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[oak_core::TextEdit], cache: &'a mut impl oak_core::ParseCache<PlantUmlLanguage>) -> oak_core::ParseOutput<'a, PlantUmlLanguage> {
        let lexer = crate::lexer::PlantUmlLexer::new(&self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                let item_checkpoint = state.checkpoint();
                if let Some(kind) = state.peek_kind() {
                    match kind {
                        PlantUmlTokenType::Comment => {
                            state.bump();
                            state.finish_at(item_checkpoint, ET::Comment);
                        }
                        PlantUmlTokenType::StartUml => {
                            state.bump();
                            state.finish_at(item_checkpoint, ET::StartUml);
                        }
                        PlantUmlTokenType::EndUml => {
                            state.bump();
                            state.finish_at(item_checkpoint, ET::EndUml);
                        }
                        PlantUmlTokenType::Class => {
                            state.bump();
                            // 解析类名
                            if let Some(PlantUmlTokenType::Id) = state.peek_kind() {
                                state.bump();
                            }
                            state.finish_at(item_checkpoint, ET::Class);
                        }
                        PlantUmlTokenType::Interface => {
                            state.bump();
                            // 解析接口名
                            if let Some(PlantUmlTokenType::Id) = state.peek_kind() {
                                state.bump();
                            }
                            state.finish_at(item_checkpoint, ET::Interface);
                        }
                        PlantUmlTokenType::Id => {
                            state.bump();
                            state.finish_at(item_checkpoint, ET::Id);
                        }
                        PlantUmlTokenType::Label => {
                            state.bump();
                            state.finish_at(item_checkpoint, ET::Label);
                        }
                        PlantUmlTokenType::Newline | PlantUmlTokenType::Whitespace => {
                            state.bump();
                        }
                        _ => {
                            state.bump();
                        }
                    }
                }
                else {
                    state.advance();
                }
            }

            let root = state.finish_at(checkpoint, ET::Root);
            Ok(root)
        })
    }
}
