#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::PurescriptLanguage, lexer::token_type::PurescriptTokenType};
use oak_core::{Lexer, LexerCache, LexerState, OakError, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, PurescriptLanguage>;

#[derive(Clone)]
pub struct PurescriptLexer<'config> {
    _config: &'config PurescriptLanguage,
}

impl<'config> Lexer<PurescriptLanguage> for PurescriptLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<PurescriptLanguage>) -> LexOutput<PurescriptLanguage> {
        let mut state = State::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> PurescriptLexer<'config> {
    /// 创建一个新的 PurescriptLexer
    pub fn new(config: &'config PurescriptLanguage) -> Self {
        Self { _config: config }
    }

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

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_char_literal(state) {
                continue;
            }

            if self.lex_operator(state) {
                continue;
            }

            if self.lex_delimiter(state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(PurescriptTokenType::Error, start_pos, state.get_position())
            }

            state.advance_if_dead_lock(safe_point)
        }

        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' { state.advance(ch.len_utf8()) } else { break }
        }

        if state.get_position() > start_pos {
            state.add_token(PurescriptTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(PurescriptTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1)
            }
            state.add_token(PurescriptTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('-') = state.peek() {
            state.advance(1);
            if let Some('-') = state.peek() {
                // 单行注释
                state.advance(1);
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8())
                }
                state.add_token(PurescriptTokenType::Comment, start_pos, state.get_position());
                true
            }
            else {
                state.set_position(start_pos);
                false
            }
        }
        else if let Some('{') = state.peek() {
            state.advance(1);
            if let Some('-') = state.peek() {
                // 多行注释
                state.advance(1);
                let mut depth = 1;
                while let Some(ch) = state.peek() {
                    if ch == '{' {
                        state.advance(1);
                        if let Some('-') = state.peek() {
                            depth += 1;
                            state.advance(1)
                        }
                    }
                    else if ch == '-' {
                        state.advance(1);
                        if let Some('}') = state.peek() {
                            depth -= 1;
                            state.advance(1);
                            if depth == 0 {
                                break;
                            }
                        }
                    }
                    else {
                        state.advance(ch.len_utf8())
                    }
                }
                state.add_token(PurescriptTokenType::Comment, start_pos, state.get_position());
                true
            }
            else {
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理标识符或关键字
    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '\'' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 检查是否为关键字
                let text = state.get_text_in((start_pos..state.get_position()).into());

                let token_kind = match text.as_ref() {
                    "ado" => PurescriptTokenType::Ado,
                    "case" => PurescriptTokenType::Case,
                    "class" => PurescriptTokenType::Class,
                    "data" => PurescriptTokenType::Data,
                    "derive" => PurescriptTokenType::Derive,
                    "do" => PurescriptTokenType::Do,
                    "else" => PurescriptTokenType::Else,
                    "false" => PurescriptTokenType::False,
                    "forall" => PurescriptTokenType::Forall,
                    "foreign" => PurescriptTokenType::Foreign,
                    "if" => PurescriptTokenType::If,
                    "import" => PurescriptTokenType::Import,
                    "in" => PurescriptTokenType::In,
                    "infix" => PurescriptTokenType::Infix,
                    "infixl" => PurescriptTokenType::Infixl,
                    "infixr" => PurescriptTokenType::Infixr,
                    "instance" => PurescriptTokenType::Instance,
                    "let" => PurescriptTokenType::Let,
                    "module" => PurescriptTokenType::Module,
                    "newtype" => PurescriptTokenType::Newtype,
                    "of" => PurescriptTokenType::Of,
                    "then" => PurescriptTokenType::Then,
                    "true" => PurescriptTokenType::True,
                    "type" => PurescriptTokenType::Type,
                    "where" => PurescriptTokenType::Where,
                    _ => PurescriptTokenType::Identifier,
                };
                state.add_token(token_kind, start_pos, state.get_position());
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

    /// 处理数字字面
    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);

                // 处理十六进制数字
                if ch == '0' {
                    if let Some('x') | Some('X') = state.peek() {
                        state.advance(1);
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_hexdigit() {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                    else {
                        // 处理普通数
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() { state.advance(1) } else { break }
                        }
                    }
                }
                else {
                    // 处理十进制数
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() { state.advance(1) } else { break }
                    }
                }

                // 处理小数
                if let Some('.') = state.peek() {
                    state.advance(1);
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() { state.advance(1) } else { break }
                    }
                }

                // 处理指数
                if let Some('e') | Some('E') = state.peek() {
                    state.advance(1);
                    if let Some('+') | Some('-') = state.peek() {
                        state.advance(1)
                    }
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() { state.advance(1) } else { break }
                    }
                }

                state.add_token(PurescriptTokenType::NumberLiteral, start_pos, state.get_position());
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

    /// 处理字符串字面量
    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    break;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if let Some(_) = state.peek() {
                        state.advance(1)
                    }
                }
                else if ch == '\n' || ch == '\r' {
                    break; // 字符串不能跨行
                }
                else {
                    state.advance(ch.len_utf8())
                }
            }

            state.add_token(PurescriptTokenType::StringLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符字面
    fn lex_char_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
            state.advance(1);

            if let Some(ch) = state.peek() {
                if ch == '\\' {
                    state.advance(1);
                    if let Some(_) = state.peek() {
                        state.advance(1)
                    }
                }
                else if ch != '\'' {
                    state.advance(ch.len_utf8())
                }
            }

            if let Some('\'') = state.peek() {
                state.advance(1);
                state.add_token(PurescriptTokenType::CharLiteral, start_pos, state.get_position());
                true
            }
            else {
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理操作
    fn lex_operator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    PurescriptTokenType::Plus
                }
                '-' => {
                    state.advance(1);
                    if let Some('>') = state.peek() {
                        state.advance(1);
                        PurescriptTokenType::Arrow
                    }
                    else {
                        PurescriptTokenType::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('*') = state.peek() {
                        state.advance(1);
                        PurescriptTokenType::Caret // 使用 Caret 代替 Power
                    }
                    else {
                        PurescriptTokenType::Star
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PurescriptTokenType::NotEqual
                    }
                    else {
                        PurescriptTokenType::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    PurescriptTokenType::Percent
                }
                '=' => {
                    state.advance(1);
                    match state.peek() {
                        Some('=') => {
                            state.advance(1);
                            PurescriptTokenType::Equal
                        }
                        Some('>') => {
                            state.advance(1);
                            PurescriptTokenType::FatArrow
                        }
                        _ => PurescriptTokenType::Equal,
                    }
                }
                '<' => {
                    state.advance(1);
                    match state.peek() {
                        Some('=') => {
                            state.advance(1);
                            PurescriptTokenType::LessEqual
                        }
                        Some('-') => {
                            state.advance(1);
                            PurescriptTokenType::Bind
                        }
                        _ => PurescriptTokenType::Less,
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PurescriptTokenType::GreaterEqual
                    }
                    else {
                        PurescriptTokenType::Greater
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        PurescriptTokenType::And
                    }
                    else {
                        return false;
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        PurescriptTokenType::Or
                    }
                    else {
                        PurescriptTokenType::Pipe
                    }
                }
                '\\' => {
                    state.advance(1);
                    PurescriptTokenType::Backslash
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

    /// 处理分隔
    fn lex_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => PurescriptTokenType::LeftParen,
                ')' => PurescriptTokenType::RightParen,
                '[' => PurescriptTokenType::LeftBracket,
                ']' => PurescriptTokenType::RightBracket,
                '{' => PurescriptTokenType::LeftBrace,
                '}' => PurescriptTokenType::RightBrace,
                ',' => PurescriptTokenType::Comma,
                ';' => PurescriptTokenType::Semicolon,
                '.' => PurescriptTokenType::Dot,
                ':' => {
                    state.advance(1);
                    if let Some(':') = state.peek() {
                        state.advance(1);
                        state.add_token(PurescriptTokenType::ColonColon, start_pos, state.get_position());
                        return true;
                    }
                    else {
                        state.add_token(PurescriptTokenType::Colon, start_pos, state.get_position());
                        return true;
                    }
                }
                '?' => PurescriptTokenType::Question,
                '_' => PurescriptTokenType::Underscore,
                '@' => PurescriptTokenType::At,
                '`' => PurescriptTokenType::Tick,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}
