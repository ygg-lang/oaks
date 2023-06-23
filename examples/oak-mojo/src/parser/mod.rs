pub mod element_type;
pub use element_type::MojoElementType;

use crate::{
    MojoLanguage,
    ast::*,
    lexer::{MojoLexer, MojoTokenType},
};
use oak_core::{
    OakError,
    parser::{
        ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer,
        pratt::{Associativity, Pratt, PrattParser},
    },
    source::{Source, TextEdit},
    tree::GreenNode,
};

pub(crate) type State<'a, S> = ParserState<'a, MojoLanguage, S>;

/// Mojo 语法解析器
#[derive(Default)]
pub struct MojoParser {}

impl Parser<MojoLanguage> for MojoParser {
    fn parse<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<MojoLanguage>) -> ParseOutput<'a, MojoLanguage> {
        let lexer = MojoLexer::new();
        parse_with_lexer(&lexer, source, edits, cache, |state| self.parse_root(state))
    }
}

impl MojoParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn parse_root<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, MojoLanguage>, OakError> {
        let cp = state.checkpoint();
        while state.not_at_end() {
            self.skip_trivia(state);
            if !state.not_at_end() {
                break;
            }
            if state.at(MojoTokenType::Newline) {
                state.bump();
                continue;
            }
            self.parse_statement(state)?;
        }
        Ok(state.finish_at(cp, MojoElementType::Root.into()))
    }

    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while state.not_at_end() {
            if let Some(kind) = state.peek_kind() {
                if kind == MojoTokenType::Whitespace || kind == MojoTokenType::Comment {
                    state.bump();
                    continue;
                }
            }
            break;
        }
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.skip_trivia(state);
        if state.at(MojoTokenType::Fn) {
            self.parse_function_def(state)
        }
        else if state.at(MojoTokenType::Var) || state.at(MojoTokenType::Let) {
            self.parse_variable_decl(state)
        }
        else if state.at(MojoTokenType::If) {
            self.parse_if_stmt(state)
        }
        else if state.at(MojoTokenType::While) {
            self.parse_while_stmt(state)
        }
        else if state.at(MojoTokenType::Return) {
            self.parse_return_stmt(state)
        }
        else {
            self.parse_expression_stmt(state)
        }
    }

    fn parse_function_def<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(MojoElementType::FunctionDef.into(), |state| {
            state.expect(MojoTokenType::Fn)?;
            self.skip_trivia(state);
            state.expect(MojoTokenType::Identifier)?;
            self.skip_trivia(state);
            state.expect(MojoTokenType::LeftParen)?;
            // TODO: Parameters
            state.expect(MojoTokenType::RightParen)?;
            self.skip_trivia(state);
            if state.eat(MojoTokenType::Arrow) {
                self.skip_trivia(state);
                state.expect(MojoTokenType::Identifier)?; // Return type
                self.skip_trivia(state);
            }
            state.expect(MojoTokenType::Colon)?;
            self.parse_block(state)
        })
    }

    fn parse_variable_decl<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(MojoElementType::VariableDecl.into(), |state| {
            if state.at(MojoTokenType::Var) {
                state.bump();
            }
            else {
                state.expect(MojoTokenType::Let)?;
            }
            self.skip_trivia(state);
            state.expect(MojoTokenType::Identifier)?;
            self.skip_trivia(state);
            if state.eat(MojoTokenType::Colon) {
                self.skip_trivia(state);
                state.expect(MojoTokenType::Identifier)?; // Type
                self.skip_trivia(state);
            }
            if state.eat(MojoTokenType::Equal) {
                self.skip_trivia(state);
                self.parse_expression(state, 0);
            }
            state.eat(MojoTokenType::Newline);
            Ok(())
        })
    }

    fn parse_if_stmt<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(MojoElementType::IfStatement.into(), |state| {
            state.expect(MojoTokenType::If)?;
            self.skip_trivia(state);
            self.parse_expression(state, 0);
            self.skip_trivia(state);
            state.expect(MojoTokenType::Colon)?;
            self.parse_block(state)?;
            self.skip_trivia(state);
            if state.eat(MojoTokenType::Else) {
                self.skip_trivia(state);
                if state.at(MojoTokenType::If) {
                    self.parse_if_stmt(state)?;
                }
                else {
                    state.expect(MojoTokenType::Colon)?;
                    self.parse_block(state)?;
                }
            }
            Ok(())
        })
    }

    fn parse_while_stmt<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(MojoElementType::WhileStatement.into(), |state| {
            state.expect(MojoTokenType::While)?;
            self.skip_trivia(state);
            self.parse_expression(state, 0);
            self.skip_trivia(state);
            state.expect(MojoTokenType::Colon)?;
            self.parse_block(state)
        })
    }

    fn parse_return_stmt<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(MojoElementType::ReturnStatement.into(), |state| {
            state.expect(MojoTokenType::Return)?;
            self.skip_trivia(state);
            if !state.at(MojoTokenType::Newline) && !state.at(MojoTokenType::EndOfStream) {
                self.parse_expression(state, 0);
            }
            state.eat(MojoTokenType::Newline);
            Ok(())
        })
    }

    fn parse_expression_stmt<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(MojoElementType::ExpressionStatement.into(), |state| {
            self.parse_expression(state, 0);
            state.eat(MojoTokenType::Newline);
            Ok(())
        })
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(MojoElementType::Block.into(), |state| {
            // 跳过冒号后的空格
            self.skip_trivia(state);
            // 必须有一个换行符
            state.expect(MojoTokenType::Newline)?;
            // 之后可能有多个空行
            while state.eat(MojoTokenType::Newline) {
                self.skip_trivia(state);
            }
            // 缩进开始
            state.expect(MojoTokenType::Indent)?;
            while state.not_at_end() && !state.at(MojoTokenType::Dedent) {
                self.skip_trivia(state);
                if state.eat(MojoTokenType::Newline) {
                    continue;
                }
                self.parse_statement(state)?;
            }
            state.expect(MojoTokenType::Dedent)
        })
    }

    fn parse_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, min_precedence: u8) -> &'a GreenNode<'a, MojoLanguage> {
        PrattParser::parse(state, min_precedence, self)
    }
}

impl Pratt<MojoLanguage> for MojoParser {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, MojoLanguage> {
        self.skip_trivia(state);
        let cp = state.checkpoint();
        if state.at(MojoTokenType::Identifier) {
            state.bump();
            state.finish_at(cp, MojoElementType::IdentifierExpr.into())
        }
        else if state.at(MojoTokenType::Integer) || state.at(MojoTokenType::Float) || state.at(MojoTokenType::String) {
            state.bump();
            state.finish_at(cp, MojoElementType::LiteralExpr.into())
        }
        else if state.at(MojoTokenType::LeftParen) {
            state.bump();
            self.parse_expression(state, 0);
            state.expect(MojoTokenType::RightParen).ok();
            state.finish_at(cp, MojoElementType::Grouping.into())
        }
        else {
            state.bump(); // Error recovery
            state.finish_at(cp, MojoElementType::Error.into())
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, MojoLanguage> {
        self.primary(state)
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, MojoLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, MojoLanguage>> {
        self.skip_trivia(state);
        let kind = state.peek_kind()?;
        let (precedence, associativity) = match kind {
            MojoTokenType::Plus | MojoTokenType::Minus => (10, Associativity::Left),
            MojoTokenType::Star | MojoTokenType::Slash | MojoTokenType::Percent => (20, Associativity::Left),
            MojoTokenType::EqualEqual | MojoTokenType::NotEqual | MojoTokenType::Less | MojoTokenType::LessEqual | MojoTokenType::Greater | MojoTokenType::GreaterEqual => (5, Associativity::Left),
            _ => return None,
        };

        if precedence < min_precedence {
            return None;
        }

        let cp = state.checkpoint_before(left);
        state.bump();
        let next_prec = if associativity == Associativity::Left { precedence + 1 } else { precedence };
        self.parse_expression(state, next_prec);
        Some(state.finish_at(cp, MojoElementType::BinaryExpr.into()))
    }
}
