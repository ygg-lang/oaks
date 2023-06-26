use crate::{language::VbNetLanguage, lexer::token_type::VbNetTokenType, parser::element_type::VbNetElementType};
use oak_core::{OakError, parser::ParserState, source::Source};

use super::{State, VbNetParser};

impl<'config> VbNetParser<'config> {
    pub(crate) fn parse_if<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(VbNetTokenType::If)?;

        self.parse_expression(state)?;
        state.expect(VbNetTokenType::Then)?;

        while state.not_at_end() && !state.at(VbNetTokenType::Else) && !state.at(VbNetTokenType::ElseIf) && !state.at(VbNetTokenType::End) {
            self.parse_statement(state)?;
        }

        if state.at(VbNetTokenType::Else) {
            state.bump();
            while state.not_at_end() && !state.at(VbNetTokenType::End) {
                self.parse_statement(state)?;
            }
        }
        else if state.at(VbNetTokenType::ElseIf) {
            state.bump();
            self.parse_if(state)?;
        }

        if state.at(VbNetTokenType::End) {
            state.bump();
            state.eat(VbNetTokenType::If);
        }

        state.finish_at(checkpoint, VbNetElementType::If);
        Ok(())
    }

    pub(crate) fn parse_for<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(VbNetTokenType::For)?;

        state.expect(VbNetTokenType::Identifier)?;

        if state.at(VbNetTokenType::As) {
            state.bump();
            while state.not_at_end() && (state.at(VbNetTokenType::Identifier) || state.at(VbNetTokenType::LeftParen)) {
                if state.at(VbNetTokenType::Identifier) {
                    state.bump();
                }
                else if state.at(VbNetTokenType::LeftParen) {
                    state.bump();
                    if state.at(VbNetTokenType::Of) {
                        state.bump();
                        while state.not_at_end() && (state.at(VbNetTokenType::Identifier) || state.at(VbNetTokenType::Dot) || state.at(VbNetTokenType::Comma)) {
                            if state.at(VbNetTokenType::Identifier) {
                                state.bump();
                            }
                            else if state.at(VbNetTokenType::Dot) {
                                state.bump();
                            }
                            else if state.at(VbNetTokenType::Comma) {
                                state.bump();
                            }
                        }
                    }
                    state.expect(VbNetTokenType::RightParen)?;
                }
                if state.at(VbNetTokenType::Dot) { state.bump() } else { break }
            }
        }

        state.expect(VbNetTokenType::Equal)?;

        self.parse_expression(state)?;
        state.expect(VbNetTokenType::To)?;

        self.parse_expression(state)?;

        if state.at(VbNetTokenType::Step) {
            state.bump();
            self.parse_expression(state)?;
        }

        while state.not_at_end() && !state.at(VbNetTokenType::Next) {
            self.parse_statement(state)?;
        }

        if state.at(VbNetTokenType::Next) {
            state.bump();
            state.eat(VbNetTokenType::Identifier);
        }

        state.finish_at(checkpoint, VbNetElementType::For);
        Ok(())
    }

    pub(crate) fn parse_foreach<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(VbNetTokenType::For)?;
        state.expect(VbNetTokenType::Each)?;

        state.expect(VbNetTokenType::Identifier)?;

        if state.at(VbNetTokenType::As) {
            state.bump();
            while state.not_at_end() && (state.at(VbNetTokenType::Identifier) || state.at(VbNetTokenType::LeftParen)) {
                if state.at(VbNetTokenType::Identifier) {
                    state.bump();
                }
                else if state.at(VbNetTokenType::LeftParen) {
                    state.bump();
                    if state.at(VbNetTokenType::Of) {
                        state.bump();
                        while state.not_at_end() && (state.at(VbNetTokenType::Identifier) || state.at(VbNetTokenType::Dot) || state.at(VbNetTokenType::Comma)) {
                            if state.at(VbNetTokenType::Identifier) {
                                state.bump();
                            }
                            else if state.at(VbNetTokenType::Dot) {
                                state.bump();
                            }
                            else if state.at(VbNetTokenType::Comma) {
                                state.bump();
                            }
                        }
                    }
                    state.expect(VbNetTokenType::RightParen)?;
                }
                if state.at(VbNetTokenType::Dot) { state.bump() } else { break }
            }
        }

        state.expect(VbNetTokenType::In)?;

        self.parse_expression(state)?;

        while state.not_at_end() && !state.at(VbNetTokenType::Next) {
            self.parse_statement(state)?;
        }

        if state.at(VbNetTokenType::Next) {
            state.bump();
            state.eat(VbNetTokenType::Identifier);
        }

        state.finish_at(checkpoint, VbNetElementType::ForEach);
        Ok(())
    }

    pub(crate) fn parse_while<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(VbNetTokenType::While)?;

        self.parse_expression(state)?;

        while state.not_at_end() && !state.at(VbNetTokenType::End) {
            self.parse_statement(state)?;
        }

        if state.at(VbNetTokenType::End) {
            state.bump();
            state.eat(VbNetTokenType::While);
        }

        state.finish_at(checkpoint, VbNetElementType::While);
        Ok(())
    }

    pub(crate) fn parse_do<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(VbNetTokenType::Do)?;

        let mut check_at_end = true;
        if state.at(VbNetTokenType::While) || state.at(VbNetTokenType::Until) {
            check_at_end = false;
            state.bump();
            self.parse_expression(state)?;
        }

        while state.not_at_end() && !state.at(VbNetTokenType::Loop) {
            self.parse_statement(state)?;
        }

        if state.at(VbNetTokenType::Loop) {
            state.bump();
            if check_at_end && (state.at(VbNetTokenType::While) || state.at(VbNetTokenType::Until)) {
                state.bump();
                self.parse_expression(state)?;
            }
        }

        state.finish_at(checkpoint, VbNetElementType::DoWhile);
        Ok(())
    }

    pub(crate) fn parse_select<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(VbNetTokenType::Select)?;
        state.expect(VbNetTokenType::Case)?;

        self.parse_expression(state)?;

        while state.not_at_end() && state.at(VbNetTokenType::Case) {
            state.bump();
            while state.not_at_end() && !state.at(VbNetTokenType::Case) && !state.at(VbNetTokenType::Default) && !state.at(VbNetTokenType::End) {
                self.parse_expression(state)?;
                if state.at(VbNetTokenType::Comma) { state.bump() } else { break }
            }
            while state.not_at_end() && !state.at(VbNetTokenType::Case) && !state.at(VbNetTokenType::Default) && !state.at(VbNetTokenType::End) {
                self.parse_statement(state)?;
            }
        }

        if state.at(VbNetTokenType::Default) {
            state.bump();
            while state.not_at_end() && !state.at(VbNetTokenType::End) {
                self.parse_statement(state)?;
            }
        }

        if state.at(VbNetTokenType::End) {
            state.bump();
            state.eat(VbNetTokenType::Select);
        }

        state.finish_at(checkpoint, VbNetElementType::SelectCase);
        Ok(())
    }

    pub(crate) fn parse_try<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(VbNetTokenType::Try)?;

        while state.not_at_end() && !state.at(VbNetTokenType::Catch) && !state.at(VbNetTokenType::Finally) && !state.at(VbNetTokenType::End) {
            self.parse_statement(state)?;
        }

        while state.not_at_end() && state.at(VbNetTokenType::Catch) {
            state.bump();
            if state.at(VbNetTokenType::Identifier) {
                state.bump();
                if state.at(VbNetTokenType::As) {
                    state.bump();
                    while state.not_at_end() && state.at(VbNetTokenType::Identifier) {
                        state.bump();
                        if state.at(VbNetTokenType::Dot) { state.bump() } else { break }
                    }
                }
            }
            while state.not_at_end() && !state.at(VbNetTokenType::Catch) && !state.at(VbNetTokenType::Finally) && !state.at(VbNetTokenType::End) {
                self.parse_statement(state)?;
            }
        }

        if state.at(VbNetTokenType::Finally) {
            state.bump();
            while state.not_at_end() && !state.at(VbNetTokenType::End) {
                self.parse_statement(state)?;
            }
        }

        if state.at(VbNetTokenType::End) {
            state.bump();
            state.eat(VbNetTokenType::Try);
        }

        state.finish_at(checkpoint, VbNetElementType::Try);
        Ok(())
    }

    pub(crate) fn parse_return<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(VbNetTokenType::Return)?;

        if !state.at(VbNetTokenType::End) && !state.at(VbNetTokenType::Newline) {
            self.parse_expression(state)?;
        }

        state.finish_at(checkpoint, VbNetElementType::Return);
        Ok(())
    }

    pub(crate) fn parse_with<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(VbNetTokenType::With)?;

        self.parse_expression(state)?;

        while state.not_at_end() && !state.at(VbNetTokenType::End) {
            self.parse_statement(state)?;
        }

        if state.at(VbNetTokenType::End) {
            state.bump();
            state.eat(VbNetTokenType::With);
        }

        state.finish_at(checkpoint, VbNetElementType::With);
        Ok(())
    }

    pub(crate) fn parse_exit<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(VbNetTokenType::Exit)?;

        if let Some(kind) = state.peek_kind() {
            match kind {
                VbNetTokenType::Sub | VbNetTokenType::Function | VbNetTokenType::For | VbNetTokenType::While | VbNetTokenType::Do | VbNetTokenType::Try => {
                    state.bump();
                }
                _ => {}
            }
        }

        state.finish_at(checkpoint, VbNetElementType::Exit);
        Ok(())
    }

    pub(crate) fn parse_continue<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(VbNetTokenType::Continue)?;

        if let Some(kind) = state.peek_kind() {
            match kind {
                VbNetTokenType::For | VbNetTokenType::While | VbNetTokenType::Do => {
                    state.bump();
                }
                _ => {}
            }
        }

        state.finish_at(checkpoint, VbNetElementType::Continue);
        Ok(())
    }

    pub(crate) fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.skip_trivia(state);

        let mut has_modifiers = false;
        while state.not_at_end() {
            match state.peek_kind() {
                Some(VbNetTokenType::Public)
                | Some(VbNetTokenType::Private)
                | Some(VbNetTokenType::Protected)
                | Some(VbNetTokenType::Friend)
                | Some(VbNetTokenType::ProtectedFriend)
                | Some(VbNetTokenType::Shared)
                | Some(VbNetTokenType::MustInherit)
                | Some(VbNetTokenType::NotInheritable)
                | Some(VbNetTokenType::MustOverride)
                | Some(VbNetTokenType::Overridable)
                | Some(VbNetTokenType::Overrides)
                | Some(VbNetTokenType::NotOverridable)
                | Some(VbNetTokenType::MustOverrideReadOnly)
                | Some(VbNetTokenType::ReadOnly)
                | Some(VbNetTokenType::WriteOnly)
                | Some(VbNetTokenType::Static)
                | Some(VbNetTokenType::Partial)
                | Some(VbNetTokenType::Async)
                | Some(VbNetTokenType::Await) => {
                    state.bump();
                    has_modifiers = true;
                    self.skip_trivia(state);
                }
                _ => break,
            }
        }

        let kind = state.peek_kind();
        match kind {
            Some(VbNetTokenType::Namespace) => self.parse_namespace(state)?,
            Some(VbNetTokenType::Imports) => self.parse_imports(state)?,
            Some(VbNetTokenType::Class) => self.parse_class(state)?,
            Some(VbNetTokenType::Interface) => self.parse_interface(state)?,
            Some(VbNetTokenType::Structure) => self.parse_structure(state)?,
            Some(VbNetTokenType::Enum) => self.parse_enum(state)?,
            Some(VbNetTokenType::Module) => self.parse_module(state)?,
            Some(VbNetTokenType::Function) => self.parse_function(state)?,
            Some(VbNetTokenType::Sub) => self.parse_sub(state)?,
            Some(VbNetTokenType::Property) => self.parse_property(state)?,
            Some(VbNetTokenType::Dim) => self.parse_dim(state)?,
            Some(VbNetTokenType::Const) => self.parse_const(state)?,
            Some(VbNetTokenType::If) => self.parse_if(state)?,
            Some(VbNetTokenType::For) => {
                state.bump();
                self.skip_trivia(state);
                if state.peek_kind() == Some(VbNetTokenType::Each) {
                    state.bump();
                    self.parse_foreach(state)?;
                }
                else {
                    self.parse_for(state)?;
                }
            }
            Some(VbNetTokenType::While) => self.parse_while(state)?,
            Some(VbNetTokenType::Do) => self.parse_do(state)?,
            Some(VbNetTokenType::Select) => self.parse_select(state)?,
            Some(VbNetTokenType::With) => self.parse_with(state)?,
            Some(VbNetTokenType::Try) => self.parse_try(state)?,
            Some(VbNetTokenType::Return) => self.parse_return(state)?,
            Some(VbNetTokenType::Exit) => self.parse_exit(state)?,
            Some(VbNetTokenType::Continue) => self.parse_continue(state)?,
            Some(VbNetTokenType::From) => self.parse_linq(state)?,
            _ => {
                self.skip_trivia(state);

                if !state.not_at_end() {
                    return Ok(());
                }

                self.parse_expression(state)?;
            }
        }
        Ok(())
    }
}
