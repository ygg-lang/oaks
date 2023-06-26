/// Element types and categories for the Structurizr language.
pub mod element_type;

use crate::{language::StructurizrLanguage, lexer::token_type::StructurizrTokenType, parser::element_type::StructurizrElementType as ET};
use oak_core::{Parser, ParserState, source::Source};

/// A parser for the Structurizr DSL.
pub struct StructurizrParser<'config> {
    pub(crate) config: &'config StructurizrLanguage,
}

impl<'config> StructurizrParser<'config> {
    /// Creates a new StructurizrParser with the given configuration.
    pub fn new(config: &'config StructurizrLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<StructurizrLanguage> for StructurizrParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[oak_core::TextEdit], cache: &'a mut impl oak_core::ParseCache<StructurizrLanguage>) -> oak_core::ParseOutput<'a, StructurizrLanguage> {
        let lexer = crate::lexer::StructurizrLexer::new(&self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                let item_checkpoint = state.checkpoint();
                if let Some(kind) = state.peek_kind() {
                    match kind {
                        StructurizrTokenType::Comment | StructurizrTokenType::Whitespace | StructurizrTokenType::Newline => {
                            state.bump();
                        }
                        StructurizrTokenType::Workspace => {
                            self.parse_workspace(state, item_checkpoint);
                        }
                        StructurizrTokenType::Model => {
                            self.parse_model(state, item_checkpoint);
                        }
                        StructurizrTokenType::Person => {
                            self.parse_person(state, item_checkpoint);
                        }
                        StructurizrTokenType::SoftwareSystem => {
                            self.parse_software_system(state, item_checkpoint);
                        }
                        StructurizrTokenType::Container => {
                            self.parse_container(state, item_checkpoint);
                        }
                        StructurizrTokenType::Component => {
                            self.parse_component(state, item_checkpoint);
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

impl<'config> StructurizrParser<'config> {
    fn parse_workspace<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, StructurizrLanguage, S>, checkpoint: (usize, usize)) {
        state.bump(); // Workspace

        // Parse workspace id
        if let Some(StructurizrTokenType::Id) = state.peek_kind() {
            state.bump();
        }

        // Parse workspace label
        if let Some(StructurizrTokenType::Label) = state.peek_kind() {
            state.bump();
        }

        // Parse workspace body
        if let Some(StructurizrTokenType::LeftBrace) = state.peek_kind() {
            state.bump();

            while state.not_at_end() {
                if let Some(StructurizrTokenType::RightBrace) = state.peek_kind() {
                    state.bump();
                    break;
                }
                else if let Some(kind) = state.peek_kind() {
                    match kind {
                        StructurizrTokenType::Comment | StructurizrTokenType::Whitespace | StructurizrTokenType::Newline => {
                            state.bump();
                        }
                        StructurizrTokenType::Model => {
                            let model_checkpoint = state.checkpoint();
                            state.bump(); // Model

                            // Parse model body
                            if let Some(StructurizrTokenType::LeftBrace) = state.peek_kind() {
                                state.bump();

                                while state.not_at_end() {
                                    if let Some(StructurizrTokenType::RightBrace) = state.peek_kind() {
                                        state.bump();
                                        break;
                                    }
                                    else if let Some(inner_kind) = state.peek_kind() {
                                        match inner_kind {
                                            StructurizrTokenType::Comment | StructurizrTokenType::Whitespace | StructurizrTokenType::Newline => {
                                                state.bump();
                                            }
                                            StructurizrTokenType::Person => {
                                                self.parse_person(state, state.checkpoint());
                                            }
                                            StructurizrTokenType::SoftwareSystem => {
                                                self.parse_software_system(state, state.checkpoint());
                                            }
                                            StructurizrTokenType::Container => {
                                                self.parse_container(state, state.checkpoint());
                                            }
                                            StructurizrTokenType::Component => {
                                                self.parse_component(state, state.checkpoint());
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
                            }

                            state.finish_at(model_checkpoint, ET::Model);
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
        }

        state.finish_at(checkpoint, ET::Workspace);
    }

    fn parse_model<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, StructurizrLanguage, S>, checkpoint: (usize, usize)) {
        state.bump(); // Model

        // Parse model body
        if let Some(StructurizrTokenType::LeftBrace) = state.peek_kind() {
            state.bump();

            while state.not_at_end() {
                if let Some(StructurizrTokenType::RightBrace) = state.peek_kind() {
                    state.bump();
                    break;
                }
                else if let Some(kind) = state.peek_kind() {
                    match kind {
                        StructurizrTokenType::Comment | StructurizrTokenType::Whitespace | StructurizrTokenType::Newline => {
                            state.bump();
                        }
                        StructurizrTokenType::Person => {
                            self.parse_person(state, state.checkpoint());
                        }
                        StructurizrTokenType::SoftwareSystem => {
                            self.parse_software_system(state, state.checkpoint());
                        }
                        StructurizrTokenType::Container => {
                            self.parse_container(state, state.checkpoint());
                        }
                        StructurizrTokenType::Component => {
                            self.parse_component(state, state.checkpoint());
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
        }

        state.finish_at(checkpoint, ET::Model);
    }

    fn parse_person<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, StructurizrLanguage, S>, checkpoint: (usize, usize)) {
        state.bump(); // Person

        // Parse person id
        if let Some(StructurizrTokenType::Id) = state.peek_kind() {
            state.bump();
        }

        // Parse person label
        if let Some(StructurizrTokenType::Label) = state.peek_kind() {
            state.bump();
        }

        // Parse person body
        if let Some(StructurizrTokenType::LeftBrace) = state.peek_kind() {
            state.bump();

            while state.not_at_end() {
                if let Some(StructurizrTokenType::RightBrace) = state.peek_kind() {
                    state.bump();
                    break;
                }
                else if let Some(kind) = state.peek_kind() {
                    match kind {
                        StructurizrTokenType::Comment | StructurizrTokenType::Whitespace | StructurizrTokenType::Newline => {
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
        }

        state.finish_at(checkpoint, ET::Person);
    }

    fn parse_software_system<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, StructurizrLanguage, S>, checkpoint: (usize, usize)) {
        state.bump(); // SoftwareSystem

        // Parse software system id
        if let Some(StructurizrTokenType::Id) = state.peek_kind() {
            state.bump();
        }

        // Parse software system label
        if let Some(StructurizrTokenType::Label) = state.peek_kind() {
            state.bump();
        }

        // Parse software system body
        if let Some(StructurizrTokenType::LeftBrace) = state.peek_kind() {
            state.bump();

            while state.not_at_end() {
                if let Some(StructurizrTokenType::RightBrace) = state.peek_kind() {
                    state.bump();
                    break;
                }
                else if let Some(kind) = state.peek_kind() {
                    match kind {
                        StructurizrTokenType::Comment | StructurizrTokenType::Whitespace | StructurizrTokenType::Newline => {
                            state.bump();
                        }
                        StructurizrTokenType::Container => {
                            self.parse_container(state, state.checkpoint());
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
        }

        state.finish_at(checkpoint, ET::SoftwareSystem);
    }

    fn parse_container<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, StructurizrLanguage, S>, checkpoint: (usize, usize)) {
        state.bump(); // Container

        // Parse container id
        if let Some(StructurizrTokenType::Id) = state.peek_kind() {
            state.bump();
        }

        // Parse container label
        if let Some(StructurizrTokenType::Label) = state.peek_kind() {
            state.bump();
        }

        // Parse container body
        if let Some(StructurizrTokenType::LeftBrace) = state.peek_kind() {
            state.bump();

            while state.not_at_end() {
                if let Some(StructurizrTokenType::RightBrace) = state.peek_kind() {
                    state.bump();
                    break;
                }
                else if let Some(kind) = state.peek_kind() {
                    match kind {
                        StructurizrTokenType::Comment | StructurizrTokenType::Whitespace | StructurizrTokenType::Newline => {
                            state.bump();
                        }
                        StructurizrTokenType::Component => {
                            self.parse_component(state, state.checkpoint());
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
        }

        state.finish_at(checkpoint, ET::Container);
    }

    fn parse_component<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, StructurizrLanguage, S>, checkpoint: (usize, usize)) {
        state.bump(); // Component

        // Parse component id
        if let Some(StructurizrTokenType::Id) = state.peek_kind() {
            state.bump();
        }

        // Parse component label
        if let Some(StructurizrTokenType::Label) = state.peek_kind() {
            state.bump();
        }

        // Parse component body
        if let Some(StructurizrTokenType::LeftBrace) = state.peek_kind() {
            state.bump();

            while state.not_at_end() {
                if let Some(StructurizrTokenType::RightBrace) = state.peek_kind() {
                    state.bump();
                    break;
                }
                else if let Some(kind) = state.peek_kind() {
                    match kind {
                        StructurizrTokenType::Comment | StructurizrTokenType::Whitespace | StructurizrTokenType::Newline => {
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
        }

        state.finish_at(checkpoint, ET::Component);
    }
}
