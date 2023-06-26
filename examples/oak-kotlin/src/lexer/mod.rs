#![doc = include_str!("readme.md")]

/// Token types for the Kotlin lexer.
pub mod token_type;

use crate::{language::KotlinLanguage, lexer::token_type::KotlinTokenType};
use oak_core::{
    Lexer, LexerState, OakError, Source, TextEdit,
    lexer::{LexOutput, LexerCache},
};

pub(crate) type State<'a, S> = LexerState<'a, S, KotlinLanguage>;

trait LexerStateExt {
    fn eat(&mut self, ch: char) -> bool;
}

impl<'a, S: Source + ?Sized> LexerStateExt for State<'a, S> {
    fn eat(&mut self, ch: char) -> bool {
        if self.peek() == Some(ch) {
            self.advance(ch.len_utf8());
            true
        }
        else {
            false
        }
    }
}

/// A lexer for the Kotlin language.
#[derive(Clone)]
pub struct KotlinLexer<'config> {
    config: &'config KotlinLanguage,
}

impl<'config> KotlinLexer<'config> {
    /// Creates a new Kotlin lexer.
    pub fn new(config: &'config KotlinLanguage) -> Self {
        Self { config }
    }

    /// Skips whitespace characters.
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' { state.advance(ch.len_utf8()) } else { break }
        }

        if state.get_position() > start_pos {
            state.add_token(KotlinTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles newlines.
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(KotlinTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1)
            }
            state.add_token(KotlinTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles comments.
    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            state.advance(1);
            if let Some('/') = state.peek() {
                // Single-line comment
                state.advance(1);
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                state.add_token(KotlinTokenType::Comment, start_pos, state.get_position());
                true
            }
            else if let Some('*') = state.peek() {
                // Multi-line comment
                state.advance(1);
                let mut depth = 1;
                while depth > 0 && state.not_at_end() {
                    if let Some('/') = state.peek() {
                        state.advance(1);
                        if let Some('*') = state.peek() {
                            state.advance(1);
                            depth += 1;
                        }
                    }
                    else if let Some('*') = state.peek() {
                        state.advance(1);
                        if let Some('/') = state.peek() {
                            state.advance(1);
                            depth -= 1;
                        }
                    }
                    else if let Some(ch) = state.peek() {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                state.add_token(KotlinTokenType::Comment, start_pos, state.get_position());
                true
            }
            else {
                // Backtrack, this is a division operator
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// Handles string literals.
    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1);

            // Check if it's a triple-quoted string
            if let Some('"') = state.peek() {
                state.advance(1);
                if let Some('"') = state.peek() {
                    // Triple-quoted string
                    state.advance(1);
                    while state.not_at_end() {
                        if let Some('"') = state.peek() {
                            state.advance(1);
                            if let Some('"') = state.peek() {
                                state.advance(1);
                                if let Some('"') = state.peek() {
                                    state.advance(1);
                                    break;
                                }
                            }
                        }
                        else if let Some(ch) = state.peek() {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                    state.add_token(KotlinTokenType::StringLiteral, start_pos, state.get_position());
                    return true;
                }
                else {
                    // Empty string
                    state.add_token(KotlinTokenType::StringLiteral, start_pos, state.get_position());
                    return true;
                }
            }

            // Normal string
            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    break;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if let Some(escaped) = state.peek() {
                        state.advance(escaped.len_utf8())
                    }
                }
                else if ch == '\n' || ch == '\r' {
                    break; // String cannot span lines
                }
                else {
                    state.advance(ch.len_utf8())
                }
            }
            state.add_token(KotlinTokenType::StringLiteral, start_pos, state.get_position());
            true
        }
        else if let Some('\'') = state.peek() {
            // Character literal
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '\'' {
                    state.advance(1);
                    break;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if let Some(escaped) = state.peek() {
                        state.advance(escaped.len_utf8());
                    }
                }
                else if ch == '\n' || ch == '\r' {
                    break;
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }
            state.add_token(KotlinTokenType::CharLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles number literals.
    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '_' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // Handle fractional part
                if let Some('.') = state.peek() {
                    // Lookahead to check if the next character is a digit (float) or something else (method call)
                    if let Some(next) = state.peek_next_n(1) {
                        if next.is_ascii_digit() {
                            state.advance(1);
                            while let Some(ch) = state.peek() {
                                if ch.is_ascii_digit() || ch == '_' {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }
                        }
                    }
                }

                // Handle exponent part
                if let Some('e') | Some('E') = state.peek() {
                    state.advance(1);
                    if let Some('+') | Some('-') = state.peek() {
                        state.advance(1);
                    }
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() || ch == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                state.add_token(KotlinTokenType::NumberLiteral, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// Handles identifiers and keywords.
    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' || ch == '$' {
                state.advance(ch.len_utf8());
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '$' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_in((start_pos..state.get_position()).into());
                let kind = match text.as_ref() {
                    "abstract" => KotlinTokenType::Abstract,
                    "annotation" => KotlinTokenType::Annotation,
                    "as" => KotlinTokenType::As,
                    "break" => KotlinTokenType::Break,
                    "catch" => KotlinTokenType::Catch,
                    "class" => KotlinTokenType::Class,
                    "companion" => KotlinTokenType::Companion,
                    "continue" => KotlinTokenType::Continue,
                    "crossinline" => KotlinTokenType::Crossinline,
                    "data" => KotlinTokenType::Data,
                    "else" => KotlinTokenType::Else,
                    "enum" => KotlinTokenType::Enum,
                    "external" => KotlinTokenType::External,
                    "false" => KotlinTokenType::False,
                    "final" => KotlinTokenType::Final,
                    "finally" => KotlinTokenType::Finally,
                    "for" => KotlinTokenType::For,
                    "fun" => KotlinTokenType::Fun,
                    "if" => KotlinTokenType::If,
                    "import" => KotlinTokenType::Import,
                    "in" => KotlinTokenType::In,
                    "infix" => KotlinTokenType::Infix,
                    "inline" => KotlinTokenType::Inline,
                    "interface" => KotlinTokenType::Interface,
                    "internal" => KotlinTokenType::Internal,
                    "is" => KotlinTokenType::Is,
                    "noinline" => KotlinTokenType::Noinline,
                    "null" => KotlinTokenType::Null,
                    "object" => KotlinTokenType::Object,
                    "open" => KotlinTokenType::Open,
                    "operator" => KotlinTokenType::Operator,
                    "out" => KotlinTokenType::Out,
                    "override" => KotlinTokenType::Override,
                    "package" => KotlinTokenType::Package,
                    "private" => KotlinTokenType::Private,
                    "protected" => KotlinTokenType::Protected,
                    "public" => KotlinTokenType::Public,
                    "reified" => KotlinTokenType::Reified,
                    "return" => KotlinTokenType::Return,
                    "sealed" => KotlinTokenType::Sealed,
                    "super" => KotlinTokenType::Super,
                    "suspend" => KotlinTokenType::Suspend,
                    "tailrec" => KotlinTokenType::Tailrec,
                    "this" => KotlinTokenType::This,
                    "throw" => KotlinTokenType::Throw,
                    "true" => KotlinTokenType::True,
                    "try" => KotlinTokenType::Try,
                    "val" => KotlinTokenType::Val,
                    "var" => KotlinTokenType::Var,
                    "vararg" => KotlinTokenType::Vararg,
                    "when" => KotlinTokenType::When,
                    "while" => KotlinTokenType::While,
                    _ => KotlinTokenType::Identifier,
                };

                state.add_token(kind, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// Handles special characters and operators.
    fn lex_special_char<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '(' => {
                    state.advance(1);
                    KotlinTokenType::LParen
                }
                ')' => {
                    state.advance(1);
                    KotlinTokenType::RParen
                }
                '{' => {
                    state.advance(1);
                    KotlinTokenType::LBrace
                }
                '}' => {
                    state.advance(1);
                    KotlinTokenType::RBrace
                }
                '[' => {
                    state.advance(1);
                    KotlinTokenType::LBracket
                }
                ']' => {
                    state.advance(1);
                    KotlinTokenType::RBracket
                }
                ',' => {
                    state.advance(1);
                    KotlinTokenType::Comma
                }
                ';' => {
                    state.advance(1);
                    KotlinTokenType::Semi
                }
                ':' => {
                    state.advance(1);
                    if state.eat(':') { KotlinTokenType::DoubleColon } else { KotlinTokenType::Colon }
                }
                '.' => {
                    state.advance(1);
                    if state.eat('.') { KotlinTokenType::Range } else { KotlinTokenType::Dot }
                }
                '?' => {
                    state.advance(1);
                    KotlinTokenType::Question
                }
                '!' => {
                    state.advance(1);
                    if state.eat('!') {
                        KotlinTokenType::ExclamationExclamation
                    }
                    else if state.eat('=') {
                        KotlinTokenType::NotEq
                    }
                    else {
                        KotlinTokenType::Exclamation
                    }
                }
                '+' => {
                    state.advance(1);
                    if state.eat('=') { KotlinTokenType::PlusAssign } else { KotlinTokenType::Plus }
                }
                '-' => {
                    state.advance(1);
                    if state.eat('=') {
                        KotlinTokenType::MinusAssign
                    }
                    else if state.eat('>') {
                        KotlinTokenType::Arrow
                    }
                    else {
                        KotlinTokenType::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if state.eat('=') { KotlinTokenType::StarAssign } else { KotlinTokenType::Star }
                }
                '/' => {
                    state.advance(1);
                    if state.eat('=') { KotlinTokenType::SlashAssign } else { KotlinTokenType::Slash }
                }
                '%' => {
                    state.advance(1);
                    if state.eat('=') { KotlinTokenType::PercentAssign } else { KotlinTokenType::Percent }
                }
                '=' => {
                    state.advance(1);
                    if state.eat('=') { KotlinTokenType::EqEq } else { KotlinTokenType::Assign }
                }
                '<' => {
                    state.advance(1);
                    if state.eat('=') { KotlinTokenType::LtEq } else { KotlinTokenType::Less }
                }
                '>' => {
                    state.advance(1);
                    if state.eat('=') { KotlinTokenType::GtEq } else { KotlinTokenType::Greater }
                }
                '&' => {
                    state.advance(1);
                    if state.eat('&') { KotlinTokenType::AndAnd } else { KotlinTokenType::Ampersand }
                }
                '|' => {
                    state.advance(1);
                    if state.eat('|') { KotlinTokenType::OrOr } else { KotlinTokenType::Pipe }
                }
                '^' => {
                    state.advance(1);
                    KotlinTokenType::Caret
                }
                '~' => {
                    state.advance(1);
                    KotlinTokenType::Tilde
                }
                '@' => {
                    state.advance(1);
                    KotlinTokenType::At
                }
                _ => return false,
            };

            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<KotlinLanguage> for KotlinLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<KotlinLanguage>) -> LexOutput<KotlinLanguage> {
        let mut state = State::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> KotlinLexer<'config> {
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.lex_comment(state) {
                continue;
            }

            if self.lex_string(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_special_char(state) {
                continue;
            }

            // If no rules match, skip current character and mark as error.
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(KotlinTokenType::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }
}
