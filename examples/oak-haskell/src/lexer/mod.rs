#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::HaskellLanguage, lexer::token_type::HaskellTokenType};
use oak_core::{Lexer, LexerCache, LexerState, TextEdit, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, HaskellLanguage>;

#[derive(Clone)]
pub struct HaskellLexer<'config> {
    _config: &'config HaskellLanguage,
}

impl<'config> HaskellLexer<'config> {
    pub fn new(config: &'config HaskellLanguage) -> Self {
        Self { _config: config }
    }

    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();
        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.bump();
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(HaskellTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.bump();
            state.add_token(HaskellTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.bump();
            if let Some('\n') = state.peek() {
                state.bump();
            }
            state.add_token(HaskellTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_single_line_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('-') = state.peek() {
            if let Some('-') = state.peek_next_n(1) {
                state.advance(2);
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.bump();
                }
                state.add_token(HaskellTokenType::Comment, start_pos, state.get_position());
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

    fn lex_multi_line_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('{') = state.peek() {
            if let Some('-') = state.peek_next_n(1) {
                state.advance(2);
                let mut depth = 1;
                while let Some(ch) = state.peek() {
                    if ch == '{' && state.peek_next_n(1) == Some('-') {
                        depth += 1;
                        state.advance(2)
                    }
                    else if ch == '-' && state.peek_next_n(1) == Some('}') {
                        depth -= 1;
                        state.advance(2);
                        if depth == 0 {
                            break;
                        }
                    }
                    else {
                        state.bump();
                    }
                }
                state.add_token(HaskellTokenType::Comment, start_pos, state.get_position());
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

    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.bump();

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '\'' {
                        state.bump();
                    }
                    else {
                        break;
                    }
                }

                let end_pos = state.get_position();
                let text = state.get_text_in((start_pos..end_pos).into());
                let kind = self.keyword_or_identifier(text.as_ref());

                state.add_token(kind, start_pos, end_pos);
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

    fn keyword_or_identifier(&self, text: &str) -> HaskellTokenType {
        match text {
            "case" => HaskellTokenType::Case,
            "class" => HaskellTokenType::Class,
            "data" => HaskellTokenType::Data,
            "default" => HaskellTokenType::Default,
            "deriving" => HaskellTokenType::Deriving,
            "do" => HaskellTokenType::Do,
            "else" => HaskellTokenType::Else,
            "if" => HaskellTokenType::If,
            "import" => HaskellTokenType::Import,
            "in" => HaskellTokenType::In,
            "infix" => HaskellTokenType::Infix,
            "infixl" => HaskellTokenType::Infixl,
            "infixr" => HaskellTokenType::Infixr,
            "instance" => HaskellTokenType::Instance,
            "let" => HaskellTokenType::Let,
            "module" => HaskellTokenType::Module,
            "newtype" => HaskellTokenType::Newtype,
            "of" => HaskellTokenType::Of,
            "then" => HaskellTokenType::Then,
            "type" => HaskellTokenType::Type,
            "where" => HaskellTokenType::Where,
            _ => HaskellTokenType::Identifier,
        }
    }

    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.bump();

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.bump();
                    }
                    else if ch == '.' {
                        state.bump();
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() {
                                state.bump();
                            }
                            else {
                                break;
                            }
                        }
                        break;
                    }
                    else {
                        break;
                    }
                }

                state.add_token(HaskellTokenType::Number, start_pos, state.get_position());
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

    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.bump();

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.bump();
                    state.add_token(HaskellTokenType::StringLiteral, start_pos, state.get_position());
                    return true;
                }
                else if ch == '\\' {
                    state.bump();
                    if let Some(_) = state.peek() {
                        state.bump();
                    }
                }
                else {
                    state.bump();
                }
            }

            state.add_token(HaskellTokenType::StringLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_char<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
            state.bump();

            if let Some(ch) = state.peek() {
                if ch == '\\' {
                    state.bump();
                    if let Some(_) = state.peek() {
                        state.bump();
                    }
                }
                else if ch != '\'' {
                    state.bump();
                }
            }

            if let Some('\'') = state.peek() {
                state.bump();
                state.add_token(HaskellTokenType::CharLiteral, start_pos, state.get_position());
                true
            }
            else {
                state.add_token(HaskellTokenType::CharLiteral, start_pos, state.get_position());
                true
            }
        }
        else {
            false
        }
    }

    fn lex_operators<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.bump();
                    if let Some('+') = state.peek() {
                        state.bump();
                        HaskellTokenType::Append
                    }
                    else {
                        HaskellTokenType::Plus
                    }
                }
                '-' => {
                    state.bump();
                    if let Some('>') = state.peek() {
                        state.bump();
                        HaskellTokenType::Arrow
                    }
                    else {
                        HaskellTokenType::Minus
                    }
                }
                '*' => {
                    state.bump();
                    HaskellTokenType::Star
                }
                '/' => {
                    state.bump();
                    HaskellTokenType::Slash
                }
                '=' => {
                    state.bump();
                    if let Some('=') = state.peek() {
                        state.bump();
                        HaskellTokenType::Equal
                    }
                    else {
                        HaskellTokenType::Assign
                    }
                }
                '<' => {
                    state.bump();
                    if let Some('=') = state.peek() {
                        state.bump();
                        HaskellTokenType::LessEqual
                    }
                    else if let Some('-') = state.peek() {
                        state.bump();
                        HaskellTokenType::LeftArrow
                    }
                    else {
                        HaskellTokenType::Less
                    }
                }
                '>' => {
                    state.bump();
                    if let Some('=') = state.peek() {
                        state.bump();
                        HaskellTokenType::GreaterEqual
                    }
                    else {
                        HaskellTokenType::Greater
                    }
                }
                ':' => {
                    state.bump();
                    if let Some(':') = state.peek() {
                        state.bump();
                        HaskellTokenType::DoubleColon
                    }
                    else {
                        HaskellTokenType::Colon
                    }
                }
                '|' => {
                    state.bump();
                    HaskellTokenType::Pipe
                }
                '&' => {
                    state.bump();
                    HaskellTokenType::Ampersand
                }
                '!' => {
                    state.bump();
                    HaskellTokenType::Bang
                }
                '?' => {
                    state.bump();
                    HaskellTokenType::Question
                }
                ';' => {
                    state.bump();
                    HaskellTokenType::Semicolon
                }
                ',' => {
                    state.bump();
                    HaskellTokenType::Comma
                }
                '.' => {
                    state.bump();
                    if let Some('.') = state.peek() {
                        state.bump();
                        HaskellTokenType::DoubleDot
                    }
                    else {
                        HaskellTokenType::Dot
                    }
                }
                '$' => {
                    state.bump();
                    HaskellTokenType::Dollar
                }
                '@' => {
                    state.bump();
                    HaskellTokenType::At
                }
                '~' => {
                    state.bump();
                    HaskellTokenType::Tilde
                }
                '\\' => {
                    state.bump();
                    HaskellTokenType::Backslash
                }
                '`' => {
                    state.bump();
                    HaskellTokenType::Backtick
                }
                _ => return false,
            };

            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_delimiters<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => {
                    state.bump();
                    HaskellTokenType::LeftParen
                }
                ')' => {
                    state.bump();
                    HaskellTokenType::RightParen
                }
                '[' => {
                    state.bump();
                    HaskellTokenType::LeftBracket
                }
                ']' => {
                    state.bump();
                    HaskellTokenType::RightBracket
                }
                '{' => {
                    state.bump();
                    HaskellTokenType::LeftBrace
                }
                '}' => {
                    state.bump();
                    HaskellTokenType::RightBrace
                }
                _ => return false,
            };

            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<HaskellLanguage> for HaskellLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<HaskellLanguage>) -> LexOutput<HaskellLanguage> {
        let mut state = State::new(source);

        while state.not_at_end() {
            let safe_point = state.get_position();
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_single_line_comment(&mut state) {
                continue;
            }

            if self.lex_multi_line_comment(&mut state) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_char(&mut state) {
                continue;
            }

            if self.lex_operators(&mut state) {
                continue;
            }

            if self.lex_delimiters(&mut state) {
                continue;
            }

            // 如果没有匹配到任何模式，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if state.peek().is_some() {
                state.advance(1);
                state.add_token(HaskellTokenType::Error, start_pos, state.get_position())
            }

            state.advance_if_dead_lock(safe_point)
        }

        // 添加 EOF token
        let pos = state.get_position();
        state.add_token(HaskellTokenType::Eof, pos, pos);

        state.finish_with_cache(Ok(()), cache)
    }
}
