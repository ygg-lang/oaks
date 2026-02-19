use crate::{
    language::JavaLanguage,
    lexer::token_type::JavaTokenType,
    parser::{State, element_type::JavaElementType},
};
use oak_core::{
    OakError,
    parser::pratt::{Pratt, PrattParser},
    source::Source,
};

/// Declaration parsing implementation for Java
///
/// This module provides parsing for Java declarations including:
/// - Package declarations
/// - Import declarations
/// - Class, interface, enum, struct, record declarations
/// - Method and field declarations
/// - Annotations
/// - Type parameters
pub trait DeclarationParser: Pratt<JavaLanguage> {
    /// Skip trivia tokens (whitespace, comments)
    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        super::parse_expression::skip_trivia(state);
    }

    /// Parse a block statement
    fn parse_block_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError>
    where
        Self: Sized,
    {
        super::parse_statement::parse_block_statement(self, state)
    }

    /// Parse a declaration (class, interface, enum, method, field, etc.)
    fn parse_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError>
    where
        Self: Sized,
    {
        use JavaTokenType::*;
        let cp = state.checkpoint();
        self.skip_trivia(state);

        while state.at(At) {
            self.parse_annotation(state)?;
            self.skip_trivia(state);
        }

        while state.not_at_end() && matches!(state.peek_kind(), Some(Public) | Some(Private) | Some(Protected) | Some(Static) | Some(Final) | Some(Abstract)) {
            state.bump();
            self.skip_trivia(state)
        }

        match state.peek_kind() {
            Some(Class) => {
                state.expect(Class).ok();
                self.skip_trivia(state);
                state.expect(Identifier).ok();
                self.skip_trivia(state);
                self.parse_type_parameters(state)?;
                if state.eat(Extends) {
                    self.skip_trivia(state);
                    state.expect(Identifier).ok();
                    self.skip_trivia(state)
                }
                if state.eat(Implements) {
                    self.skip_trivia(state);
                    while state.not_at_end() && !state.at(LeftBrace) {
                        state.bump();
                        self.skip_trivia(state)
                    }
                }
                self.parse_block_statement(state)?;
                state.finish_at(cp, JavaElementType::ClassDeclaration);
            }
            Some(Interface) => {
                state.expect(Interface).ok();
                self.skip_trivia(state);
                state.expect(Identifier).ok();
                self.skip_trivia(state);
                self.parse_type_parameters(state)?;
                self.parse_block_statement(state)?;
                state.finish_at(cp, JavaElementType::InterfaceDeclaration);
            }
            Some(Enum) => {
                state.expect(Enum).ok();
                self.skip_trivia(state);
                state.expect(Identifier).ok();
                self.skip_trivia(state);
                self.parse_block_statement(state)?;
                state.finish_at(cp, JavaElementType::EnumDeclaration);
            }
            Some(Struct) => {
                state.expect(Struct).ok();
                self.skip_trivia(state);
                state.expect(Identifier).ok();
                self.skip_trivia(state);
                self.parse_block_statement(state)?;
                state.finish_at(cp, JavaElementType::StructDeclaration);
            }
            Some(Record) => {
                state.expect(Record).ok();
                self.skip_trivia(state);
                state.expect(Identifier).ok();
                self.skip_trivia(state);
                self.parse_block_statement(state)?;
                state.finish_at(cp, JavaElementType::RecordDeclaration);
            }
            Some(At) => {
                state.bump();
                state.expect(Interface).ok();
                self.skip_trivia(state);
                state.expect(Identifier).ok();
                self.skip_trivia(state);
                self.parse_block_statement(state)?;
                state.finish_at(cp, JavaElementType::AnnotationTypeDeclaration);
            }
            _ => {
                self.parse_type(state).ok();
                self.skip_trivia(state);
                let name_cp = state.checkpoint();
                state.expect(Identifier).ok();
                state.finish_at(name_cp, JavaElementType::Identifier);
                self.skip_trivia(state);

                if state.at(LeftAngle) {
                    self.parse_type_parameters(state)?;
                }

                if state.at(LeftParen) {
                    state.bump();
                    self.skip_trivia(state);
                    while state.not_at_end() && !state.at(RightParen) {
                        let p_cp = state.checkpoint();
                        self.parse_type(state).ok();
                        self.skip_trivia(state);
                        let pn_cp = state.checkpoint();
                        state.expect(Identifier).ok();
                        state.finish_at(pn_cp, JavaElementType::Identifier);
                        self.skip_trivia(state);
                        while state.at(LeftBracket) {
                            state.bump();
                            self.skip_trivia(state);
                            state.expect(RightBracket).ok();
                            self.skip_trivia(state)
                        }
                        state.finish_at(p_cp, JavaElementType::Parameter);
                        if !state.eat(Comma) {
                            break;
                        }
                        self.skip_trivia(state)
                    }
                    state.expect(RightParen).ok();
                    self.skip_trivia(state);
                    if state.eat(Throws) {
                        self.skip_trivia(state);
                        while state.not_at_end() && !state.at(LeftBrace) && !state.at(Semicolon) {
                            let t_cp = state.checkpoint();
                            state.expect(Identifier).ok();
                            state.finish_at(t_cp, JavaElementType::Identifier);
                            self.skip_trivia(state);
                            if !state.eat(Comma) {
                                break;
                            }
                            self.skip_trivia(state)
                        }
                    }
                    self.skip_trivia(state);
                    if state.at(LeftBrace) {
                        self.parse_block_statement(state)?
                    }
                    else {
                        state.eat(Semicolon);
                    }
                    state.finish_at(cp, JavaElementType::MethodDeclaration);
                }
                else {
                    if state.eat(Assign) {
                        self.skip_trivia(state);
                        PrattParser::parse(state, 0, self);
                    }
                    self.skip_trivia(state);
                    state.eat(Semicolon);
                    state.finish_at(cp, JavaElementType::FieldDeclaration);
                }
            }
        }
        Ok(())
    }

    /// Parse a package declaration
    fn parse_package_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use JavaTokenType::*;
        let cp = state.checkpoint();
        state.expect(Package).ok();
        self.skip_trivia(state);
        while state.not_at_end() && !state.at(Semicolon) {
            if state.at(Identifier) || state.at(Dot) {
                state.bump()
            }
            else {
                break;
            }
            self.skip_trivia(state)
        }
        state.eat(Semicolon);
        state.finish_at(cp, JavaElementType::Package);
        Ok(())
    }

    /// Parse an import declaration
    fn parse_import_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use JavaTokenType::*;
        let cp = state.checkpoint();
        state.expect(Import).ok();
        self.skip_trivia(state);
        if state.eat(Static) {
            self.skip_trivia(state)
        }
        while state.not_at_end() && !state.at(Semicolon) {
            if state.at(Identifier) || state.at(Dot) || state.at(Asterisk) {
                state.bump()
            }
            else {
                break;
            }
            self.skip_trivia(state)
        }
        state.eat(Semicolon);
        state.finish_at(cp, JavaElementType::Import);
        Ok(())
    }

    /// Parse an annotation
    fn parse_annotation<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError>
    where
        Self: Sized,
    {
        use JavaTokenType::*;
        let cp = state.checkpoint();
        state.expect(At).ok();
        self.skip_trivia(state);
        self.parse_type(state).ok();
        self.skip_trivia(state);
        if state.eat(LeftParen) {
            self.skip_trivia(state);
            while state.not_at_end() && !state.at(RightParen) {
                PrattParser::parse(state, 0, self);
                self.skip_trivia(state);
                if !state.eat(Comma) {
                    break;
                }
                self.skip_trivia(state);
            }
            state.expect(RightParen).ok();
        }
        state.finish_at(cp, JavaElementType::Annotation);
        Ok(())
    }

    /// Parse a type (primitive or reference type)
    fn parse_type<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use JavaTokenType::*;
        self.skip_trivia(state);
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(Identifier) | Some(Int) | Some(Boolean) | Some(Void) | Some(Long) | Some(Float) | Some(Double) | Some(Char) | Some(Byte) | Some(Short) => state.bump(),
            _ => {}
        }
        self.skip_trivia(state);
        while state.at(Dot) {
            state.bump();
            self.skip_trivia(state);
            state.expect(Identifier).ok();
            self.skip_trivia(state)
        }
        if state.at(LeftAngle) {
            state.bump();
            self.skip_trivia(state);
            while state.not_at_end() && !state.at(RightAngle) {
                self.parse_type(state)?;
                self.skip_trivia(state);
                if !state.eat(Comma) {
                    break;
                }
                self.skip_trivia(state);
            }
            state.expect(RightAngle).ok();
            self.skip_trivia(state);
        }
        while state.at(LeftBracket) {
            state.bump();
            self.skip_trivia(state);
            state.expect(RightBracket).ok();
            self.skip_trivia(state)
        }
        state.finish_at(cp, JavaElementType::Identifier);
        Ok(())
    }

    /// Parse type parameters for generics
    fn parse_type_parameters<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use JavaTokenType::*;
        self.skip_trivia(state);
        if state.at(LeftAngle) {
            state.bump();
            self.skip_trivia(state);
            while state.not_at_end() && !state.at(RightAngle) {
                let tp_cp = state.checkpoint();
                state.expect(Identifier).ok();
                self.skip_trivia(state);
                while state.at(Extends) {
                    state.bump();
                    self.skip_trivia(state);
                    self.parse_type(state)?;
                    self.skip_trivia(state);
                }
                state.finish_at(tp_cp, JavaElementType::TypeParameter);
                if !state.eat(Comma) {
                    break;
                }
                self.skip_trivia(state);
            }
            state.expect(RightAngle).ok();
            self.skip_trivia(state);
        }
        Ok(())
    }

    /// Parse a variable declaration
    fn parse_variable_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError>
    where
        Self: Sized,
    {
        use JavaTokenType::*;
        self.parse_type(state)?;
        self.skip_trivia(state);
        let cp = state.checkpoint();
        state.expect(Identifier).ok();
        state.finish_at(cp, JavaElementType::Identifier);
        self.skip_trivia(state);
        if state.eat(Assign) {
            self.skip_trivia(state);
            PrattParser::parse(state, 0, self);
        }
        self.skip_trivia(state);
        state.eat(Semicolon);
        Ok(())
    }
}
