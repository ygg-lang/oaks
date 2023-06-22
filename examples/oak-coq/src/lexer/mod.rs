use crate::{kind::CoqSyntaxKind, language::CoqLanguage};
use oak_core::{
    Lexer, LexerState, TextEdit,
    lexer::{LexOutput, LexerCache},
    source::Source,
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
                        state.advance(1);
                    }
                    state.add_token(CoqSyntaxKind::Whitespace, start, state.get_position());
                }
                '\n' => {
                    state.advance(1);
                    state.add_token(CoqSyntaxKind::Newline, start, state.get_position());
                }
                '(' => {
                    state.advance(1);
                    if let Some('*') = state.peek() {
                        state.advance(1);
                        self.lex_comment(state, start);
                    }
                    else {
                        state.add_token(CoqSyntaxKind::LeftParen, start, state.get_position());
                    }
                }
                ')' => {
                    state.advance(1);
                    state.add_token(CoqSyntaxKind::RightParen, start, state.get_position());
                }
                '[' => {
                    state.advance(1);
                    state.add_token(CoqSyntaxKind::LeftBracket, start, state.get_position());
                }
                ']' => {
                    state.advance(1);
                    state.add_token(CoqSyntaxKind::RightBracket, start, state.get_position());
                }
                '{' => {
                    state.advance(1);
                    state.add_token(CoqSyntaxKind::LeftBrace, start, state.get_position());
                }
                '}' => {
                    state.advance(1);
                    state.add_token(CoqSyntaxKind::RightBrace, start, state.get_position());
                }
                ',' => {
                    state.advance(1);
                    state.add_token(CoqSyntaxKind::Comma, start, state.get_position());
                }
                ';' => {
                    state.advance(1);
                    state.add_token(CoqSyntaxKind::Semicolon, start, state.get_position());
                }
                '.' => {
                    state.advance(1);
                    state.add_token(CoqSyntaxKind::Dot, start, state.get_position());
                }
                ':' => {
                    state.advance(1);
                    if let Some(':') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            state.add_token(CoqSyntaxKind::DoubleColonEqual, start, state.get_position());
                        }
                        else {
                            state.add_token(CoqSyntaxKind::DoubleColon, start, state.get_position());
                        }
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(CoqSyntaxKind::ColonEqual, start, state.get_position());
                    }
                    else {
                        state.add_token(CoqSyntaxKind::Colon, start, state.get_position());
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('>') = state.peek() {
                        state.advance(1);
                        state.add_token(CoqSyntaxKind::DoubleArrow, start, state.get_position());
                    }
                    else {
                        state.add_token(CoqSyntaxKind::Equal, start, state.get_position());
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('-') = state.peek() {
                        state.advance(1);
                        state.add_token(CoqSyntaxKind::LeftArrow, start, state.get_position());
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        state.add_token(CoqSyntaxKind::DoubleArrow, start, state.get_position());
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(CoqSyntaxKind::LessEqual, start, state.get_position());
                    }
                    else {
                        state.add_token(CoqSyntaxKind::Less, start, state.get_position());
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(CoqSyntaxKind::GreaterEqual, start, state.get_position());
                    }
                    else {
                        state.add_token(CoqSyntaxKind::Greater, start, state.get_position());
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('-') = state.peek() {
                        state.advance(1);
                        state.add_token(CoqSyntaxKind::Turnstile, start, state.get_position());
                    }
                    else {
                        state.add_token(CoqSyntaxKind::Pipe, start, state.get_position());
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('>') = state.peek() {
                        state.advance(1);
                        state.add_token(CoqSyntaxKind::Arrow, start, state.get_position());
                    }
                    else {
                        state.add_token(CoqSyntaxKind::Minus, start, state.get_position());
                    }
                }
                '+' => {
                    state.advance(1);
                    state.add_token(CoqSyntaxKind::Plus, start, state.get_position());
                }
                '*' => {
                    state.advance(1);
                    state.add_token(CoqSyntaxKind::Star, start, state.get_position());
                }
                '/' => {
                    state.advance(1);
                    if let Some('\\') = state.peek() {
                        state.advance(1);
                        state.add_token(CoqSyntaxKind::And, start, state.get_position());
                    }
                    else {
                        state.add_token(CoqSyntaxKind::Slash, start, state.get_position());
                    }
                }
                '\\' => {
                    state.advance(1);
                    if let Some('/') = state.peek() {
                        state.advance(1);
                        state.add_token(CoqSyntaxKind::Or, start, state.get_position());
                    }
                    else {
                        state.add_token(CoqSyntaxKind::Backslash, start, state.get_position());
                    }
                }
                '~' => {
                    state.advance(1);
                    state.add_token(CoqSyntaxKind::Tilde, start, state.get_position());
                }
                '!' => {
                    state.advance(1);
                    state.add_token(CoqSyntaxKind::Exclamation, start, state.get_position());
                }
                '?' => {
                    state.advance(1);
                    state.add_token(CoqSyntaxKind::Question, start, state.get_position());
                }
                '@' => {
                    state.advance(1);
                    state.add_token(CoqSyntaxKind::At, start, state.get_position());
                }
                '#' => {
                    state.advance(1);
                    state.add_token(CoqSyntaxKind::Hash, start, state.get_position());
                }
                '$' => {
                    state.advance(1);
                    state.add_token(CoqSyntaxKind::Dollar, start, state.get_position());
                }
                '%' => {
                    state.advance(1);
                    state.add_token(CoqSyntaxKind::Percent, start, state.get_position());
                }
                '^' => {
                    state.advance(1);
                    state.add_token(CoqSyntaxKind::Caret, start, state.get_position());
                }
                '&' => {
                    state.advance(1);
                    state.add_token(CoqSyntaxKind::Ampersand, start, state.get_position());
                }
                '"' => {
                    state.advance(1);
                    self.lex_string(state, start);
                }
                '0'..='9' => {
                    state.advance(1);
                    while let Some('0'..='9') = state.peek() {
                        state.advance(1);
                    }
                    state.add_token(CoqSyntaxKind::NumberLiteral, start, state.get_position());
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    state.advance(1);
                    while let Some('a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '\'') = state.peek() {
                        state.advance(1);
                    }
                    let end = state.get_position();
                    let text = state.get_text_in((start..end).into());
                    let kind = match text.as_ref() {
                        "Theorem" => CoqSyntaxKind::Theorem,
                        "Lemma" => CoqSyntaxKind::Lemma,
                        "Remark" => CoqSyntaxKind::Remark,
                        "Fact" => CoqSyntaxKind::Fact,
                        "Corollary" => CoqSyntaxKind::Corollary,
                        "Proposition" => CoqSyntaxKind::Proposition,
                        "Definition" => CoqSyntaxKind::Definition,
                        "Example" => CoqSyntaxKind::Example,
                        "Fixpoint" => CoqSyntaxKind::Fixpoint,
                        "CoFixpoint" => CoqSyntaxKind::CoFixpoint,
                        "Inductive" => CoqSyntaxKind::Inductive,
                        "CoInductive" => CoqSyntaxKind::CoInductive,
                        "Record" => CoqSyntaxKind::Record,
                        "Structure" => CoqSyntaxKind::Structure,
                        "Variant" => CoqSyntaxKind::Variant,
                        "Module" => CoqSyntaxKind::Module,
                        "Section" => CoqSyntaxKind::Section,
                        "End" => CoqSyntaxKind::End,
                        "Require" => CoqSyntaxKind::Require,
                        "Import" => CoqSyntaxKind::Import,
                        "Export" => CoqSyntaxKind::Export,
                        "Proof" => CoqSyntaxKind::Proof,
                        "Qed" => CoqSyntaxKind::Qed,
                        "Defined" => CoqSyntaxKind::Defined,
                        "Admitted" => CoqSyntaxKind::Admitted,
                        "Abort" => CoqSyntaxKind::Abort,
                        "Match" => CoqSyntaxKind::Match,
                        "With" => CoqSyntaxKind::With,
                        "Forall" => CoqSyntaxKind::Forall,
                        "Exists" => CoqSyntaxKind::Exists,
                        "Fun" => CoqSyntaxKind::Fun,
                        "Let" => CoqSyntaxKind::Let,
                        "In" => CoqSyntaxKind::In,
                        "If" => CoqSyntaxKind::If,
                        "Then" => CoqSyntaxKind::Then,
                        "Else" => CoqSyntaxKind::Else,
                        "Type" => CoqSyntaxKind::Type,
                        "Prop" => CoqSyntaxKind::Prop,
                        "Set" => CoqSyntaxKind::Set,
                        "Check" => CoqSyntaxKind::Check,
                        "Print" => CoqSyntaxKind::Print,
                        "Search" => CoqSyntaxKind::Search,
                        "Locate" => CoqSyntaxKind::Locate,
                        "About" => CoqSyntaxKind::About,
                        _ => CoqSyntaxKind::Identifier,
                    };
                    state.add_token(kind, start, end);
                }
                _ => {
                    state.advance(1);
                    state.add_token(CoqSyntaxKind::Error, start, state.get_position());
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
                            state.add_token(CoqSyntaxKind::Comment, start, state.get_position());
                            return;
                        }
                    }
                }
                _ => {
                    state.advance(1);
                }
            }
        }
        state.add_token(CoqSyntaxKind::Comment, start, state.get_position());
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
                        state.add_token(CoqSyntaxKind::StringLiteral, start, state.get_position());
                        return;
                    }
                }
                _ => {
                    state.advance(1);
                }
            }
        }
        state.add_token(CoqSyntaxKind::StringLiteral, start, state.get_position());
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
