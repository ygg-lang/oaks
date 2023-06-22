use crate::{kind::KotlinSyntaxKind, language::KotlinLanguage, lexer::KotlinLexer};
use oak_core::{
    GreenNode, OakError, TextEdit,
    parser::{
        ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer,
        pratt::{Associativity, Pratt, PrattParser, binary, unary},
    },
    source::Source,
};

pub(crate) type State<'a, S> = ParserState<'a, KotlinLanguage, S>;

pub struct KotlinParser<'config> {
    pub(crate) config: &'config KotlinLanguage,
}

impl<'config> KotlinParser<'config> {
    pub fn new(config: &'config KotlinLanguage) -> Self {
        Self { config }
    }

    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, KotlinLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            self.skip_trivia(state);
            if state.not_at_end() {
                self.parse_statement(state).ok();
            }
        }

        Ok(state.finish_at(checkpoint, KotlinSyntaxKind::SourceFile.into()))
    }

    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        use crate::kind::KotlinSyntaxKind::*;
        while let Some(kind) = state.peek_kind() {
            if matches!(kind, Whitespace | Newline | Comment) {
                state.bump();
            }
            else {
                break;
            }
        }
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::KotlinSyntaxKind::*;
        self.skip_trivia(state);
        match state.peek_kind() {
            Some(Fun) => self.parse_function_declaration(state)?,
            Some(Class) => self.parse_class_declaration(state)?,
            Some(Val) | Some(Var) => self.parse_variable_declaration(state)?,
            Some(If) => self.parse_if_statement(state)?,
            Some(While) => self.parse_while_statement(state)?,
            Some(Return) => self.parse_return_statement(state)?,
            Some(LBrace) => self.parse_block(state)?,
            _ => {
                PrattParser::parse(state, 0, self);
                self.skip_trivia(state);
                state.eat(Semi);
            }
        }
        Ok(())
    }

    fn parse_class_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::KotlinSyntaxKind::*;
        let cp = state.checkpoint();
        state.expect(Class).ok();
        self.skip_trivia(state);
        state.expect(Identifier).ok();
        self.skip_trivia(state);
        if state.eat(LParen) {
            while state.not_at_end() && !state.at(RParen) {
                let p_cp = state.checkpoint();
                self.skip_trivia(state);
                state.eat(Val);
                state.eat(Var);
                self.skip_trivia(state);
                state.expect(Identifier).ok();
                self.skip_trivia(state);
                if state.eat(Colon) {
                    self.skip_trivia(state);
                    state.expect(Identifier).ok();
                }
                state.finish_at(p_cp, Parameter.into());
                self.skip_trivia(state);
                state.eat(Comma);
            }
            self.skip_trivia(state);
            state.expect(RParen).ok();
        }
        self.skip_trivia(state);
        if state.at(LBrace) {
            self.parse_block(state)?;
        }
        state.finish_at(cp, ClassDeclaration.into());
        Ok(())
    }

    fn parse_function_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::KotlinSyntaxKind::*;
        let cp = state.checkpoint();
        state.expect(Fun).ok();
        self.skip_trivia(state);
        state.expect(Identifier).ok();
        self.skip_trivia(state);
        if state.eat(LParen) {
            while state.not_at_end() && !state.at(RParen) {
                let p_cp = state.checkpoint();
                self.skip_trivia(state);
                state.expect(Identifier).ok();
                self.skip_trivia(state);
                if state.eat(Colon) {
                    self.skip_trivia(state);
                    state.expect(Identifier).ok();
                }
                state.finish_at(p_cp, Parameter.into());
                self.skip_trivia(state);
                state.eat(Comma);
            }
            self.skip_trivia(state);
            state.expect(RParen).ok();
        }
        self.skip_trivia(state);
        if state.eat(Colon) {
            self.skip_trivia(state);
            state.expect(Identifier).ok();
        }
        self.skip_trivia(state);
        self.parse_block(state)?;
        state.finish_at(cp, FunctionDeclaration.into());
        Ok(())
    }

    fn parse_variable_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::KotlinSyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // val/var
        self.skip_trivia(state);
        state.expect(Identifier).ok();
        self.skip_trivia(state);
        if state.eat(Colon) {
            self.skip_trivia(state);
            state.expect(Identifier).ok();
        }
        self.skip_trivia(state);
        if state.eat(Assign) {
            self.skip_trivia(state);
            PrattParser::parse(state, 0, self);
        }
        self.skip_trivia(state);
        state.eat(Semi);
        state.finish_at(cp, VariableDeclaration.into());
        Ok(())
    }

    fn parse_if_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::KotlinSyntaxKind::*;
        let cp = state.checkpoint();
        state.expect(If).ok();
        self.skip_trivia(state);
        state.expect(LParen).ok();
        self.skip_trivia(state);
        PrattParser::parse(state, 0, self);
        self.skip_trivia(state);
        state.expect(RParen).ok();
        self.skip_trivia(state);
        self.parse_statement(state)?;
        self.skip_trivia(state);
        if state.eat(Else) {
            self.skip_trivia(state);
            self.parse_statement(state)?;
        }
        state.finish_at(cp, IfStatement.into());
        Ok(())
    }

    fn parse_while_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::KotlinSyntaxKind::*;
        let cp = state.checkpoint();
        state.expect(While).ok();
        self.skip_trivia(state);
        state.expect(LParen).ok();
        self.skip_trivia(state);
        PrattParser::parse(state, 0, self);
        self.skip_trivia(state);
        state.expect(RParen).ok();
        self.skip_trivia(state);
        self.parse_statement(state)?;
        state.finish_at(cp, WhileStatement.into());
        Ok(())
    }

    fn parse_return_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::KotlinSyntaxKind::*;
        let cp = state.checkpoint();
        state.expect(Return).ok();
        self.skip_trivia(state);
        if state.not_at_end() && !state.at(Semi) && !state.at(RBrace) {
            PrattParser::parse(state, 0, self);
        }
        self.skip_trivia(state);
        state.eat(Semi);
        state.finish_at(cp, ReturnStatement.into());
        Ok(())
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::KotlinSyntaxKind::*;
        let cp = state.checkpoint();
        state.expect(LBrace).ok();
        while state.not_at_end() && !state.at(RBrace) {
            self.skip_trivia(state);
            if state.at(RBrace) {
                break;
            }
            self.parse_statement(state)?;
        }
        self.skip_trivia(state);
        state.expect(RBrace).ok();
        state.finish_at(cp, Block.into());
        Ok(())
    }
}

impl<'config> Pratt<KotlinLanguage> for KotlinParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, KotlinLanguage> {
        use crate::kind::KotlinSyntaxKind::*;
        self.skip_trivia(state);
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(Identifier) => {
                state.bump();
                state.finish_at(cp, Identifier.into())
            }
            Some(StringLiteral) | Some(CharLiteral) | Some(NumberLiteral) | Some(IntLiteral) | Some(FloatLiteral) | Some(BooleanLiteral) | Some(Null) | Some(True) | Some(False) => {
                state.bump();
                state.finish_at(cp, IntLiteral.into()) // Simplified
            }
            Some(LParen) => {
                state.bump();
                self.skip_trivia(state);
                PrattParser::parse(state, 0, self);
                self.skip_trivia(state);
                state.expect(RParen).ok();
                state.finish_at(cp, SourceFile.into()) // Placeholder
            }
            _ => {
                state.bump();
                state.finish_at(cp, Error.into())
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, KotlinLanguage> {
        use crate::kind::KotlinSyntaxKind::*;
        self.skip_trivia(state);
        let kind = match state.peek_kind() {
            Some(k) => k,
            None => return self.primary(state),
        };

        match kind {
            Plus | Minus | Exclamation | Tilde => unary(state, kind, 12, SourceFile.into(), |s, p| {
                self.skip_trivia(s);
                PrattParser::parse(s, p, self)
            }),
            _ => self.primary(state),
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, KotlinLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, KotlinLanguage>> {
        use crate::kind::KotlinSyntaxKind::*;
        self.skip_trivia(state);
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            Assign | PlusAssign | MinusAssign | StarAssign | SlashAssign | PercentAssign => (1, Associativity::Right),
            OrOr => (2, Associativity::Left),
            AndAnd => (3, Associativity::Left),
            Lt | Gt | LtEq | GtEq | EqEq | NotEq | Is | As => (5, Associativity::Left),
            Plus | Minus | Pipe | Caret => (6, Associativity::Left),
            Star | Slash | Percent | Ampersand => (7, Associativity::Left),
            Dot | DoubleColon | Range => (8, Associativity::Left),
            LParen | LBracket => (9, Associativity::Left),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        match kind {
            Dot => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.bump();
                self.skip_trivia(state);
                state.expect(Identifier).ok();
                Some(state.finish_at(cp, MemberAccessExpression.into()))
            }
            LParen => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.bump();
                while state.not_at_end() && !state.at(RParen) {
                    self.skip_trivia(state);
                    PrattParser::parse(state, 0, self);
                    self.skip_trivia(state);
                    if !state.eat(Comma) {
                        break;
                    }
                }
                self.skip_trivia(state);
                state.expect(RParen).ok();
                Some(state.finish_at(cp, CallExpression.into()))
            }
            LBracket => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.bump();
                self.skip_trivia(state);
                PrattParser::parse(state, 0, self);
                self.skip_trivia(state);
                state.expect(RBracket).ok();
                Some(state.finish_at(cp, MemberAccessExpression.into()))
            }
            Assign | PlusAssign | MinusAssign | StarAssign | SlashAssign | PercentAssign => Some(binary(state, left, kind, prec, assoc, AssignmentExpression.into(), |s, p| {
                self.skip_trivia(s);
                PrattParser::parse(s, p, self)
            })),
            _ => Some(binary(state, left, kind, prec, assoc, BinaryExpression.into(), |s, p| {
                self.skip_trivia(s);
                PrattParser::parse(s, p, self)
            })),
        }
    }
}

impl<'config> Parser<KotlinLanguage> for KotlinParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<KotlinLanguage>) -> ParseOutput<'a, KotlinLanguage> {
        let lexer = KotlinLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
