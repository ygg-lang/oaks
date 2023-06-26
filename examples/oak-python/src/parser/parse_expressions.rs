use crate::{language::PythonLanguage, lexer::PythonTokenType, parser::PythonElementType as ET};
use oak_core::{
    Source,
    parser::{GreenNode, ParserState},
};

/// Python parser state.
pub(crate) type State<'a, S> = ParserState<'a, PythonLanguage, S>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Associativity {
    Left,
    Right,
    None,
}

impl<'config> super::PythonParser<'config> {
    pub(crate) fn parse_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, min_prec: u8) {
        let mut left = self.primary(state);

        loop {
            self.skip_trivia(state);
            let kind = match state.peek_kind() {
                Some(k) => k,
                None => break,
            };

            let (prec, assoc) = match self.get_infix_prec(kind) {
                Some(p) => p,
                None => break,
            };

            if prec < min_prec {
                break;
            }

            left = self.infix(state, left, prec, assoc).unwrap_or(left);
        }
    }

    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, PythonLanguage> {
        use PythonTokenType::*;
        self.skip_trivia(state);

        let cp = state.checkpoint();
        let kind = state.peek_kind();
        match kind {
            Some(Identifier) => {
                state.bump();
                state.finish_at(cp, ET::Name)
            }
            Some(Number) | Some(String) | Some(Bytes) | Some(TrueKeyword) | Some(FalseKeyword) | Some(NoneKeyword) => {
                state.bump();
                state.finish_at(cp, ET::Constant)
            }
            Some(LeftParen) => {
                state.bump();
                self.skip_trivia(state);
                if state.at(RightParen) {
                    state.bump();
                    state.finish_at(cp, ET::Tuple)
                }
                else {
                    self.parse_expression(state, 0);
                    self.skip_trivia(state);

                    if state.at(ForKeyword) {
                        self.parse_comprehension(state);
                        state.expect(RightParen).ok();
                        state.finish_at(cp, ET::GeneratorExp)
                    }
                    else if state.at(Comma) {
                        while state.at(Comma) {
                            state.bump();
                            self.skip_trivia(state);
                            if state.at(RightParen) {
                                break;
                            }
                            self.parse_expression(state, 0);
                            self.skip_trivia(state);
                        }
                        state.expect(RightParen).ok();
                        state.finish_at(cp, ET::Tuple)
                    }
                    else {
                        state.expect(RightParen).ok();
                        state.finish_at(cp, ET::Expr)
                    }
                }
            }
            Some(LeftBracket) => {
                state.bump();
                self.skip_trivia(state);
                if state.at(RightBracket) {
                    state.bump();
                    state.finish_at(cp, ET::List)
                }
                else {
                    self.parse_expression(state, 0);
                    self.skip_trivia(state);

                    if state.at(ForKeyword) {
                        self.parse_comprehension(state);
                        state.expect(RightBracket).ok();
                        state.finish_at(cp, ET::ListComp)
                    }
                    else {
                        while state.at(Comma) {
                            state.bump();
                            self.skip_trivia(state);
                            if state.at(RightBracket) {
                                break;
                            }
                            self.parse_expression(state, 0);
                            self.skip_trivia(state);
                        }
                        state.expect(RightBracket).ok();
                        state.finish_at(cp, ET::List)
                    }
                }
            }
            Some(LeftBrace) => {
                state.bump();
                self.skip_trivia(state);
                if state.at(RightBrace) {
                    state.bump();
                    state.finish_at(cp, ET::Dict)
                }
                else {
                    // Could be set or dict
                    self.parse_expression(state, 0);
                    self.skip_trivia(state);

                    if state.at(Colon) {
                        // Dict
                        state.bump();
                        self.parse_expression(state, 0);
                        self.skip_trivia(state);

                        if state.at(ForKeyword) {
                            self.parse_comprehension(state);
                            state.expect(RightBrace).ok();
                            state.finish_at(cp, ET::DictComp)
                        }
                        else {
                            while state.at(Comma) {
                                state.bump();
                                self.skip_trivia(state);
                                if state.at(RightBrace) {
                                    break;
                                }
                                self.parse_expression(state, 0);
                                self.skip_trivia(state);
                                state.expect(Colon).ok();
                                self.parse_expression(state, 0);
                                self.skip_trivia(state);
                            }
                            state.expect(RightBrace).ok();
                            state.finish_at(cp, ET::Dict)
                        }
                    }
                    else if state.at(ForKeyword) {
                        // Set comprehension
                        self.parse_comprehension(state);
                        state.expect(RightBrace).ok();
                        state.finish_at(cp, ET::SetComp)
                    }
                    else {
                        // Set
                        while state.at(Comma) {
                            state.bump();
                            self.skip_trivia(state);
                            if state.at(RightBrace) {
                                break;
                            }
                            self.parse_expression(state, 0);
                            self.skip_trivia(state);
                        }
                        state.expect(RightBrace).ok();
                        state.finish_at(cp, ET::Set)
                    }
                }
            }
            Some(Plus) | Some(Minus) | Some(Tilde) | Some(NotKeyword) | Some(AwaitKeyword) => {
                state.bump();
                self.parse_expression(state, 12); // Unary op prec
                if kind == Some(AwaitKeyword) { state.finish_at(cp, ET::Await) } else { state.finish_at(cp, ET::UnaryOp) }
            }
            Some(LambdaKeyword) => {
                state.bump();
                // parse params
                self.skip_trivia(state);
                if !state.at(Colon) {
                    loop {
                        state.expect(Identifier).ok();
                        self.skip_trivia(state);
                        if !state.eat(Comma) {
                            break;
                        }
                        self.skip_trivia(state);
                    }
                }
                state.expect(Colon).ok();
                self.parse_expression(state, 0);
                state.finish_at(cp, ET::Lambda)
            }
            _ => {
                state.bump();
                state.finish_at(cp, ET::Error)
            }
        }
    }

    fn get_infix_prec(&self, kind: PythonTokenType) -> Option<(u8, Associativity)> {
        use PythonTokenType::*;
        match kind {
            Plus | Minus => Some((10, Associativity::Left)),
            Star | Slash | Percent | DoubleSlash => Some((11, Associativity::Left)),
            DoubleStar => Some((13, Associativity::Right)),
            Equal | NotEqual | Less | LessEqual | Greater | GreaterEqual | InKeyword | IsKeyword | NotKeyword => Some((5, Associativity::Left)),
            AndKeyword => Some((3, Associativity::Left)),
            OrKeyword => Some((2, Associativity::Left)),
            Assign | PlusAssign | MinusAssign | StarAssign | SlashAssign => Some((1, Associativity::None)),
            Dot | LeftParen | LeftBracket => Some((15, Associativity::Left)),
            _ => None,
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, _left: &'a GreenNode<'a, PythonLanguage>, prec: u8, assoc: Associativity) -> Option<&'a GreenNode<'a, PythonLanguage>> {
        use PythonTokenType::*;
        let cp = state.checkpoint_before(_left);
        let kind = state.peek_kind()?;

        match kind {
            LeftParen => {
                state.bump(); // Consume LeftParen
                self.skip_trivia(state);

                while !state.at(RightParen) && !state.at(Eof) {
                    self.parse_expression(state, 0);
                    self.skip_trivia(state);
                    if !state.eat(Comma) {
                        break;
                    }
                }

                self.skip_trivia(state);
                state.expect(RightParen).ok();
                Some(state.finish_at(cp, ET::Call))
            }
            LeftBracket => {
                state.bump(); // Consume LeftBracket

                let cp_slice = state.checkpoint();
                let mut is_slice = false;

                if state.at(Colon) {
                    is_slice = true;
                }
                else {
                    self.parse_expression(state, 0);
                    self.skip_trivia(state);
                    if state.at(Colon) {
                        is_slice = true;
                    }
                }

                if is_slice {
                    // It's a slice
                    while state.at(Colon) {
                        state.bump();
                        self.skip_trivia(state);
                        if !state.at(Colon) && !state.at(RightBracket) {
                            self.parse_expression(state, 0);
                            self.skip_trivia(state);
                        }
                    }
                    state.finish_at(cp_slice, ET::Slice);
                }

                state.expect(RightBracket).ok();
                Some(state.finish_at(cp, ET::Subscript))
            }
            Dot => {
                state.bump(); // Consume Dot
                self.skip_trivia(state);
                state.expect(Identifier).ok();
                Some(state.finish_at(cp, ET::Attribute))
            }
            _ => {
                let result_kind = if prec == 1 {
                    ET::AssignStmt
                }
                else if prec <= 3 {
                    ET::BoolOp
                }
                else if prec == 5 {
                    ET::Compare
                }
                else {
                    ET::BinOp
                };

                state.bump(); // Consume operator

                let next_prec = match assoc {
                    Associativity::Left => prec + 1,
                    Associativity::Right => prec,
                    Associativity::None => prec + 1,
                };

                self.parse_expression(state, next_prec);
                Some(state.finish_at(cp, result_kind.into()))
            }
        }
    }
}
