#![doc = include_str!("readme.md")]
/// Element types for the C AST.
pub mod element_type;
pub use element_type::CElementType;

use crate::{language::CLanguage, lexer::CTokenType};
use oak_core::{
    GreenNode, OakError, Source,
    parser::{Associativity, ParseCache, ParseOutput, Parser, ParserState, Pratt, PrattParser, binary, parse_with_lexer},
    source::TextEdit,
};

pub(crate) type State<'a, S> = ParserState<'a, CLanguage, S>;

/// Parser for the C language.
pub struct CParser<'config> {
    /// Language configuration.
    pub(crate) config: &'config CLanguage,
}

impl<'config> CParser<'config> {
    /// Creates a new `CParser` with the given language configuration.
    pub fn new(config: &'config CLanguage) -> Self {
        Self { config }
    }

    /// Parses a C statement.
    pub(crate) fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::CTokenType::*;
        self.skip_trivia(state);
        match state.peek_kind() {
            Some(If) => self.parse_if_statement(state)?,
            Some(While) => self.parse_while_statement(state)?,
            Some(For) => self.parse_for_statement(state)?,
            Some(Return) => self.parse_return_statement(state)?,
            Some(LeftBrace) => self.parse_compound_statement(state)?,
            Some(Struct) | Some(Union) | Some(Enum) | Some(Typedef) | Some(Extern) | Some(Static) | Some(Int) | Some(Char) | Some(Void) | Some(Float) | Some(Double) | Some(Long) | Some(Short) | Some(Signed) | Some(Unsigned) | Some(Bool) | Some(Const)
            | Some(Restrict) | Some(Volatile) => self.parse_declaration(state)?,
            Some(Preprocessor) => {
                // Skip preprocessor directives
                while state.not_at_end() && !state.at(CTokenType::Semicolon) {
                    state.bump();
                }
                if state.at(CTokenType::Semicolon) {
                    state.bump();
                }
            }
            Some(Identifier) => {
                // Check if it's a label
                let cp = state.checkpoint();
                state.bump();
                if state.at(Colon) {
                    state.bump();
                    state.finish_at(cp, CElementType::ExpressionStatement);
                }
                else {
                    // Handle identifier as expression
                    let expr = PrattParser::parse(state, 0, self);
                    state.push_child(expr);
                    self.skip_trivia(state);
                    if !state.eat(Semicolon) {
                        // Skip until semicolon or end of statement
                        while state.not_at_end() && !state.at(Semicolon) && !state.at(LeftBrace) && !state.at(RightBrace) {
                            state.bump();
                        }
                        state.eat(Semicolon);
                    }
                }
            }
            Some(Semicolon) => {
                // Empty statement
                state.bump();
            }
            None => {
                // End of file
                return Ok(());
            }
            _ => {
                // Unexpected token, recover by skipping until semicolon or brace
                while state.not_at_end() && !state.at(Semicolon) && !state.at(LeftBrace) && !state.at(RightBrace) {
                    state.bump();
                }
                if state.at(Semicolon) {
                    state.bump();
                }
                else if state.at(LeftBrace) {
                    // Handle left brace by parsing compound statement
                    self.parse_compound_statement(state)?;
                }
                else if state.at(RightBrace) {
                    // Handle right brace by consuming it
                    state.bump();
                }
            }
        }
        Ok(())
    }

    /// Skips trivia tokens (whitespace and comments).
    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while let Some(kind) = state.peek_kind() {
            if matches!(kind, CTokenType::Whitespace | CTokenType::LineComment | CTokenType::BlockComment) {
                state.bump();
            }
            else {
                break;
            }
        }
    }

    /// Parses a C declaration or function definition.
    fn parse_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::CTokenType::*;
        let cp = state.checkpoint();

        // Parse declaration specifiers
        self.parse_declaration_specifiers(state)?;

        // Parse declarator
        self.skip_trivia(state);
        if state.at(Identifier) || state.at(Star) {
            self.parse_declarator(state)?;
        }

        self.skip_trivia(state);

        if state.at(LeftBrace) {
            // Function definition
            self.parse_compound_statement(state)?;
            state.finish_at(cp, CElementType::FunctionDefinition);
        }
        else {
            // Variable declaration
            // Parse initializer if present
            if state.at(Assign) {
                state.bump(); // =
                self.skip_trivia(state);
                let expr = PrattParser::parse(state, 0, self);
                state.push_child(expr);
            }

            // Parse comma separator and additional declarators
            while state.at(Comma) {
                state.bump();
                self.skip_trivia(state);
                if state.at(Identifier) || state.at(Star) {
                    self.parse_declarator(state)?;

                    // Parse initializer if present
                    if state.at(Assign) {
                        state.bump(); // =
                        self.skip_trivia(state);
                        let expr = PrattParser::parse(state, 0, self);
                        state.push_child(expr);
                    }
                }
            }

            state.eat(Semicolon);
            state.finish_at(cp, CElementType::DeclarationStatement);
        }

        Ok(())
    }

    /// Parses declaration specifiers (storage class, type specifiers, type qualifiers).
    fn parse_declaration_specifiers<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::CTokenType::*;

        loop {
            self.skip_trivia(state);
            match state.peek_kind() {
                // Storage class specifiers
                Some(Extern) | Some(Static) | Some(Auto) | Some(Register) | Some(Typedef) => {
                    state.bump();
                }
                // Type specifiers
                Some(Void) | Some(Char) | Some(Short) | Some(Int) | Some(Long) | Some(Float) | Some(Double) | Some(Signed) | Some(Unsigned) | Some(Bool) => {
                    state.bump();
                }
                Some(Struct) => {
                    let cp = state.checkpoint();
                    state.bump();
                    // Parse struct name
                    if state.peek_kind() == Some(Identifier) {
                        state.bump();
                    }
                    if state.peek_kind() == Some(LeftBrace) {
                        self.parse_struct_union_body(state)?;
                    }
                    state.finish_at(cp, CElementType::StructDefinition);
                }
                Some(Union) => {
                    let cp = state.checkpoint();
                    state.bump();
                    // Parse union name
                    if state.peek_kind() == Some(Identifier) {
                        state.bump();
                    }
                    if state.peek_kind() == Some(LeftBrace) {
                        self.parse_struct_union_body(state)?;
                    }
                    state.finish_at(cp, CElementType::UnionDefinition);
                }
                Some(Enum) => {
                    let cp = state.checkpoint();
                    state.bump();
                    // Parse enum name
                    if state.peek_kind() == Some(Identifier) {
                        state.bump();
                    }
                    if state.peek_kind() == Some(LeftBrace) {
                        self.parse_enum_body(state)?;
                    }
                    state.finish_at(cp, CElementType::EnumDefinition);
                }
                // Type qualifiers
                Some(Const) | Some(Restrict) | Some(Volatile) => {
                    state.bump();
                }
                _ => break,
            }
        }
        Ok(())
    }

    /// Parses a declarator (variable name, pointer, array, function).
    fn parse_declarator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::CTokenType::*;

        // Parse pointers
        while state.at(Star) {
            state.bump();
            // Parse type qualifiers after pointer
            while state.at(Const) || state.at(Restrict) || state.at(Volatile) {
                state.bump();
            }
        }

        // Parse direct declarator
        self.parse_direct_declarator(state)?;

        Ok(())
    }

    /// Parses a direct declarator (identifier, array, function).
    fn parse_direct_declarator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::CTokenType::*;

        if state.at(Identifier) {
            state.bump();
        }
        else if state.at(LeftParen) {
            state.bump();
            // Parse parenthesized declarator or expression
            self.skip_trivia(state);

            // Parse the content inside the parentheses
            let mut paren_count = 1;
            while state.not_at_end() && paren_count > 0 {
                if state.at(LeftParen) {
                    paren_count += 1;
                }
                else if state.at(RightParen) {
                    paren_count -= 1;
                }
                if paren_count > 0 {
                    state.bump();
                }
            }

            state.expect(RightParen).ok();
        }
        else {
            return Ok(());
        }

        // Parse array declarator
        while state.at(LeftBracket) {
            state.bump();
            if !state.at(RightBracket) {
                let expr = PrattParser::parse(state, 0, self);
                state.push_child(expr);
            }
            state.expect(RightBracket).ok();
        }

        // Parse function declarator
        if state.at(LeftParen) {
            let pcp = state.checkpoint();
            state.bump();

            // Parse parameter list
            while state.not_at_end() && !state.at(RightParen) {
                self.skip_trivia(state);
                if state.at(Void) {
                    state.bump();
                }
                else if state.at(Identifier) || state.at(Char) || state.at(Int) || state.at(Float) || state.at(Double) {
                    // Parse parameter declaration
                    self.parse_declaration_specifiers(state)?;
                    if state.at(Identifier) || state.at(Star) {
                        // Parse parameter name without recursion
                        self.skip_trivia(state);
                        // Parse pointers
                        while state.at(Star) {
                            state.bump();
                            // Parse type qualifiers after pointer
                            while state.at(Const) || state.at(Restrict) || state.at(Volatile) {
                                state.bump();
                            }
                        }
                        // Parse identifier
                        if state.at(Identifier) {
                            state.bump();
                        }
                    }
                }
                if state.at(Comma) {
                    state.bump();
                }
                else if !state.at(RightParen) {
                    // Skip any unexpected tokens until comma or right paren
                    while state.not_at_end() && !state.at(Comma) && !state.at(RightParen) {
                        state.bump();
                    }
                    if state.at(Comma) {
                        state.bump();
                    }
                }
            }

            state.expect(RightParen).ok();
            state.finish_at(pcp, CElementType::ParameterList);
        }

        Ok(())
    }

    /// Parses struct/union body.
    fn parse_struct_union_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::CTokenType::*;

        if state.at(LeftBrace) {
            state.bump();

            while state.not_at_end() && !state.at(RightBrace) {
                self.skip_trivia(state);
                // Parse struct member declaration
                let member_cp = state.checkpoint();
                self.parse_declaration_specifiers(state)?;
                while state.not_at_end() && !state.at(Semicolon) && !state.at(RightBrace) {
                    if state.at(Identifier) || state.at(Star) {
                        self.parse_declarator(state)?;
                        if state.at(Comma) {
                            state.bump();
                        }
                    }
                    else {
                        break;
                    }
                }
                state.eat(Semicolon);
                state.finish_at(member_cp, CElementType::StructMember);
            }

            state.expect(RightBrace).ok();
        }

        Ok(())
    }

    /// Parses enum body.
    fn parse_enum_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::CTokenType::*;

        if state.at(LeftBrace) {
            state.bump();

            while state.not_at_end() && !state.at(RightBrace) {
                self.skip_trivia(state);
                // Parse enum constant
                if state.at(Identifier) {
                    let const_cp = state.checkpoint();
                    state.bump();
                    // Parse optional initializer
                    if state.at(Assign) {
                        state.bump();
                        let expr = PrattParser::parse(state, 0, self);
                        state.push_child(expr);
                    }
                    state.finish_at(const_cp, CElementType::EnumConstant);
                }

                if state.at(Comma) {
                    state.bump();
                }
                else if !state.at(RightBrace) {
                    // Skip any unexpected tokens until comma or right brace
                    while state.not_at_end() && !state.at(Comma) && !state.at(RightBrace) {
                        state.bump();
                    }
                    if state.at(Comma) {
                        state.bump();
                    }
                }
            }

            state.expect(RightBrace).ok();
        }

        Ok(())
    }

    fn parse_if_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // if
        state.expect(CTokenType::LeftParen).ok();
        let expr = PrattParser::parse(state, 0, self);
        state.push_child(expr);
        state.expect(CTokenType::RightParen).ok();
        self.parse_statement(state)?;
        if state.eat(CTokenType::Else) {
            self.parse_statement(state)?;
        }
        state.finish_at(cp, CElementType::IfStatement);
        Ok(())
    }

    fn parse_while_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // while
        state.expect(CTokenType::LeftParen).ok();
        let expr = PrattParser::parse(state, 0, self);
        state.push_child(expr);
        state.expect(CTokenType::RightParen).ok();
        self.parse_statement(state)?;
        state.finish_at(cp, CElementType::WhileStatement);
        Ok(())
    }

    fn parse_for_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // for
        state.expect(CTokenType::LeftParen).ok();

        // Init
        if !state.at(CTokenType::Semicolon) {
            let expr = PrattParser::parse(state, 0, self);
            state.push_child(expr);
        }
        state.expect(CTokenType::Semicolon).ok();

        // Condition
        if !state.at(CTokenType::Semicolon) {
            let expr = PrattParser::parse(state, 0, self);
            state.push_child(expr);
        }
        state.expect(CTokenType::Semicolon).ok();

        // Increment
        if !state.at(CTokenType::RightParen) {
            let expr = PrattParser::parse(state, 0, self);
            state.push_child(expr);
        }
        state.expect(CTokenType::RightParen).ok();

        self.parse_statement(state)?;
        state.finish_at(cp, CElementType::ForStatement);
        Ok(())
    }

    fn parse_return_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // return
        if !state.at(CTokenType::Semicolon) {
            let expr = PrattParser::parse(state, 0, self);
            state.push_child(expr);
        }
        state.eat(CTokenType::Semicolon);
        state.finish_at(cp, CElementType::ReturnStatement);
        Ok(())
    }

    fn parse_compound_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        if !state.eat(CTokenType::LeftBrace) {
            // Skip until right brace or end of file
            while state.not_at_end() && !state.at(CTokenType::RightBrace) {
                state.bump();
            }
            if state.at(CTokenType::RightBrace) {
                state.bump();
            }
            state.finish_at(cp, CElementType::CompoundStatement);
            return Ok(());
        }

        while state.not_at_end() && !state.at(CTokenType::RightBrace) {
            self.parse_statement(state)?;
        }

        if !state.eat(CTokenType::RightBrace) {
            // Skip until end of file or next statement
            while state.not_at_end() && !state.at(CTokenType::Semicolon) && !state.at(CTokenType::LeftBrace) {
                state.bump();
            }
        }

        state.finish_at(cp, CElementType::CompoundStatement);
        Ok(())
    }
}

impl<'config> Pratt<CLanguage> for CParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, CLanguage> {
        use crate::lexer::CTokenType::*;
        self.skip_trivia(state);
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(Identifier) => {
                state.bump();
                state.finish_at(cp, CElementType::Token(Identifier))
            }
            Some(IntConstant) | Some(FloatConstant) | Some(CharConstant) | Some(StringLiteral) => {
                state.bump();
                state.finish_at(cp, CElementType::ExpressionStatement) // Simplified processing
            }
            Some(LeftParen) => {
                state.bump();
                let expr = PrattParser::parse(state, 0, self);
                state.push_child(expr);
                self.skip_trivia(state);
                state.expect(RightParen).ok();
                state.finish_at(cp, CElementType::ExpressionStatement)
            }
            _ => {
                state.bump();
                state.finish_at(cp, CElementType::Error)
            }
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, CLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, CLanguage>> {
        use crate::lexer::CTokenType::*;
        self.skip_trivia(state);
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            Assign | PlusAssign | MinusAssign | StarAssign | SlashAssign | PercentAssign | AndAssign | OrAssign | XorAssign | LeftShiftAssign | RightShiftAssign => (1, Associativity::Right),
            LogicalOr => (2, Associativity::Left),
            LogicalAnd => (3, Associativity::Left),
            Equal | NotEqual | Less | Greater | LessEqual | GreaterEqual => (4, Associativity::Left),
            Plus | Minus => (10, Associativity::Left),
            Star | Slash | Percent => (11, Associativity::Left),
            LeftParen | LeftBracket | Dot | Arrow => (15, Associativity::Left),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        match kind {
            LeftParen => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(LeftParen).ok();
                while state.not_at_end() && !state.at(RightParen) {
                    let expr = PrattParser::parse(state, 0, self);
                    state.push_child(expr);
                    self.skip_trivia(state);
                    if !state.eat(Comma) {
                        break;
                    }
                }
                state.expect(RightParen).ok();
                Some(state.finish_at(cp, CElementType::FunctionCall))
            }
            LeftBracket => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(LeftBracket).ok();
                let expr = PrattParser::parse(state, 0, self);
                state.push_child(expr);
                state.expect(RightBracket).ok();
                Some(state.finish_at(cp, CElementType::ExpressionStatement))
            }
            Dot | Arrow => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(kind).ok();
                state.expect(Identifier).ok();
                Some(state.finish_at(cp, CElementType::ExpressionStatement))
            }
            _ => Some(binary(state, left, kind, prec, assoc, CElementType::ExpressionStatement, |s, p| PrattParser::parse(s, p, self))),
        }
    }
}

impl<'config> Parser<CLanguage> for CParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<CLanguage>) -> ParseOutput<'a, CLanguage> {
        let lexer = crate::lexer::CLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let cp = state.checkpoint();
            while state.not_at_end() {
                self.parse_statement(state).ok();
            }
            Ok(state.finish_at(cp, CElementType::Root))
        })
    }
}
