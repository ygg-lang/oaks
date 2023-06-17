use crate::{kind::HaskellSyntaxKind, language::HaskellLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, HaskellLanguage>;

pub struct HaskellLexer<'config> {
    config: &'config HaskellLanguage,
}

impl<'config> HaskellLexer<'config> {
    pub fn new(config: &'config HaskellLanguage) -> Self {
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
            state.add_token(HaskellSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(HaskellSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(HaskellSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        // 单行注释 --
        if let Some('-') = state.peek() {
            if let Some('-') = source.get_char_at(start_pos + 1) {
                state.advance(2);

                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(HaskellSyntaxKind::Comment, start_pos, state.get_position());
                return true;
            }
        }

        // 多行注释 {- ... -}
        if let Some('{') = state.peek() {
            if let Some('-') = source.get_char_at(start_pos + 1) {
                state.advance(2);
                let mut depth = 1;

                while depth > 0 && state.not_at_end() {
                    if let Some('{') = state.peek() {
                        if let Some('-') = source.get_char_at(state.get_position() + 1) {
                            state.advance(2);
                            depth += 1;
                            continue;
                        }
                    }

                    if let Some('-') = state.peek() {
                        if let Some('}') = source.get_char_at(state.get_position() + 1) {
                            state.advance(2);
                            depth -= 1;
                            continue;
                        }
                    }

                    if let Some(ch) = state.peek() {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(HaskellSyntaxKind::Comment, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理标识符和关键
    fn lex_identifier_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            // 构造函数（大写开头）或标识符（小写开头）
            if ch.is_ascii_alphabetic() || ch == '_' {
                let is_constructor = ch.is_ascii_uppercase();
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '\'' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = source.get_text_in(core::range::Range { start: start_pos, end: state.get_position() });

                let token_kind = if is_constructor {
                    HaskellSyntaxKind::Constructor
                }
                else {
                    match text {
                        Some("case") => HaskellSyntaxKind::Case,
                        Some("class") => HaskellSyntaxKind::Class,
                        Some("data") => HaskellSyntaxKind::Data,
                        Some("default") => HaskellSyntaxKind::Default,
                        Some("deriving") => HaskellSyntaxKind::Deriving,
                        Some("do") => HaskellSyntaxKind::Do,
                        Some("else") => HaskellSyntaxKind::Else,
                        Some("foreign") => HaskellSyntaxKind::Foreign,
                        Some("if") => HaskellSyntaxKind::If,
                        Some("import") => HaskellSyntaxKind::Import,
                        Some("in") => HaskellSyntaxKind::In,
                        Some("infix") => HaskellSyntaxKind::Infix,
                        Some("infixl") => HaskellSyntaxKind::Infixl,
                        Some("infixr") => HaskellSyntaxKind::Infixr,
                        Some("instance") => HaskellSyntaxKind::Instance,
                        Some("let") => HaskellSyntaxKind::Let,
                        Some("module") => HaskellSyntaxKind::Module,
                        Some("newtype") => HaskellSyntaxKind::Newtype,
                        Some("of") => HaskellSyntaxKind::Of,
                        Some("then") => HaskellSyntaxKind::Then,
                        Some("type") => HaskellSyntaxKind::Type,
                        Some("where") => HaskellSyntaxKind::Where,
                        Some("as") => HaskellSyntaxKind::As,
                        Some("qualified") => HaskellSyntaxKind::Qualified,
                        Some("hiding") => HaskellSyntaxKind::Hiding,
                        Some("_") => HaskellSyntaxKind::Underscore,
                        _ => HaskellSyntaxKind::Identifier,
                    }
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

    /// 处理数字
    fn lex_number(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);

                // 处理整数部分
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // 检查是否是浮点
                let mut is_float = false;
                if let Some('.') = state.peek() {
                    if let Some(next_ch) = source.get_char_at(state.get_position() + 1) {
                        if next_ch.is_ascii_digit() {
                            is_float = true;
                            state.advance(1); // 跳过 '.'

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

                // 处理指数部分
                if let Some(ch) = state.peek() {
                    if ch == 'e' || ch == 'E' {
                        is_float = true;
                        state.advance(1);

                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
                                state.advance(1);
                            }
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
                }

                let token_kind = if is_float { HaskellSyntaxKind::Float } else { HaskellSyntaxKind::Integer };

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

    /// 处理字符
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    state.add_token(HaskellSyntaxKind::String, start_pos, state.get_position());
                    return true;
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

            // 未闭合的字符
            state.add_token(HaskellSyntaxKind::Error, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符
    fn lex_char(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
            state.advance(1);

            if let Some(ch) = state.peek() {
                if ch == '\\' {
                    state.advance(1);
                    if let Some(_) = state.peek() {
                        state.advance(1);
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }

                if let Some('\'') = state.peek() {
                    state.advance(1);
                    state.add_token(HaskellSyntaxKind::Char, start_pos, state.get_position());
                    return true;
                }
            }

            // 未闭合的字符
            state.add_token(HaskellSyntaxKind::Error, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理操作
    fn lex_operators(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    HaskellSyntaxKind::Plus
                }
                '-' => {
                    state.advance(1);
                    if let Some('>') = state.peek() {
                        state.advance(1);
                        HaskellSyntaxKind::Arrow
                    }
                    else {
                        HaskellSyntaxKind::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    HaskellSyntaxKind::Star
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        HaskellSyntaxKind::NotEqual
                    }
                    else {
                        HaskellSyntaxKind::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    HaskellSyntaxKind::Percent
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        HaskellSyntaxKind::Equal
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        HaskellSyntaxKind::DoubleArrow
                    }
                    else {
                        return false; // 单独= Haskell 中不是操作符
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        HaskellSyntaxKind::LessEqual
                    }
                    else {
                        HaskellSyntaxKind::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        HaskellSyntaxKind::GreaterEqual
                    }
                    else {
                        HaskellSyntaxKind::Greater
                    }
                }
                ':' => {
                    state.advance(1);
                    if let Some(':') = state.peek() {
                        state.advance(1);
                        HaskellSyntaxKind::DoubleColon
                    }
                    else {
                        HaskellSyntaxKind::Colon
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        HaskellSyntaxKind::Or
                    }
                    else {
                        HaskellSyntaxKind::Pipe
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        HaskellSyntaxKind::And
                    }
                    else {
                        HaskellSyntaxKind::Ampersand
                    }
                }
                '!' => {
                    state.advance(1);
                    HaskellSyntaxKind::Exclamation
                }
                '?' => {
                    state.advance(1);
                    HaskellSyntaxKind::Question
                }
                ';' => {
                    state.advance(1);
                    HaskellSyntaxKind::Semicolon
                }
                ',' => {
                    state.advance(1);
                    HaskellSyntaxKind::Comma
                }
                '.' => {
                    state.advance(1);
                    if let Some('.') = state.peek() {
                        state.advance(1);
                        HaskellSyntaxKind::DotDot
                    }
                    else {
                        HaskellSyntaxKind::Dot
                    }
                }
                '$' => {
                    state.advance(1);
                    HaskellSyntaxKind::Dollar
                }
                '@' => {
                    state.advance(1);
                    HaskellSyntaxKind::At
                }
                '~' => {
                    state.advance(1);
                    HaskellSyntaxKind::Tilde
                }
                '\\' => {
                    state.advance(1);
                    HaskellSyntaxKind::Backslash
                }
                '`' => {
                    state.advance(1);
                    HaskellSyntaxKind::Backquote
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
    fn lex_delimiters(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => HaskellSyntaxKind::LeftParen,
                ')' => HaskellSyntaxKind::RightParen,
                '[' => HaskellSyntaxKind::LeftBracket,
                ']' => HaskellSyntaxKind::RightBracket,
                '{' => HaskellSyntaxKind::LeftBrace,
                '}' => HaskellSyntaxKind::RightBrace,
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

impl<'config> Lexer<HaskellLanguage> for HaskellLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<HaskellSyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_comment(&mut state, source) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_char(&mut state) {
                continue;
            }

            if self.lex_number(&mut state, source) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_operators(&mut state, source) {
                continue;
            }

            if self.lex_delimiters(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(HaskellSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(HaskellSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
