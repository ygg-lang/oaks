use crate::{kind::SchemeSyntaxKind, language::SchemeLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, SchemeLanguage>;

pub struct SchemeLexer<'config> {
    config: &'config SchemeLanguage,
}

impl<'config> SchemeLexer<'config> {
    pub fn new(config: &'config SchemeLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
    fn skip_whitespace(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(SchemeSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(SchemeSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(SchemeSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理行注
    fn lex_line_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(';') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            state.add_token(SchemeSyntaxKind::LineComment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理数字字面
    fn lex_number(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() || (ch == '-' || ch == '+') {
                // 处理符号
                if ch == '-' || ch == '+' {
                    state.advance(1);
                    if let Some(next_ch) = state.peek() {
                        if !next_ch.is_ascii_digit() {
                            state.set_position(start_pos);
                            return false;
                        }
                    }
                    else {
                        state.set_position(start_pos);
                        return false;
                    }
                }

                // 处理数字部分
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // 处理小数
                if let Some('.') = state.peek() {
                    state.advance(1);
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                // 处理科学计数
                if let Some('e') | Some('E') = state.peek() {
                    state.advance(1);
                    if let Some('+') | Some('-') = state.peek() {
                        state.advance(1);
                    }
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                state.add_token(SchemeSyntaxKind::NumberLiteral, start_pos, state.get_position());
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
    fn lex_string(&self, state: &mut State) -> bool {
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
                        state.advance(1);
                    }
                }
                else if ch == '\n' || ch == '\r' {
                    break; // 字符串不能跨
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            state.add_token(SchemeSyntaxKind::StringLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符字面
    fn lex_character(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            state.advance(1);
            if let Some('\\') = state.peek() {
                state.advance(1);

                // 处理特殊字符名称
                let char_start = state.get_position();
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphabetic() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // 如果没有读到字母，则读取一个字
                if state.get_position() == char_start {
                    if let Some(ch) = state.peek() {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(SchemeSyntaxKind::CharacterLiteral, start_pos, state.get_position());
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

    /// 处理标识符或关键
    fn lex_identifier_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if self.is_identifier_start(ch) {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if self.is_identifier_continue(ch) {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = source.get_text_in((start_pos..state.get_position()).into()).unwrap_or("");
                let token_kind = self.keyword_or_identifier(text);
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

    /// 判断字符是否可以作为标识符开
    fn is_identifier_start(&self, ch: char) -> bool {
        ch.is_ascii_alphabetic()
            || matches!(ch, '!' | '$' | '%' | '&' | '*' | '/' | ':' | '<' | '=' | '>' | '?' | '^' | '_' | '~' | '+' | '-')
    }

    /// 判断字符是否可以作为标识符继
    fn is_identifier_continue(&self, ch: char) -> bool {
        self.is_identifier_start(ch) || ch.is_ascii_digit() || ch == '.' || ch == '@'
    }

    /// 判断是关键字还是标识
    fn keyword_or_identifier(&self, text: &str) -> SchemeSyntaxKind {
        match text {
            "define" => SchemeSyntaxKind::Define,
            "lambda" => SchemeSyntaxKind::Lambda,
            "if" => SchemeSyntaxKind::If,
            "cond" => SchemeSyntaxKind::Cond,
            "case" => SchemeSyntaxKind::Case,
            "let" => SchemeSyntaxKind::Let,
            "let*" => SchemeSyntaxKind::LetStar,
            "letrec" => SchemeSyntaxKind::Letrec,
            "begin" => SchemeSyntaxKind::Begin,
            "do" => SchemeSyntaxKind::Do,
            "quote" => SchemeSyntaxKind::Quote,
            "quasiquote" => SchemeSyntaxKind::Quasiquote,
            "unquote" => SchemeSyntaxKind::Unquote,
            "unquote-splicing" => SchemeSyntaxKind::UnquoteSplicing,
            "and" => SchemeSyntaxKind::And,
            "or" => SchemeSyntaxKind::Or,
            "not" => SchemeSyntaxKind::Not,
            "set!" => SchemeSyntaxKind::Set,
            "#t" | "#f" => SchemeSyntaxKind::BooleanLiteral,
            _ => SchemeSyntaxKind::Identifier,
        }
    }

    /// 处理特殊符号
    fn lex_special_symbol(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '\'' => {
                    state.advance(1);
                    SchemeSyntaxKind::Quote_
                }
                '`' => {
                    state.advance(1);
                    SchemeSyntaxKind::Quasiquote_
                }
                ',' => {
                    state.advance(1);
                    if let Some('@') = state.peek() {
                        state.advance(1);
                        SchemeSyntaxKind::UnquoteSplicing_
                    }
                    else {
                        SchemeSyntaxKind::Unquote_
                    }
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
    fn lex_delimiter(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => SchemeSyntaxKind::LeftParen,
                ')' => SchemeSyntaxKind::RightParen,
                '[' => SchemeSyntaxKind::LeftBracket,
                ']' => SchemeSyntaxKind::RightBracket,
                '.' => SchemeSyntaxKind::Dot,
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

impl<'config> Lexer<SchemeLanguage> for SchemeLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<SchemeSyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_line_comment(&mut state) {
                continue;
            }

            if self.lex_character(&mut state) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_special_symbol(&mut state) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_delimiter(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(SchemeSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(SchemeSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
