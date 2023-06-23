use crate::language::CSharpLanguage;
pub mod element_type;
pub use element_type::CSharpElementType;
use oak_core::{
    GreenNode, OakError,
    parser::{
        ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer,
        pratt::{Associativity, Pratt, PrattParser, binary},
    },
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, CSharpLanguage, S>;

pub struct CSharpParser<'config> {
    pub(crate) _language: &'config CSharpLanguage,
}

impl<'config> Pratt<CSharpLanguage> for CSharpParser<'config> {
    fn primary<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, CSharpLanguage, S>) -> &'a GreenNode<'a, CSharpLanguage> {
        use crate::lexer::token_type::CSharpTokenType;
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(CSharpTokenType::Identifier) => {
                state.bump();
                state.finish_at(cp, crate::parser::element_type::CSharpElementType::IdentifierName)
            }
            Some(CSharpTokenType::Number)
            | Some(CSharpTokenType::NumberLiteral)
            | Some(CSharpTokenType::String)
            | Some(CSharpTokenType::StringLiteral)
            | Some(CSharpTokenType::TrueKeyword)
            | Some(CSharpTokenType::FalseKeyword)
            | Some(CSharpTokenType::NullKeyword) => {
                state.bump();
                state.finish_at(cp, crate::parser::element_type::CSharpElementType::LiteralExpression)
            }
            Some(CSharpTokenType::LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(CSharpTokenType::RightParen).ok();
                state.finish_at(cp, crate::parser::element_type::CSharpElementType::BinaryExpression) // 简化处理
            }
            _ => {
                state.bump();
                state.finish_at(cp, crate::parser::element_type::CSharpElementType::Root)
            }
        }
    }

    fn prefix<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, CSharpLanguage, S>) -> &'a GreenNode<'a, CSharpLanguage> {
        self.primary(state)
    }

    fn infix<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, CSharpLanguage, S>, left: &'a GreenNode<'a, CSharpLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, CSharpLanguage>> {
        use crate::{lexer::token_type::CSharpTokenType, parser::CSharpElementType::*};
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            CSharpTokenType::Assign
            | CSharpTokenType::PlusAssign
            | CSharpTokenType::MinusAssign
            | CSharpTokenType::StarAssign
            | CSharpTokenType::SlashAssign
            | CSharpTokenType::PercentAssign
            | CSharpTokenType::AndAssign
            | CSharpTokenType::OrAssign
            | CSharpTokenType::XorAssign
            | CSharpTokenType::LeftShiftAssign
            | CSharpTokenType::RightShiftAssign
            | CSharpTokenType::QuestionQuestionAssign => (1, Associativity::Right),
            CSharpTokenType::LogicalOr => (2, Associativity::Left),
            CSharpTokenType::LogicalAnd => (3, Associativity::Left),
            CSharpTokenType::Equal | CSharpTokenType::NotEqual | CSharpTokenType::Less | CSharpTokenType::Greater | CSharpTokenType::LessEqual | CSharpTokenType::GreaterEqual | CSharpTokenType::IsKeyword | CSharpTokenType::AsKeyword => {
                (4, Associativity::Left)
            }
            CSharpTokenType::Plus | CSharpTokenType::Minus => (10, Associativity::Left),
            CSharpTokenType::Star | CSharpTokenType::Slash | CSharpTokenType::Percent => (11, Associativity::Left),
            CSharpTokenType::LeftParen | CSharpTokenType::LeftBracket | CSharpTokenType::Dot => (15, Associativity::Left),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        match kind {
            CSharpTokenType::LeftParen => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(CSharpTokenType::LeftParen).ok();
                while state.not_at_end() && !state.at(CSharpTokenType::RightParen) {
                    state.bump();
                }
                state.expect(CSharpTokenType::RightParen).ok();
                Some(state.finish_at(cp, crate::parser::element_type::CSharpElementType::InvocationExpression))
            }
            CSharpTokenType::LeftBracket => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(CSharpTokenType::LeftBracket).ok();
                while state.not_at_end() && !state.at(CSharpTokenType::RightBracket) {
                    state.bump();
                }
                state.expect(CSharpTokenType::RightBracket).ok();
                Some(state.finish_at(cp, crate::parser::element_type::CSharpElementType::ElementAccessExpression))
            }
            CSharpTokenType::Dot => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(CSharpTokenType::Dot).ok();
                state.expect(CSharpTokenType::Identifier).ok();
                Some(state.finish_at(cp, crate::parser::element_type::CSharpElementType::MemberAccessExpression))
            }
            CSharpTokenType::Assign
            | CSharpTokenType::PlusAssign
            | CSharpTokenType::MinusAssign
            | CSharpTokenType::StarAssign
            | CSharpTokenType::SlashAssign
            | CSharpTokenType::PercentAssign
            | CSharpTokenType::AndAssign
            | CSharpTokenType::OrAssign
            | CSharpTokenType::XorAssign
            | CSharpTokenType::LeftShiftAssign
            | CSharpTokenType::RightShiftAssign
            | CSharpTokenType::QuestionQuestionAssign => Some(binary(state, left, kind, prec, assoc, AssignmentExpression, |s, p| PrattParser::parse(s, p, self))),
            _ => Some(binary(state, left, kind, prec, assoc, BinaryExpression, |s, p| PrattParser::parse(s, p, self))),
        }
    }
}

impl<'config> CSharpParser<'config> {
    pub fn new(language: &'config CSharpLanguage) -> Self {
        Self { _language: language }
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::{lexer::token_type::CSharpTokenType, parser::CSharpElementType::*};
        match state.peek_kind() {
            Some(CSharpTokenType::Namespace) => self.parse_namespace_declaration(state)?,
            Some(CSharpTokenType::Using) => self.parse_using_directive(state)?,
            Some(CSharpTokenType::Class)
            | Some(CSharpTokenType::Struct)
            | Some(CSharpTokenType::Interface)
            | Some(CSharpTokenType::Enum)
            | Some(CSharpTokenType::Record)
            | Some(CSharpTokenType::Delegate)
            | Some(CSharpTokenType::Public)
            | Some(CSharpTokenType::Private)
            | Some(CSharpTokenType::Protected)
            | Some(CSharpTokenType::Internal)
            | Some(CSharpTokenType::Static)
            | Some(CSharpTokenType::AsyncKeyword)
            | Some(CSharpTokenType::Abstract)
            | Some(CSharpTokenType::Virtual)
            | Some(CSharpTokenType::Override) => self.parse_declaration(state)?,
            Some(CSharpTokenType::If) => self.parse_if_statement(state)?,
            Some(CSharpTokenType::While) => self.parse_while_statement(state)?,
            Some(CSharpTokenType::For) => self.parse_for_statement(state)?,
            Some(CSharpTokenType::Foreach) => self.parse_foreach_statement(state)?,
            Some(CSharpTokenType::Return) => self.parse_return_statement(state)?,
            Some(CSharpTokenType::Break) => {
                let cp = state.checkpoint();
                state.bump();
                state.eat(CSharpTokenType::Semicolon);
                state.finish_at(cp, crate::parser::CSharpElementType::BreakStatement);
            }
            Some(CSharpTokenType::Continue) => {
                let cp = state.checkpoint();
                state.bump();
                state.eat(CSharpTokenType::Semicolon);
                state.finish_at(cp, crate::parser::CSharpElementType::ContinueStatement);
            }
            Some(CSharpTokenType::LeftBrace) => self.parse_block(state)?,
            _ => {
                let cp = state.checkpoint();
                PrattParser::parse(state, 0, self);
                state.eat(CSharpTokenType::Semicolon);
                state.finish_at(cp, crate::parser::CSharpElementType::ExpressionStatement);
            }
        }
        Ok(())
    }

    fn parse_foreach_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::CSharpTokenType;
        let cp = state.checkpoint();
        state.bump(); // foreach
        state.expect(CSharpTokenType::LeftParen).ok();
        // type name in iterable
        while state.not_at_end() && !state.at(CSharpTokenType::RightParen) {
            state.bump();
        }
        state.expect(CSharpTokenType::RightParen).ok();
        self.parse_statement(state)?;
        state.finish_at(cp, crate::parser::element_type::CSharpElementType::ForeachStatement);
        Ok(())
    }

    fn parse_namespace_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::CSharpTokenType;
        let cp = state.checkpoint();
        state.expect(CSharpTokenType::Namespace).ok();
        while state.not_at_end() && !state.at(CSharpTokenType::LeftBrace) {
            state.bump();
        }
        self.parse_block(state)?;
        state.finish_at(cp, crate::parser::element_type::CSharpElementType::NamespaceDeclaration);
        Ok(())
    }

    fn parse_using_directive<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::CSharpTokenType;
        let cp = state.checkpoint();
        state.expect(CSharpTokenType::Using).ok();
        while state.not_at_end() && !state.at(CSharpTokenType::Semicolon) {
            state.bump();
        }
        state.eat(CSharpTokenType::Semicolon);
        state.finish_at(cp, crate::parser::element_type::CSharpElementType::UsingDirective);
        Ok(())
    }

    fn parse_accessor_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::CSharpTokenType;
        state.expect(CSharpTokenType::LeftBrace).ok();
        while state.not_at_end() && !state.at(CSharpTokenType::RightBrace) {
            match state.peek_kind() {
                Some(CSharpTokenType::GetKeyword) | Some(CSharpTokenType::SetKeyword) | Some(CSharpTokenType::AddKeyword) | Some(CSharpTokenType::RemoveKeyword) => {
                    state.bump();
                    if state.at(CSharpTokenType::LeftBrace) {
                        self.parse_block(state)?;
                    }
                    else {
                        state.eat(CSharpTokenType::Semicolon);
                    }
                }
                _ => {
                    state.bump();
                }
            }
        }
        state.expect(CSharpTokenType::RightBrace).ok();
        Ok(())
    }

    fn parse_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::{lexer::token_type::CSharpTokenType, parser::CSharpElementType::*};
        let cp = state.checkpoint();

        // Handle modifiers
        while state.not_at_end()
            && matches!(
                state.peek_kind(),
                Some(CSharpTokenType::Public)
                    | Some(CSharpTokenType::Private)
                    | Some(CSharpTokenType::Protected)
                    | Some(CSharpTokenType::Internal)
                    | Some(CSharpTokenType::Static)
                    | Some(CSharpTokenType::Readonly)
                    | Some(CSharpTokenType::Abstract)
                    | Some(CSharpTokenType::Virtual)
                    | Some(CSharpTokenType::Override)
                    | Some(CSharpTokenType::AsyncKeyword)
            )
        {
            state.bump();
        }

        match state.peek_kind() {
            Some(CSharpTokenType::Class) => {
                state.bump();
                state.expect(CSharpTokenType::Identifier).ok();
                // Base types and generics
                while state.not_at_end() && !state.at(CSharpTokenType::LeftBrace) {
                    state.bump();
                }
                self.parse_block(state)?;
                state.finish_at(cp, crate::parser::element_type::CSharpElementType::ClassDeclaration);
            }
            Some(CSharpTokenType::Interface) => {
                state.bump();
                state.expect(CSharpTokenType::Identifier).ok();
                while state.not_at_end() && !state.at(CSharpTokenType::LeftBrace) {
                    state.bump();
                }
                self.parse_block(state)?;
                state.finish_at(cp, crate::parser::element_type::CSharpElementType::InterfaceDeclaration);
            }
            Some(CSharpTokenType::Struct) => {
                state.bump();
                state.expect(CSharpTokenType::Identifier).ok();
                while state.not_at_end() && !state.at(CSharpTokenType::LeftBrace) {
                    state.bump();
                }
                self.parse_block(state)?;
                state.finish_at(cp, crate::parser::element_type::CSharpElementType::StructDeclaration);
            }
            Some(CSharpTokenType::Enum) => {
                state.bump();
                state.expect(CSharpTokenType::Identifier).ok();
                self.parse_block(state)?;
                state.finish_at(cp, crate::parser::element_type::CSharpElementType::EnumDeclaration);
            }
            Some(CSharpTokenType::Record) => {
                state.bump();
                state.expect(CSharpTokenType::Identifier).ok();
                while state.not_at_end() && !state.at(CSharpTokenType::LeftBrace) && !state.at(CSharpTokenType::Semicolon) {
                    state.bump();
                }
                if state.at(CSharpTokenType::LeftBrace) {
                    self.parse_block(state)?;
                }
                else {
                    state.eat(CSharpTokenType::Semicolon);
                }
                state.finish_at(cp, crate::parser::element_type::CSharpElementType::RecordDeclaration);
            }
            Some(CSharpTokenType::Delegate) => {
                state.bump();
                // Type name (parameters);
                while state.not_at_end() && !state.at(CSharpTokenType::Semicolon) {
                    state.bump();
                }
                state.eat(CSharpTokenType::Semicolon);
                state.finish_at(cp, crate::parser::element_type::CSharpElementType::DelegateDeclaration);
            }
            Some(CSharpTokenType::Event) => {
                state.bump();
                // Type name;
                while state.not_at_end() && !state.at(CSharpTokenType::Semicolon) && !state.at(CSharpTokenType::LeftBrace) {
                    state.bump();
                }
                if state.at(CSharpTokenType::LeftBrace) {
                    self.parse_accessor_block(state)?;
                }
                else {
                    state.eat(CSharpTokenType::Semicolon);
                }
                state.finish_at(cp, crate::parser::element_type::CSharpElementType::EventDeclaration);
            }
            _ => {
                // Property, Method, or Field
                // 简化处理
                state.bump(); // Type
                while state.not_at_end() && !state.at(CSharpTokenType::Semicolon) && !state.at(CSharpTokenType::LeftBrace) && !state.at(CSharpTokenType::LeftParen) {
                    state.bump();
                }

                if state.eat(CSharpTokenType::This) && state.at(CSharpTokenType::LeftBracket) {
                    // Indexer
                    state.bump(); // [
                    while state.not_at_end() && !state.at(CSharpTokenType::RightBracket) {
                        state.bump();
                    }
                    state.expect(CSharpTokenType::RightBracket).ok();
                    self.parse_accessor_block(state)?;
                    state.finish_at(cp, crate::parser::element_type::CSharpElementType::IndexerDeclaration);
                }
                else {
                    state.expect(CSharpTokenType::Identifier).ok();
                    if state.at(CSharpTokenType::LeftParen) {
                        // Method
                        state.bump(); // (
                        while state.not_at_end() && !state.at(CSharpTokenType::RightParen) {
                            state.bump();
                        }
                        state.expect(CSharpTokenType::RightParen).ok();
                        if state.at(CSharpTokenType::LeftBrace) {
                            self.parse_block(state)?;
                        }
                        else {
                            state.eat(CSharpTokenType::Semicolon);
                        }
                        state.finish_at(cp, crate::parser::element_type::CSharpElementType::MethodDeclaration);
                    }
                    else if state.at(CSharpTokenType::LeftBrace) {
                        // Property
                        self.parse_accessor_block(state)?;
                        state.finish_at(cp, crate::parser::element_type::CSharpElementType::PropertyDeclaration);
                    }
                    else {
                        // Field
                        state.eat(CSharpTokenType::Semicolon);
                        state.finish_at(cp, crate::parser::element_type::CSharpElementType::FieldDeclaration);
                    }
                }
            }
        }
        Ok(())
    }

    fn parse_if_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::CSharpTokenType;
        let cp = state.checkpoint();
        state.bump(); // if
        state.expect(CSharpTokenType::LeftParen).ok();
        PrattParser::parse(state, 0, self);
        state.expect(CSharpTokenType::RightParen).ok();
        self.parse_statement(state)?;
        if state.eat(CSharpTokenType::Else) {
            self.parse_statement(state)?;
        }
        state.finish_at(cp, crate::parser::element_type::CSharpElementType::IfStatement);
        Ok(())
    }

    fn parse_while_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::CSharpTokenType;
        let cp = state.checkpoint();
        state.bump(); // while
        state.expect(CSharpTokenType::LeftParen).ok();
        PrattParser::parse(state, 0, self);
        state.expect(CSharpTokenType::RightParen).ok();
        self.parse_statement(state)?;
        state.finish_at(cp, crate::parser::element_type::CSharpElementType::WhileStatement);
        Ok(())
    }

    fn parse_for_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::CSharpTokenType;
        let cp = state.checkpoint();
        state.bump(); // for
        state.expect(CSharpTokenType::LeftParen).ok();
        PrattParser::parse(state, 0, self);
        state.expect(CSharpTokenType::RightParen).ok();
        self.parse_statement(state)?;
        state.finish_at(cp, crate::parser::element_type::CSharpElementType::ForStatement);
        Ok(())
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::CSharpTokenType;
        let cp = state.checkpoint();
        state.expect(CSharpTokenType::LeftBrace).ok();
        while state.not_at_end() && !state.at(CSharpTokenType::RightBrace) {
            self.parse_statement(state)?;
        }
        state.expect(CSharpTokenType::RightBrace).ok();
        state.finish_at(cp, crate::parser::element_type::CSharpElementType::Block);
        Ok(())
    }

    fn parse_return_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::CSharpTokenType;
        let cp = state.checkpoint();
        state.bump(); // return
        if !state.at(CSharpTokenType::Semicolon) && !state.at(CSharpTokenType::RightBrace) {
            PrattParser::parse(state, 0, self);
        }
        state.eat(CSharpTokenType::Semicolon);
        state.finish_at(cp, crate::parser::element_type::CSharpElementType::ReturnStatement);
        Ok(())
    }
}

impl<'config> Parser<CSharpLanguage> for CSharpParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<CSharpLanguage>) -> ParseOutput<'a, CSharpLanguage> {
        let lexer = crate::lexer::CSharpLexer::new(self._language);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let cp = state.checkpoint();
            while state.not_at_end() {
                self.parse_statement(state)?;
            }
            Ok(state.finish_at(cp, crate::parser::element_type::CSharpElementType::Root))
        })
    }
}
