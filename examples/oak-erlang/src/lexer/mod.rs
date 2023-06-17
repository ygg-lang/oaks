use crate::{kind::ErlangSyntaxKind, language::ErlangLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, ErlangLanguage>;

pub struct ErlangLexer<'config> {
    config: &'config ErlangLanguage,
}

impl<'config> ErlangLexer<'config> {
    pub fn new(config: &'config ErlangLanguage) -> Self {
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
            state.add_token(ErlangSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(ErlangSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(ErlangSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('%') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(ErlangSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理标识符、原子或关键
    fn lex_identifier_atom_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_lowercase() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '@' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let end_pos = state.get_position();
                let text = source.get_text_in(core::range::Range { start: start_pos, end: end_pos }).unwrap_or("");

                let token_kind = match text {
                    "after" => ErlangSyntaxKind::After,
                    "and" => ErlangSyntaxKind::And,
                    "andalso" => ErlangSyntaxKind::Andalso,
                    "band" => ErlangSyntaxKind::Band,
                    "begin" => ErlangSyntaxKind::Begin,
                    "bnot" => ErlangSyntaxKind::Bnot,
                    "bor" => ErlangSyntaxKind::Bor,
                    "bsl" => ErlangSyntaxKind::Bsl,
                    "bsr" => ErlangSyntaxKind::Bsr,
                    "bxor" => ErlangSyntaxKind::Bxor,
                    "case" => ErlangSyntaxKind::Case,
                    "catch" => ErlangSyntaxKind::Catch,
                    "cond" => ErlangSyntaxKind::Cond,
                    "div" => ErlangSyntaxKind::Div,
                    "end" => ErlangSyntaxKind::End,
                    "fun" => ErlangSyntaxKind::Fun,
                    "if" => ErlangSyntaxKind::If,
                    "let" => ErlangSyntaxKind::Let,
                    "not" => ErlangSyntaxKind::Not,
                    "of" => ErlangSyntaxKind::Of,
                    "or" => ErlangSyntaxKind::Or,
                    "orelse" => ErlangSyntaxKind::Orelse,
                    "query" => ErlangSyntaxKind::Query,
                    "receive" => ErlangSyntaxKind::Receive,
                    "rem" => ErlangSyntaxKind::Rem,
                    "try" => ErlangSyntaxKind::Try,
                    "when" => ErlangSyntaxKind::When,
                    "xor" => ErlangSyntaxKind::Xor,
                    _ => ErlangSyntaxKind::Atom,
                };

                state.add_token(token_kind, start_pos, state.get_position());
                true
            }
            else if ch.is_ascii_uppercase() || ch == '_' {
                // 变量
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '@' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                state.add_token(ErlangSyntaxKind::Variable, start_pos, state.get_position());
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

    /// 处理数字
    fn lex_number(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(ch.len_utf8());

                // 处理整数部分
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 检查是否是进制
                if let Some('#') = state.peek() {
                    state.advance(1);
                    // 处理进制数的数字部分
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_alphanumeric() {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                }
                else {
                    // 处理小数部分
                    if let Some('.') = state.peek() {
                        let dot_pos = state.get_position();
                        state.advance(1);

                        if let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() {
                                while let Some(ch) = state.peek() {
                                    if ch.is_ascii_digit() {
                                        state.advance(ch.len_utf8());
                                    }
                                    else {
                                        break;
                                    }
                                }

                                // 处理科学计数
                                if let Some(ch) = state.peek() {
                                    if ch == 'e' || ch == 'E' {
                                        state.advance(1);
                                        if let Some(ch) = state.peek() {
                                            if ch == '+' || ch == '-' {
                                                state.advance(1);
                                            }
                                        }
                                        while let Some(ch) = state.peek() {
                                            if ch.is_ascii_digit() {
                                                state.advance(ch.len_utf8());
                                            }
                                            else {
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                            else {
                                // 回退点号
                                state.set_position(dot_pos);
                            }
                        }
                        else {
                            // 回退点号
                            state.set_position(dot_pos);
                        }
                    }
                }

                state.add_token(ErlangSyntaxKind::Number, start_pos, state.get_position());
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

    /// 处理字符
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    state.add_token(ErlangSyntaxKind::String, start_pos, state.get_position());
                    return true;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if state.peek().is_some() {
                        state.advance(1);
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            // 未闭合的字符
            state.add_token(ErlangSyntaxKind::Error, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符
    fn lex_character(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('$') = state.peek() {
            state.advance(1);

            if let Some(ch) = state.peek() {
                if ch == '\\' {
                    state.advance(1);
                    if state.peek().is_some() {
                        state.advance(1);
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }

                state.add_token(ErlangSyntaxKind::Character, start_pos, state.get_position());
                true
            }
            else {
                state.add_token(ErlangSyntaxKind::Error, start_pos, state.get_position());
                true
            }
        }
        else {
            false
        }
    }

    /// 处理操作
    fn lex_operator(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            match ch {
                '+' => {
                    let next_pos = state.get_position() + 1;
                    if next_pos < source.len() {
                        let next_ch = source.raw.chars().nth(next_pos);
                        if next_ch == Some('+') {
                            state.advance(1);
                            state.advance(1);
                            state.add_token(ErlangSyntaxKind::PlusPlus, start_pos, state.get_position());
                            true
                        }
                        else {
                            state.advance(1);
                            state.add_token(ErlangSyntaxKind::Plus, start_pos, state.get_position());
                            true
                        }
                    }
                    else {
                        state.advance(1);
                        state.add_token(ErlangSyntaxKind::Plus, start_pos, state.get_position());
                        true
                    }
                }
                '-' => {
                    let next_pos = state.get_position() + 1;
                    if next_pos < source.len() {
                        let next_ch = source.raw.chars().nth(next_pos);
                        if next_ch == Some('-') {
                            state.advance(1);
                            state.advance(1);
                            state.add_token(ErlangSyntaxKind::MinusMinus, start_pos, state.get_position());
                            true
                        }
                        else if next_ch == Some('>') {
                            state.advance(1);
                            state.advance(1);
                            state.add_token(ErlangSyntaxKind::Arrow, start_pos, state.get_position());
                            true
                        }
                        else {
                            state.advance(1);
                            state.add_token(ErlangSyntaxKind::Minus, start_pos, state.get_position());
                            true
                        }
                    }
                    else {
                        state.advance(1);
                        state.add_token(ErlangSyntaxKind::Minus, start_pos, state.get_position());
                        true
                    }
                }
                '*' => {
                    state.advance(1);
                    state.add_token(ErlangSyntaxKind::Star, start_pos, state.get_position());
                    true
                }
                '/' => {
                    let next_pos = state.get_position() + 1;
                    if next_pos < state.source().len() {
                        let next_ch = state.source().chars().nth(next_pos);
                        if next_ch == Some('=') {
                            state.advance(1);
                            state.advance(1);
                            state.add_token(ErlangSyntaxKind::SlashEqual, start_pos, state.get_position());
                            true
                        }
                        else {
                            state.advance(1);
                            state.add_token(ErlangSyntaxKind::Slash, start_pos, state.get_position());
                            true
                        }
                    }
                    else {
                        state.advance(1);
                        state.add_token(ErlangSyntaxKind::Slash, start_pos, state.get_position());
                        true
                    }
                }
                '=' => {
                    let next_pos = state.get_position() + 1;
                    if next_pos < state.source().len() {
                        let next_ch = state.source().chars().nth(next_pos);
                        match next_ch {
                            Some('=') => {
                                state.advance(1);
                                state.advance(1);
                                state.add_token(ErlangSyntaxKind::EqualEqual, start_pos, state.get_position());
                                true
                            }
                            Some(':') => {
                                let third_pos = state.get_position() + 2;
                                if third_pos < state.source().len() {
                                    let third_ch = state.source().chars().nth(third_pos);
                                    if third_ch == Some('=') {
                                        state.advance(1);
                                        state.advance(1);
                                        state.advance(1);
                                        state.add_token(ErlangSyntaxKind::ExactEqual, start_pos, state.get_position());
                                        true
                                    }
                                    else {
                                        state.advance(1);
                                        state.add_token(ErlangSyntaxKind::Equal, start_pos, state.get_position());
                                        true
                                    }
                                }
                                else {
                                    state.advance(1);
                                    state.add_token(ErlangSyntaxKind::Equal, start_pos, state.get_position());
                                    true
                                }
                            }
                            Some('<') => {
                                state.advance(1);
                                state.advance(1);
                                state.add_token(ErlangSyntaxKind::EqualLess, start_pos, state.get_position());
                                true
                            }
                            _ => {
                                state.advance(1);
                                state.add_token(ErlangSyntaxKind::Equal, start_pos, state.get_position());
                                true
                            }
                        }
                    }
                    else {
                        state.advance(1);
                        state.add_token(ErlangSyntaxKind::Equal, start_pos, state.get_position());
                        true
                    }
                }
                '<' => {
                    let next_pos = state.get_position() + 1;
                    if next_pos < state.source().len() {
                        let next_ch = state.source().chars().nth(next_pos);
                        if next_ch == Some('-') {
                            state.advance(1);
                            state.advance(1);
                            state.add_token(ErlangSyntaxKind::LeftArrow, start_pos, state.get_position());
                            true
                        }
                        else if next_ch == Some('=') {
                            state.advance(1);
                            state.advance(1);
                            state.add_token(ErlangSyntaxKind::LessEqual, start_pos, state.get_position());
                            true
                        }
                        else {
                            state.advance(1);
                            state.add_token(ErlangSyntaxKind::Less, start_pos, state.get_position());
                            true
                        }
                    }
                    else {
                        state.advance(1);
                        state.add_token(ErlangSyntaxKind::Less, start_pos, state.get_position());
                        true
                    }
                }
                '>' => {
                    let next_pos = state.get_position() + 1;
                    if next_pos < state.source().len() {
                        let next_ch = state.source().chars().nth(next_pos);
                        if next_ch == Some('=') {
                            state.advance(1);
                            state.advance(1);
                            state.add_token(ErlangSyntaxKind::GreaterEqual, start_pos, state.get_position());
                            true
                        }
                        else {
                            state.advance(1);
                            state.add_token(ErlangSyntaxKind::Greater, start_pos, state.get_position());
                            true
                        }
                    }
                    else {
                        state.advance(1);
                        state.add_token(ErlangSyntaxKind::Greater, start_pos, state.get_position());
                        true
                    }
                }
                '!' => {
                    state.advance(1);
                    state.add_token(ErlangSyntaxKind::Bang, start_pos, state.get_position());
                    true
                }
                '?' => {
                    state.advance(1);
                    state.add_token(ErlangSyntaxKind::Question, start_pos, state.get_position());
                    true
                }
                '|' => {
                    let next_pos = state.get_position() + 1;
                    if next_pos < state.source().len() {
                        let next_ch = state.source().chars().nth(next_pos);
                        if next_ch == Some('|') {
                            state.advance(1);
                            state.advance(1);
                            state.add_token(ErlangSyntaxKind::PipePipe, start_pos, state.get_position());
                            true
                        }
                        else {
                            state.advance(1);
                            state.add_token(ErlangSyntaxKind::Pipe, start_pos, state.get_position());
                            true
                        }
                    }
                    else {
                        state.advance(1);
                        state.add_token(ErlangSyntaxKind::Pipe, start_pos, state.get_position());
                        true
                    }
                }
                '#' => {
                    state.advance(1);
                    state.add_token(ErlangSyntaxKind::Hash, start_pos, state.get_position());
                    true
                }
                _ => false,
            }
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
                '(' => ErlangSyntaxKind::LeftParen,
                ')' => ErlangSyntaxKind::RightParen,
                '{' => ErlangSyntaxKind::LeftBrace,
                '}' => ErlangSyntaxKind::RightBrace,
                '[' => ErlangSyntaxKind::LeftBracket,
                ']' => ErlangSyntaxKind::RightBracket,
                ',' => ErlangSyntaxKind::Comma,
                ';' => ErlangSyntaxKind::Semicolon,
                '.' => ErlangSyntaxKind::Dot,
                ':' => ErlangSyntaxKind::Colon,
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

impl<'config> Lexer<ErlangLanguage> for ErlangLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<ErlangSyntaxKind> {
        let mut state = State::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_comment(&mut state) {
                continue;
            }

            if self.lex_identifier_atom_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_character(&mut state) {
                continue;
            }

            if self.lex_operator(&mut state, source) {
                continue;
            }

            if self.lex_delimiter(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，处理错误字符
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(ErlangSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(ErlangSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
