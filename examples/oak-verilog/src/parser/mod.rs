#![doc = include_str!("readme.md")]

/// Verilog element types.
pub mod element_type;

pub use crate::lexer::token_type::VerilogKind as VerilogElementType;

use crate::{
    language::VerilogLanguage,
    lexer::{VerilogLexer, token_type::VerilogKind},
};
use oak_core::{
    GreenNode, OakError, TextEdit,
    parser::{ParseCache, Parser, ParserState, parse_with_lexer},
    source::Source,
};

pub(crate) type State<'a, S> = ParserState<'a, VerilogLanguage, S>;

/// Verilog parser implementation.
pub struct VerilogParser<'config> {
    pub(crate) config: &'config VerilogLanguage,
}

impl<'config> VerilogParser<'config> {
    /// Creates a new Verilog parser.
    pub fn new(config: &'config VerilogLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<VerilogLanguage> for VerilogParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<VerilogLanguage>) -> oak_core::ParseOutput<'a, VerilogLanguage> {
        let lexer = VerilogLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                self.skip_trivia(state);
                if state.at(VerilogKind::ModuleKw) {
                    self.parse_module(state);
                }
                else {
                    state.advance();
                }
                self.skip_trivia(state);
            }

            Ok(state.finish_at(checkpoint, VerilogKind::Root.into()))
        })
    }
}

impl<'config> VerilogParser<'config> {
    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while state.at(VerilogKind::Whitespace) || state.at(VerilogKind::Comment) {
            state.bump();
        }
    }

    fn parse_module<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(VerilogKind::ModuleKw).ok();
        self.skip_trivia(state);

        if state.at(VerilogKind::Identifier) {
            state.bump();
        }
        self.skip_trivia(state);

        // Port list
        if state.at(VerilogKind::LeftParen) {
            self.parse_port_list(state);
        }
        self.skip_trivia(state);

        if state.at(VerilogKind::Semicolon) {
            state.bump();
        }
        self.skip_trivia(state);

        // Module items
        while state.not_at_end() && !state.at(VerilogKind::EndmoduleKw) {
            self.parse_module_item(state);
            self.skip_trivia(state);
        }

        if state.at(VerilogKind::EndmoduleKw) {
            state.bump();
        }

        state.finish_at(cp, VerilogKind::Module.into());
    }

    fn parse_port_list<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(VerilogKind::LeftParen).ok();
        self.skip_trivia(state);

        while state.not_at_end() && !state.at(VerilogKind::RightParen) {
            self.parse_port(state);
            self.skip_trivia(state);
            if state.at(VerilogKind::Comma) {
                state.bump();
                self.skip_trivia(state);
            }
        }

        if state.at(VerilogKind::RightParen) {
            state.bump();
        }
        state.finish_at(cp, VerilogKind::PortList.into());
    }

    fn parse_port<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();

        if state.at(VerilogKind::InputKw) || state.at(VerilogKind::OutputKw) || state.at(VerilogKind::InoutKw) {
            state.bump();
            self.skip_trivia(state);
        }

        if state.at(VerilogKind::WireKw) || state.at(VerilogKind::RegKw) {
            state.bump();
            self.skip_trivia(state);
        }

        if state.at(VerilogKind::Identifier) {
            state.bump();
        }

        state.finish_at(cp, VerilogKind::Port.into());
    }

    fn parse_module_item<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();

        if state.at(VerilogKind::AssignKw) {
            self.parse_assign(state);
        }
        else if state.at(VerilogKind::AlwaysKw) {
            self.parse_always(state);
        }
        else if state.at(VerilogKind::InitialKw) {
            self.parse_initial(state);
        }
        else if state.at(VerilogKind::WireKw) || state.at(VerilogKind::RegKw) || state.at(VerilogKind::ParameterKw) {
            self.parse_declaration(state);
        }
        else {
            state.advance();
        }

        state.finish_at(cp, VerilogKind::ModuleItem.into());
    }

    fn parse_assign<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(VerilogKind::AssignKw).ok();
        self.skip_trivia(state);

        while state.not_at_end() && !state.at(VerilogKind::Semicolon) {
            state.bump();
        }

        if state.at(VerilogKind::Semicolon) {
            state.bump();
        }
        state.finish_at(cp, VerilogKind::Assign.into());
    }

    fn parse_always<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(VerilogKind::AlwaysKw).ok();
        self.skip_trivia(state);

        if state.at(VerilogKind::At) {
            state.bump();
            self.skip_trivia(state);
            if state.at(VerilogKind::LeftParen) {
                state.bump();
                while state.not_at_end() && !state.at(VerilogKind::RightParen) {
                    state.bump();
                }
                if state.at(VerilogKind::RightParen) {
                    state.bump();
                }
            }
        }
        self.skip_trivia(state);

        self.parse_statement(state);

        state.finish_at(cp, VerilogKind::Always.into());
    }

    fn parse_initial<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(VerilogKind::InitialKw).ok();
        self.skip_trivia(state);

        self.parse_statement(state);

        state.finish_at(cp, VerilogKind::Initial.into());
    }

    fn parse_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.bump(); // type (wire, reg, parameter)
        self.skip_trivia(state);

        // Optional range [msb:lsb]
        if state.at(VerilogKind::LeftBracket) {
            state.bump();
            while state.not_at_end() && !state.at(VerilogKind::RightBracket) {
                state.bump();
            }
            if state.at(VerilogKind::RightBracket) {
                state.bump();
            }
            self.skip_trivia(state);
        }

        while state.not_at_end() {
            if state.at(VerilogKind::Identifier) {
                state.bump();
            }
            self.skip_trivia(state);

            // Optional assignment
            if state.at(VerilogKind::AssignKw) || state.current().map_or(false, |t| state.source.get_text_in(t.span) == "=") {
                state.bump();
                self.skip_trivia(state);
                while state.not_at_end() && !state.at(VerilogKind::Comma) && !state.at(VerilogKind::Semicolon) {
                    state.bump();
                }
            }

            if state.at(VerilogKind::Comma) {
                state.bump();
                self.skip_trivia(state);
            }
            else {
                break;
            }
        }

        if state.at(VerilogKind::Semicolon) {
            state.bump();
        }
        state.finish_at(cp, VerilogKind::Declaration.into());
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();

        if state.at(VerilogKind::Identifier) && state.current().map_or(false, |t| state.source.get_text_in(t.span) == "begin") {
            // Block
            state.bump(); // begin
            self.skip_trivia(state);
            while state.not_at_end() && !state.current().map_or(false, |t| state.source.get_text_in(t.span) == "end") {
                self.parse_statement(state);
                self.skip_trivia(state);
            }
            if state.not_at_end() {
                state.bump(); // end
            }
            state.finish_at(cp, VerilogKind::Block.into());
        }
        else {
            // Simple statement
            while state.not_at_end()
                && !state.at(VerilogKind::Semicolon)
                && !state.current().map_or(false, |t| {
                    let text = state.source.get_text_in(t.span);
                    text == "end" || text == "endmodule"
                })
            {
                state.bump();
            }
            if state.at(VerilogKind::Semicolon) {
                state.bump();
            }
            state.finish_at(cp, VerilogKind::Statement.into());
        }
    }
}
