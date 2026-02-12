/// The element type module for VHDL.
pub mod element_type;

use crate::{
    language::VhdlLanguage,
    lexer::{VhdlLexer, token_type::VhdlTokenType},
    parser::element_type::VhdlElementType,
};
use oak_core::{
    TextEdit,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::Source,
};

/// The parser state type for VHDL.
pub(crate) type State<'a, S> = ParserState<'a, VhdlLanguage, S>;

/// VHDL parser implementation.
pub struct VhdlParser<'config> {
    /// The VHDL language configuration.
    pub(crate) config: &'config VhdlLanguage,
}

impl<'config> VhdlParser<'config> {
    /// Creates a new `VhdlParser` with the given configuration.
    pub fn new(config: &'config VhdlLanguage) -> Self {
        Self { config }
    }

    fn parse_library_clause<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(VhdlTokenType::LibraryKw).ok();
        while state.not_at_end() && !state.at(VhdlTokenType::Semicolon) {
            state.advance();
        }
        state.eat(VhdlTokenType::Semicolon);
        state.finish_at(cp, VhdlElementType::LibraryClause);
    }

    fn parse_use_clause<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(VhdlTokenType::UseKw).ok();
        while state.not_at_end() && !state.at(VhdlTokenType::Semicolon) {
            state.advance();
        }
        state.eat(VhdlTokenType::Semicolon);
        state.finish_at(cp, VhdlElementType::UseClause);
    }

    fn parse_entity_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(VhdlTokenType::EntityKw).ok();
        state.expect(VhdlTokenType::Identifier).ok();
        state.expect(VhdlTokenType::IsKw).ok();

        while state.not_at_end() {
            if state.at(VhdlTokenType::PortKw) {
                self.parse_port_clause(state);
            }
            else if state.at(VhdlTokenType::GenericKw) {
                self.parse_generic_clause(state);
            }
            else if state.at(VhdlTokenType::EndKw) {
                state.advance();
                state.eat(VhdlTokenType::EntityKw);
                state.eat(VhdlTokenType::Identifier);
                state.eat(VhdlTokenType::Semicolon);
                break;
            }
            else {
                state.advance();
            }
        }

        state.finish_at(cp, VhdlElementType::EntityDeclaration);
    }

    fn parse_port_clause<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(VhdlTokenType::PortKw).ok();
        state.expect(VhdlTokenType::LeftParen).ok();

        while state.not_at_end() && !state.at(VhdlTokenType::RightParen) {
            self.parse_port_declaration(state);
            if state.at(VhdlTokenType::Semicolon) {
                state.bump();
            }
        }

        state.expect(VhdlTokenType::RightParen).ok();
        state.expect(VhdlTokenType::Semicolon).ok();
        state.finish_at(cp, VhdlElementType::PortClause);
    }

    fn parse_port_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(VhdlTokenType::Identifier).ok();
        state.expect(VhdlTokenType::Colon).ok();

        // direction
        if state.at(VhdlTokenType::InKw) || state.at(VhdlTokenType::OutKw) || state.at(VhdlTokenType::InoutKw) || state.at(VhdlTokenType::BufferKw) {
            state.bump();
        }

        // type
        while state.not_at_end() && !state.at(VhdlTokenType::Semicolon) && !state.at(VhdlTokenType::RightParen) {
            state.advance();
        }

        state.finish_at(cp, VhdlElementType::PortDeclaration);
    }

    fn parse_generic_clause<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(VhdlTokenType::GenericKw).ok();
        state.expect(VhdlTokenType::LeftParen).ok();

        while state.not_at_end() && !state.at(VhdlTokenType::RightParen) {
            state.advance();
        }

        state.expect(VhdlTokenType::RightParen).ok();
        state.expect(VhdlTokenType::Semicolon).ok();
        state.finish_at(cp, VhdlElementType::GenericClause);
    }

    fn parse_architecture_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(VhdlTokenType::ArchitectureKw).ok();
        state.expect(VhdlTokenType::Identifier).ok(); // architecture name
        state.expect(VhdlTokenType::OfKw).ok();
        state.expect(VhdlTokenType::Identifier).ok(); // entity name
        state.expect(VhdlTokenType::IsKw).ok();

        // Declarative part
        while state.not_at_end() && !state.at(VhdlTokenType::BeginKw) {
            if state.at(VhdlTokenType::SignalKw) {
                self.parse_signal_declaration(state);
            }
            else if state.at(VhdlTokenType::ComponentKw) {
                self.parse_component_declaration(state);
            }
            else {
                state.advance();
            }
        }

        if state.eat(VhdlTokenType::BeginKw) {
            // Statement part
            while state.not_at_end() && !state.at(VhdlTokenType::EndKw) {
                if state.at(VhdlTokenType::ProcessKw) {
                    self.parse_process_statement(state);
                }
                else {
                    state.advance();
                }
            }
        }

        if state.at(VhdlTokenType::EndKw) {
            state.bump();
            state.eat(VhdlTokenType::ArchitectureKw);
            state.eat(VhdlTokenType::Identifier);
            state.eat(VhdlTokenType::Semicolon);
        }

        state.finish_at(cp, VhdlElementType::ArchitectureBody);
    }

    fn parse_signal_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(VhdlTokenType::SignalKw).ok();
        state.expect(VhdlTokenType::Identifier).ok();
        state.expect(VhdlTokenType::Colon).ok();
        while state.not_at_end() && !state.at(VhdlTokenType::Semicolon) {
            state.advance();
        }
        state.eat(VhdlTokenType::Semicolon);
        state.finish_at(cp, VhdlElementType::SignalDeclaration);
    }

    fn parse_process_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(VhdlTokenType::ProcessKw).ok();

        // Optional sensitivity list
        if state.at(VhdlTokenType::LeftParen) {
            state.bump();
            while state.not_at_end() && !state.at(VhdlTokenType::RightParen) {
                state.advance();
            }
            state.eat(VhdlTokenType::RightParen);
        }

        state.eat(VhdlTokenType::IsKw);

        while state.not_at_end() && !state.at(VhdlTokenType::BeginKw) {
            state.advance();
        }

        if state.eat(VhdlTokenType::BeginKw) {
            while state.not_at_end() && !state.at(VhdlTokenType::EndKw) {
                state.advance();
            }
        }

        if state.at(VhdlTokenType::EndKw) {
            state.bump();
            state.expect(VhdlTokenType::ProcessKw).ok();
            state.eat(VhdlTokenType::Semicolon);
        }

        state.finish_at(cp, VhdlElementType::ProcessStatement);
    }

    fn parse_component_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(VhdlTokenType::ComponentKw).ok();
        state.expect(VhdlTokenType::Identifier).ok();
        state.expect(VhdlTokenType::IsKw).ok();

        while state.not_at_end() && !state.at(VhdlTokenType::EndKw) {
            if state.at(VhdlTokenType::PortKw) {
                self.parse_port_clause(state);
            }
            else if state.at(VhdlTokenType::GenericKw) {
                self.parse_generic_clause(state);
            }
            else {
                state.advance();
            }
        }

        if state.at(VhdlTokenType::EndKw) {
            state.bump();
            state.expect(VhdlTokenType::ComponentKw).ok();
            state.eat(VhdlTokenType::Semicolon);
        }

        state.finish_at(cp, VhdlElementType::ComponentDeclaration);
    }

    fn parse_package_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(VhdlTokenType::PackageKw).ok();
        if state.eat(VhdlTokenType::BodyKw) {
            state.expect(VhdlTokenType::Identifier).ok();
            state.expect(VhdlTokenType::IsKw).ok();
        }
        else {
            state.expect(VhdlTokenType::Identifier).ok();
            state.expect(VhdlTokenType::IsKw).ok();
        }

        while state.not_at_end() {
            if state.at(VhdlTokenType::EndKw) {
                state.advance();
                if state.at(VhdlTokenType::PackageKw) {
                    state.advance();
                    state.eat(VhdlTokenType::BodyKw);
                }
                state.eat(VhdlTokenType::Identifier);
                state.eat(VhdlTokenType::Semicolon);
                break;
            }
            state.advance();
        }

        state.finish_at(cp, VhdlElementType::PackageDeclaration);
    }
}

impl<'config> Parser<VhdlLanguage> for VhdlParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<VhdlLanguage>) -> ParseOutput<'a, VhdlLanguage> {
        let lexer = VhdlLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                if state.at(VhdlTokenType::LibraryKw) {
                    self.parse_library_clause(state);
                }
                else if state.at(VhdlTokenType::UseKw) {
                    self.parse_use_clause(state);
                }
                else if state.at(VhdlTokenType::EntityKw) {
                    self.parse_entity_declaration(state);
                }
                else if state.at(VhdlTokenType::ArchitectureKw) {
                    self.parse_architecture_body(state);
                }
                else if state.at(VhdlTokenType::PackageKw) {
                    self.parse_package_declaration(state);
                }
                else {
                    state.advance();
                }
            }

            Ok(state.finish_at(checkpoint, VhdlElementType::Root))
        })
    }
}
