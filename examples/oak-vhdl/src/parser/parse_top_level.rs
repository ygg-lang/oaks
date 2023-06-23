use crate::{
    lexer::token_type::VhdlTokenType,
    parser::{State, VhdlParser, element_type::VhdlElementType},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> VhdlParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::language::VhdlLanguage>, OakError> {
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
            if state.at(VhdlTokenType::EndKw) {
                state.advance();
                state.eat(VhdlTokenType::EntityKw);
                state.eat(VhdlTokenType::Identifier);
                state.eat(VhdlTokenType::Semicolon);
                break;
            }
            state.advance();
        }

        state.finish_at(cp, VhdlElementType::EntityDeclaration);
    }

    fn parse_architecture_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(VhdlTokenType::ArchitectureKw).ok();
        state.expect(VhdlTokenType::Identifier).ok();
        state.expect(VhdlTokenType::OfKw).ok();
        state.expect(VhdlTokenType::Identifier).ok();
        state.expect(VhdlTokenType::IsKw).ok();

        while state.not_at_end() {
            if state.at(VhdlTokenType::EndKw) {
                state.advance();
                state.eat(VhdlTokenType::ArchitectureKw);
                state.eat(VhdlTokenType::Identifier);
                state.eat(VhdlTokenType::Semicolon);
                break;
            }
            state.advance();
        }

        state.finish_at(cp, VhdlElementType::ArchitectureBody);
    }

    fn parse_package_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(VhdlTokenType::PackageKw).ok();
        if state.eat(VhdlTokenType::BodyKw) {
            // Package body
            state.expect(VhdlTokenType::Identifier).ok();
            state.expect(VhdlTokenType::IsKw).ok();
            while state.not_at_end() {
                if state.at(VhdlTokenType::EndKw) {
                    state.advance();
                    state.eat(VhdlTokenType::PackageKw);
                    state.eat(VhdlTokenType::BodyKw);
                    state.eat(VhdlTokenType::Identifier);
                    state.eat(VhdlTokenType::Semicolon);
                    break;
                }
                state.advance();
            }
            state.finish_at(cp, VhdlElementType::PackageBody);
        }
        else {
            // Package declaration
            state.expect(VhdlTokenType::Identifier).ok();
            state.expect(VhdlTokenType::IsKw).ok();
            while state.not_at_end() {
                if state.at(VhdlTokenType::EndKw) {
                    state.advance();
                    state.eat(VhdlTokenType::PackageKw);
                    state.eat(VhdlTokenType::Identifier);
                    state.eat(VhdlTokenType::Semicolon);
                    break;
                }
                state.advance();
            }
            state.finish_at(cp, VhdlElementType::PackageDeclaration);
        }
    }
}
