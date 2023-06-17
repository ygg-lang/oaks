use crate::{kind::SassSyntaxKind, language::SassLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, SassLanguage>;

pub struct SassLexer<'config> {
    config: &'config SassLanguage,
}

impl<'config> SassLexer<'config> {
    pub fn new(config: &'config SassLanguage) -> Self {
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
            state.add_token(SassSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(SassSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(SassSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理行注释
    fn lex_line_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            if let Some('/') = state.peek_next_n(1) {
                state.advance(2);

                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(SassSyntaxKind::LineComment, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理块注释
    fn lex_block_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            if let Some('*') = state.peek_next_n(1) {
                state.advance(2);

                while let Some(ch) = state.peek() {
                    if ch == '*' && state.peek_next_n(1) == Some('/') {
                        state.advance(2);
                        break;
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(SassSyntaxKind::BlockComment, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理标识符和关键字
    fn lex_identifier_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = source.get_text_in(core::range::Range { start: start_pos, end: state.get_position() }).unwrap_or("");
                let token_kind = match text {
                    "@import" | "import" => SassSyntaxKind::Import,
                    "@include" | "include" => SassSyntaxKind::Include,
                    "@extend" | "extend" => SassSyntaxKind::Extend,
                    "@mixin" | "mixin" => SassSyntaxKind::Mixin,
                    "@function" | "function" => SassSyntaxKind::Function,
                    "return" => SassSyntaxKind::Return,
                    "@if" | "if" => SassSyntaxKind::If,
                    "@else" | "else" => SassSyntaxKind::Else,
                    "@else if" | "else if" => SassSyntaxKind::ElseIf,
                    "@for" | "for" => SassSyntaxKind::For,
                    "@each" | "each" => SassSyntaxKind::Each,
                    "@while" | "while" => SassSyntaxKind::While,
                    "default" => SassSyntaxKind::Default,
                    "important" => SassSyntaxKind::Important,
                    "optional" => SassSyntaxKind::Optional,
                    "global" => SassSyntaxKind::Global,
                    "and" => SassSyntaxKind::And,
                    "or" => SassSyntaxKind::Or,
                    "not" => SassSyntaxKind::Not,
                    _ => SassSyntaxKind::Identifier,
                };

                state.add_token(token_kind, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理变量
    fn lex_variable(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('$') = state.peek() {
            state.advance(1);

            if let Some(ch) = state.peek() {
                if ch.is_ascii_alphabetic() || ch == '_' {
                    state.advance(ch.len_utf8());

                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }

                    state.add_token(SassSyntaxKind::Variable, start_pos, state.get_position());
                    return true;
                }
            }
            // 回退
            state.set_position(start_pos);
        }
        false
    }

    /// 处理数字
    fn lex_number(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);

                // 读取数字部分
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // 检查是否为浮点数
                if state.peek() == Some('.') {
                    if let Some(next_ch) = state.peek_next_n(1) {
                        if next_ch.is_ascii_digit() {
                            state.advance(1); // 跳过 '.'

                            // 读取小数部分
                            while let Some(ch) = state.peek() {
                                if ch.is_ascii_digit() {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }
                        }
                    }
                }

                // 检查单位
                if let Some(ch) = state.peek() {
                    if ch.is_ascii_alphabetic() {
                        let unit_start = state.get_position();
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_alphabetic() {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                        state.add_token(SassSyntaxKind::Unit, unit_start, state.get_position());
                    }
                }

                state.add_token(SassSyntaxKind::NumberLiteral, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理字符串
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1);
                        break;
                    }
                    else if ch == '\\' {
                        state.advance(1);
                        if let Some(_) = state.peek() {
                            state.advance(1);
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(SassSyntaxKind::StringLiteral, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理颜色字面量
    fn lex_color(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            state.advance(1);

            let mut hex_count = 0;
            while let Some(ch) = state.peek() {
                if ch.is_ascii_hexdigit() {
                    state.advance(1);
                    hex_count += 1;
                }
                else {
                    break;
                }
            }

            if hex_count == 3 || hex_count == 6 || hex_count == 8 {
                state.add_token(SassSyntaxKind::ColorLiteral, start_pos, state.get_position());
                return true;
            }
            else {
                // 回退，这不是颜色
                state.set_position(start_pos);
            }
        }
        false
    }

    /// 处理操作符
    fn lex_operator(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => SassSyntaxKind::Plus,
                '-' => SassSyntaxKind::Minus,
                '*' => SassSyntaxKind::Star,
                '/' => SassSyntaxKind::Slash,
                '%' => SassSyntaxKind::Percent,
                '=' => {
                    if state.peek_next_n(1) == Some('=') {
                        state.advance(1);
                        SassSyntaxKind::EqEq
                    }
                    else {
                        SassSyntaxKind::Eq
                    }
                }
                '!' => {
                    if state.peek_next_n(1) == Some('=') {
                        state.advance(1);
                        SassSyntaxKind::Ne
                    }
                    else {
                        SassSyntaxKind::Exclamation
                    }
                }
                '<' => {
                    if state.peek_next_n(1) == Some('=') {
                        state.advance(1);
                        SassSyntaxKind::Le
                    }
                    else {
                        SassSyntaxKind::Lt
                    }
                }
                '>' => {
                    if state.peek_next_n(1) == Some('=') {
                        state.advance(1);
                        SassSyntaxKind::Ge
                    }
                    else {
                        SassSyntaxKind::Gt
                    }
                }
                '&' => SassSyntaxKind::Ampersand,
                '~' => SassSyntaxKind::Tilde,
                _ => return false,
            };

            state.advance(1);
            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理分隔符和标点
    fn lex_delimiter(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => SassSyntaxKind::LeftParen,
                ')' => SassSyntaxKind::RightParen,
                '[' => SassSyntaxKind::LeftBracket,
                ']' => SassSyntaxKind::RightBracket,
                '{' => SassSyntaxKind::LeftBrace,
                '}' => SassSyntaxKind::RightBrace,
                ';' => SassSyntaxKind::Semicolon,
                ':' => SassSyntaxKind::Colon,
                ',' => SassSyntaxKind::Comma,
                '.' => SassSyntaxKind::Dot,
                '#' => SassSyntaxKind::Hash,
                '$' => SassSyntaxKind::Dollar,
                '@' => SassSyntaxKind::At,
                '?' => SassSyntaxKind::Question,
                _ => return false,
            };

            state.advance(1);
            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<SassLanguage> for SassLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<SassSyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则，按优先级排序
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_line_comment(&mut state) {
                continue;
            }

            if self.lex_block_comment(&mut state) {
                continue;
            }

            if self.lex_variable(&mut state) {
                continue;
            }

            if self.lex_color(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_operator(&mut state) {
                continue;
            }

            if self.lex_delimiter(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(SassSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(SassSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
