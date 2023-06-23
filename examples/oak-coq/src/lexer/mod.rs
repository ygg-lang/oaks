#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::CoqLanguage, lexer::token_type::CoqTokenType};
use oak_core::{
    Lexer, LexerState, Source, TextEdit,
    lexer::{LexOutput, LexerCache},
};

/// A lexer for the Coq programming language.
#[derive(Clone, Debug)]
pub struct CoqLexer<'config> {
    #[allow(dead_code)]
    config: &'config CoqLanguage,
}

type State<'a, S> = LexerState<'a, S, CoqLanguage>;

impl<'config> CoqLexer<'config> {
    /// Creates a new CoqLexer with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - A reference to the CoqLanguage configuration
    ///
    /// # Returns
    ///
    /// A new CoqLexer instance
    pub fn new(config: &'config CoqLanguage) -> Self {
        Self { config }
    }

    fn lex_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while let Some(c) = state.peek() {
            let start = state.get_position();
            match c {
                ' ' | '\t' | '\r' => {
                    state.advance(1);
                    while let Some(' ' | '\t' | '\r') = state.peek() {
                        state.advance(1)
                    }
                    state.add_token(CoqTokenType::Whitespace, start, state.get_position())
                }
                '\n' => {
                    state.advance(1);
                    state.add_token(CoqTokenType::Newline, start, state.get_position())
                }
                '(' => {
                    state.advance(1);
                    if let Some('*') = state.peek() {
                        state.advance(1);
                        self.lex_comment(state, start)
                    }
                    else {
                        state.add_token(CoqTokenType::LeftParen, start, state.get_position())
                    }
                }
                ')' => {
                    state.advance(1);
                    state.add_token(CoqTokenType::RightParen, start, state.get_position())
                }
                '[' => {
                    state.advance(1);
                    state.add_token(CoqTokenType::LeftBracket, start, state.get_position())
                }
                ']' => {
                    state.advance(1);
                    state.add_token(CoqTokenType::RightBracket, start, state.get_position())
                }
                '{' => {
                    state.advance(1);
                    state.add_token(CoqTokenType::LeftBrace, start, state.get_position())
                }
                '}' => {
                    state.advance(1);
                    state.add_token(CoqTokenType::RightBrace, start, state.get_position())
                }
                ',' => {
                    state.advance(1);
                    state.add_token(CoqTokenType::Comma, start, state.get_position())
                }
                ';' => {
                    state.advance(1);
                    state.add_token(CoqTokenType::Semicolon, start, state.get_position())
                }
                '.' => {
                    state.advance(1);
                    state.add_token(CoqTokenType::Dot, start, state.get_position())
                }
                ':' => {
                    state.advance(1);
                    if let Some(':') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            state.add_token(CoqTokenType::DoubleColonEqual, start, state.get_position())
                        }
                        else {
                            state.add_token(CoqTokenType::DoubleColon, start, state.get_position())
                        }
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(CoqTokenType::ColonEqual, start, state.get_position())
                    }
                    else {
                        state.add_token(CoqTokenType::Colon, start, state.get_position())
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('>') = state.peek() {
                        state.advance(1);
                        state.add_token(CoqTokenType::DoubleArrow, start, state.get_position())
                    }
                    else {
                        state.add_token(CoqTokenType::Equal, start, state.get_position())
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('-') = state.peek() {
                        state.advance(1);
                        state.add_token(CoqTokenType::LeftArrow, start, state.get_position())
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        state.add_token(CoqTokenType::DoubleArrow, start, state.get_position())
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(CoqTokenType::LessEqual, start, state.get_position())
                    }
                    else {
                        state.add_token(CoqTokenType::Less, start, state.get_position())
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(CoqTokenType::GreaterEqual, start, state.get_position())
                    }
                    else {
                        state.add_token(CoqTokenType::Greater, start, state.get_position())
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('-') = state.peek() {
                        state.advance(1);
                        state.add_token(CoqTokenType::Turnstile, start, state.get_position())
                    }
                    else {
                        state.add_token(CoqTokenType::Pipe, start, state.get_position())
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('>') = state.peek() {
                        state.advance(1);
                        state.add_token(CoqTokenType::Arrow, start, state.get_position())
                    }
                    else {
                        state.add_token(CoqTokenType::Minus, start, state.get_position())
                    }
                }
                '+' => {
                    state.advance(1);
                    state.add_token(CoqTokenType::Plus, start, state.get_position())
                }
                '*' => {
                    state.advance(1);
                    state.add_token(CoqTokenType::Star, start, state.get_position())
                }
                '/' => {
                    state.advance(1);
                    if let Some('\\') = state.peek() {
                        state.advance(1);
                        state.add_token(CoqTokenType::And, start, state.get_position())
                    }
                    else {
                        state.add_token(CoqTokenType::Slash, start, state.get_position())
                    }
                }
                '\\' => {
                    state.advance(1);
                    if let Some('/') = state.peek() {
                        state.advance(1);
                        state.add_token(CoqTokenType::Or, start, state.get_position())
                    }
                    else {
                        state.add_token(CoqTokenType::Backslash, start, state.get_position())
                    }
                }
                '~' => {
                    state.advance(1);
                    state.add_token(CoqTokenType::Tilde, start, state.get_position())
                }
                '!' => {
                    state.advance(1);
                    state.add_token(CoqTokenType::Exclamation, start, state.get_position())
                }
                '?' => {
                    state.advance(1);
                    state.add_token(CoqTokenType::Question, start, state.get_position())
                }
                '@' => {
                    state.advance(1);
                    state.add_token(CoqTokenType::At, start, state.get_position())
                }
                '#' => {
                    state.advance(1);
                    state.add_token(CoqTokenType::Hash, start, state.get_position())
                }
                '$' => {
                    state.advance(1);
                    state.add_token(CoqTokenType::Dollar, start, state.get_position())
                }
                '%' => {
                    state.advance(1);
                    state.add_token(CoqTokenType::Percent, start, state.get_position())
                }
                '^' => {
                    state.advance(1);
                    state.add_token(CoqTokenType::Caret, start, state.get_position())
                }
                '&' => {
                    state.advance(1);
                    state.add_token(CoqTokenType::Ampersand, start, state.get_position())
                }
                '"' => {
                    state.advance(1);
                    self.lex_string(state, start)
                }
                '0'..='9' => {
                    state.advance(1);
                    while let Some('0'..='9') = state.peek() {
                        state.advance(1)
                    }
                    state.add_token(CoqTokenType::NumberLiteral, start, state.get_position())
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    state.advance(1);
                    while let Some('a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '\'') = state.peek() {
                        state.advance(1)
                    }
                    let end = state.get_position();
                    let text = state.get_text_in((start..end).into());
                    let kind = match text.as_ref() {
                        "Theorem" => CoqTokenType::Theorem,
                        "Lemma" => CoqTokenType::Lemma,
                        "Remark" => CoqTokenType::Remark,
                        "Fact" => CoqTokenType::Fact,
                        "Corollary" => CoqTokenType::Corollary,
                        "Proposition" => CoqTokenType::Proposition,
                        "Definition" => CoqTokenType::Definition,
                        "Example" => CoqTokenType::Example,
                        "Fixpoint" => CoqTokenType::Fixpoint,
                        "CoFixpoint" => CoqTokenType::CoFixpoint,
                        "Inductive" => CoqTokenType::Inductive,
                        "CoInductive" => CoqTokenType::CoInductive,
                        "Record" => CoqTokenType::Record,
                        "Structure" => CoqTokenType::Structure,
                        "Variant" => CoqTokenType::Variant,
                        "Module" => CoqTokenType::Module,
                        "Section" => CoqTokenType::Section,
                        "End" => CoqTokenType::End,
                        "Require" => CoqTokenType::Require,
                        "Import" => CoqTokenType::Import,
                        "Export" => CoqTokenType::Export,
                        "Proof" => CoqTokenType::Proof,
                        "Qed" => CoqTokenType::Qed,
                        "Defined" => CoqTokenType::Defined,
                        "Admitted" => CoqTokenType::Admitted,
                        "Abort" => CoqTokenType::Abort,
                        "Match" => CoqTokenType::Match,
                        "With" => CoqTokenType::With,
                        "Forall" => CoqTokenType::Forall,
                        "Exists" => CoqTokenType::Exists,
                        "Fun" => CoqTokenType::Fun,
                        "Let" => CoqTokenType::Let,
                        "In" => CoqTokenType::In,
                        "If" => CoqTokenType::If,
                        "Then" => CoqTokenType::Then,
                        "Else" => CoqTokenType::Else,
                        "Type" => CoqTokenType::Type,
                        "Prop" => CoqTokenType::Prop,
                        "Set" => CoqTokenType::Set,
                        "Check" => CoqTokenType::Check,
                        "Print" => CoqTokenType::Print,
                        "Search" => CoqTokenType::Search,
                        "Locate" => CoqTokenType::Locate,
                        "About" => CoqTokenType::About,
                        _ => CoqTokenType::Identifier,
                    };
                    state.add_token(kind, start, end)
                }
                _ => {
                    state.advance(1);
                    state.add_token(CoqTokenType::Error, start, state.get_position());
                }
            }
        }
    }

    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, start: usize) {
        let mut depth = 1;
        while let Some(c) = state.peek() {
            match c {
                '(' => {
                    state.advance(1);
                    if let Some('*') = state.peek() {
                        state.advance(1);
                        depth += 1;
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some(')') = state.peek() {
                        state.advance(1);
                        depth -= 1;
                        if depth == 0 {
                            state.add_token(CoqTokenType::Comment, start, state.get_position());
                            return;
                        }
                    }
                }
                _ => state.advance(1),
            }
        }
        state.add_token(CoqTokenType::Comment, start, state.get_position());
    }

    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, start: usize) {
        while let Some(c) = state.peek() {
            match c {
                '"' => {
                    state.advance(1);
                    if let Some('"') = state.peek() {
                        state.advance(1); // Escaped quote
                    }
                    else {
                        state.add_token(CoqTokenType::StringLiteral, start, state.get_position());
                        return;
                    }
                }
                _ => state.advance(1),
            }
        }
        state.add_token(CoqTokenType::StringLiteral, start, state.get_position());
    }
}

impl<'config> Lexer<CoqLanguage> for CoqLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<CoqLanguage>) -> LexOutput<CoqLanguage> {
        let mut state: State<'_, S> = LexerState::new(text);
        // TODO: Implement incremental lexing using edits and cache
        self.lex_internal(&mut state);
        state.add_eof();
        state.finish_with_cache(Ok(()), cache)
    }
}
