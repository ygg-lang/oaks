use crate::{kind::RustSyntaxKind, language::RustLanguage};
use oak_core::{Lexer, LexerState, SourceText, Token, lexer::LexOutput};

type State<'input> = LexerState<'input, RustLanguage>;
pub type RustToken = Token<RustSyntaxKind>;

pub struct RustLexer<'config> {
    config: &'config RustLanguage,
}

impl<'config> RustLexer<'config> {
    pub fn new(config: &'config RustLanguage) -> Self {
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
            state.add_token(RustSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(RustSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(RustSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            if let Some('/') = source.get_char_at(start_pos + 1) {
                // 单行注释
                state.advance(2);
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                state.add_token(RustSyntaxKind::Comment, start_pos, state.get_position());
                true
            }
            else if let Some('*') = source.get_char_at(start_pos + 1) {
                // 多行注释
                state.advance(2);
                let mut depth = 1;

                while let Some(ch) = state.peek() {
                    if ch == '/' && state.peek_next_n(1) == Some('*') {
                        state.advance(2);
                        depth += 1;
                    }
                    else if ch == '*' && state.peek_next_n(1) == Some('/') {
                        state.advance(2);
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }
                state.add_token(RustSyntaxKind::Comment, start_pos, state.get_position());
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
    fn lex_string_literal(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1);
            let mut escaped = false;

            while let Some(ch) = state.peek() {
                if escaped {
                    escaped = false;
                    state.advance(ch.len_utf8());
                }
                else if ch == '\\' {
                    escaped = true;
                    state.advance(1);
                }
                else if ch == '"' {
                    state.advance(1);
                    state.add_token(RustSyntaxKind::StringLiteral, start_pos, state.get_position());
                    return true;
                }
                else if ch == '\n' || ch == '\r' {
                    break; // 字符串不能跨
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            // 未闭合的字符

            state.add_token(RustSyntaxKind::Error, start_pos, state.get_position());
            return true;
        }
        else {
            false
        }
    }

    /// 处理字符字面

    fn lex_char_literal(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
            state.advance(1);
            let mut escaped = false;
            let mut char_count = 0;

            while let Some(ch) = state.peek() {
                if escaped {
                    escaped = false;
                    state.advance(ch.len_utf8());
                    char_count += 1;
                }
                else if ch == '\\' {
                    escaped = true;
                    state.advance(1);
                }
                else if ch == '\'' {
                    state.advance(1);
                    if char_count == 1 || (char_count == 0 && escaped) {
                        state.add_token(RustSyntaxKind::CharLiteral, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(RustSyntaxKind::Error, start_pos, state.get_position());
                    }
                    return true;
                }
                else if ch == '\n' || ch == '\r' {
                    break;
                }
                else {
                    state.advance(ch.len_utf8());
                    char_count += 1;
                }
            }

            // 未闭合的字符字面            state.add_token(RustSyntaxKind::Error, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理数字字面

    fn lex_number_literal(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                // 整数部分
                while let Some(digit) = state.peek() {
                    if digit.is_ascii_digit() || digit == '_' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // 检查是否是浮点

                if let Some('.') = state.peek() {
                    if let Some(next_ch) = state.peek_next_n(1) {
                        if next_ch.is_ascii_digit() {
                            state.advance(1); // 跳过 '.'
                            while let Some(digit) = state.peek() {
                                if digit.is_ascii_digit() || digit == '_' {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }
                            state.add_token(RustSyntaxKind::FloatLiteral, start_pos, state.get_position());
                            return true;
                        }
                    }
                }

                // 检查科学计数法
                if let Some(e) = state.peek() {
                    if e == 'e' || e == 'E' {
                        let mut temp_pos = state.get_position() + 1;
                        if let Some(sign) = state.peek_next_n(1) {
                            if sign == '+' || sign == '-' {
                                temp_pos += 1;
                            }
                        }
                        if let Some(digit) = state.peek_next_n(temp_pos - state.get_position()) {
                            if digit.is_ascii_digit() {
                                state.set_position(temp_pos);
                                while let Some(digit) = state.peek() {
                                    if digit.is_ascii_digit() || digit == '_' {
                                        state.advance(1);
                                    }
                                    else {
                                        break;
                                    }
                                }
                                state.add_token(RustSyntaxKind::FloatLiteral, start_pos, state.get_position());
                                return true;
                            }
                        }
                    }
                }

                state.add_token(RustSyntaxKind::IntegerLiteral, start_pos, state.get_position());
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

    /// 处理标识符和关键

    fn lex_identifier_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = source.get_text_in(core::range::Range { start: start_pos, end: state.get_position() }).unwrap_or("");
                let token_kind = match text {
                    "as" => RustSyntaxKind::As,
                    "async" => RustSyntaxKind::Async,
                    "await" => RustSyntaxKind::Await,
                    "break" => RustSyntaxKind::Break,
                    "const" => RustSyntaxKind::Const,
                    "continue" => RustSyntaxKind::Continue,
                    "crate" => RustSyntaxKind::Crate,
                    "dyn" => RustSyntaxKind::Dyn,
                    "else" => RustSyntaxKind::Else,
                    "enum" => RustSyntaxKind::Enum,
                    "extern" => RustSyntaxKind::Extern,
                    "false" => RustSyntaxKind::False,
                    "fn" => RustSyntaxKind::Fn,
                    "for" => RustSyntaxKind::For,
                    "if" => RustSyntaxKind::If,
                    "impl" => RustSyntaxKind::Impl,
                    "in" => RustSyntaxKind::In,
                    "let" => RustSyntaxKind::Let,
                    "loop" => RustSyntaxKind::Loop,
                    "match" => RustSyntaxKind::Match,
                    "mod" => RustSyntaxKind::Mod,
                    "move" => RustSyntaxKind::Move,
                    "mut" => RustSyntaxKind::Mut,
                    "pub" => RustSyntaxKind::Pub,
                    "ref" => RustSyntaxKind::Ref,
                    "return" => RustSyntaxKind::Return,
                    "self" => RustSyntaxKind::SelfValue,
                    "Self" => RustSyntaxKind::SelfType,
                    "static" => RustSyntaxKind::Static,
                    "struct" => RustSyntaxKind::Struct,
                    "super" => RustSyntaxKind::Super,
                    "trait" => RustSyntaxKind::Trait,
                    "true" => RustSyntaxKind::True,
                    "type" => RustSyntaxKind::Type,
                    "unsafe" => RustSyntaxKind::Unsafe,
                    "use" => RustSyntaxKind::Use,
                    "where" => RustSyntaxKind::Where,
                    "while" => RustSyntaxKind::While,
                    _ => RustSyntaxKind::Identifier,
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

    /// 处理生命周期
    fn lex_lifetime(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
            // 检查下一个字符是否是字母或下划线
            if let Some(next_ch) = state.peek_next_n(1) {
                if next_ch.is_alphabetic() || next_ch == '_' {
                    state.advance(1); // 跳过 '
                    state.advance(next_ch.len_utf8());

                    while let Some(ch) = state.peek() {
                        if ch.is_alphanumeric() || ch == '_' {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }

                    state.add_token(RustSyntaxKind::Lifetime, start_pos, state.get_position());
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
        else {
            false
        }
    }

    /// 处理操作
    fn lex_operators(&self, state: &mut State, _source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            match ch {
                '+' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        state.add_token(RustSyntaxKind::PlusEq, start_pos, state.get_position());
                    }
                    else {
                        state.advance(1);
                        state.add_token(RustSyntaxKind::Plus, start_pos, state.get_position());
                    }
                    true
                }
                '-' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        state.add_token(RustSyntaxKind::MinusEq, start_pos, state.get_position());
                    }
                    else if let Some('>') = state.peek_next_n(1) {
                        state.advance(2);
                        state.add_token(RustSyntaxKind::RArrow, start_pos, state.get_position());
                    }
                    else {
                        state.advance(1);
                        state.add_token(RustSyntaxKind::Minus, start_pos, state.get_position());
                    }
                    true
                }
                '*' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        state.add_token(RustSyntaxKind::StarEq, start_pos, state.get_position());
                    }
                    else {
                        state.advance(1);
                        state.add_token(RustSyntaxKind::Star, start_pos, state.get_position());
                    }
                    true
                }
                '/' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        state.add_token(RustSyntaxKind::SlashEq, start_pos, state.get_position());
                    }
                    else {
                        state.advance(1);
                        state.add_token(RustSyntaxKind::Slash, start_pos, state.get_position());
                    }
                    true
                }
                '%' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        state.add_token(RustSyntaxKind::PercentEq, start_pos, state.get_position());
                    }
                    else {
                        state.advance(1);
                        state.add_token(RustSyntaxKind::Percent, start_pos, state.get_position());
                    }
                    true
                }
                '^' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        state.add_token(RustSyntaxKind::CaretEq, start_pos, state.get_position());
                    }
                    else {
                        state.advance(1);
                        state.add_token(RustSyntaxKind::Caret, start_pos, state.get_position());
                    }
                    true
                }
                '&' => {
                    if let Some('&') = state.peek_next_n(1) {
                        state.advance(2);
                        state.add_token(RustSyntaxKind::AndAnd, start_pos, state.get_position());
                    }
                    else if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        state.add_token(RustSyntaxKind::AndEq, start_pos, state.get_position());
                    }
                    else {
                        state.advance(1);
                        state.add_token(RustSyntaxKind::And, start_pos, state.get_position());
                    }
                    true
                }
                '|' => {
                    if let Some('|') = state.peek_next_n(1) {
                        state.advance(2);
                        state.add_token(RustSyntaxKind::OrOr, start_pos, state.get_position());
                    }
                    else if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        state.add_token(RustSyntaxKind::OrEq, start_pos, state.get_position());
                    }
                    else {
                        state.advance(1);
                        state.add_token(RustSyntaxKind::Or, start_pos, state.get_position());
                    }
                    true
                }
                '<' => {
                    if let Some('<') = state.peek_next_n(1) {
                        if let Some('=') = state.peek_next_n(2) {
                            state.advance(3);
                            state.add_token(RustSyntaxKind::ShlEq, start_pos, state.get_position());
                        }
                        else {
                            state.advance(2);
                            state.add_token(RustSyntaxKind::Shl, start_pos, state.get_position());
                        }
                    }
                    else if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        state.add_token(RustSyntaxKind::Le, start_pos, state.get_position());
                    }
                    else {
                        state.advance(1);
                        state.add_token(RustSyntaxKind::Lt, start_pos, state.get_position());
                    }
                    true
                }
                '>' => {
                    if let Some('>') = state.peek_next_n(1) {
                        if let Some('=') = state.peek_next_n(2) {
                            state.advance(3);
                            state.add_token(RustSyntaxKind::ShrEq, start_pos, state.get_position());
                        }
                        else {
                            state.advance(2);
                            state.add_token(RustSyntaxKind::Shr, start_pos, state.get_position());
                        }
                    }
                    else if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        state.add_token(RustSyntaxKind::Ge, start_pos, state.get_position());
                    }
                    else {
                        state.advance(1);
                        state.add_token(RustSyntaxKind::Gt, start_pos, state.get_position());
                    }
                    true
                }
                '=' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        state.add_token(RustSyntaxKind::EqEq, start_pos, state.get_position());
                    }
                    else if let Some('>') = state.peek_next_n(1) {
                        state.advance(2);
                        state.add_token(RustSyntaxKind::FatArrow, start_pos, state.get_position());
                    }
                    else {
                        state.advance(1);
                        state.add_token(RustSyntaxKind::Eq, start_pos, state.get_position());
                    }
                    true
                }
                '!' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        state.add_token(RustSyntaxKind::Ne, start_pos, state.get_position());
                    }
                    else {
                        state.advance(1);
                        state.add_token(RustSyntaxKind::Not, start_pos, state.get_position());
                    }
                    true
                }
                '.' => {
                    if let Some('.') = state.peek_next_n(1) {
                        if let Some('.') = state.peek_next_n(2) {
                            state.advance(3);
                            state.add_token(RustSyntaxKind::DotDotDot, start_pos, state.get_position());
                        }
                        else if let Some('=') = state.peek_next_n(2) {
                            state.advance(3);
                            state.add_token(RustSyntaxKind::DotDotEq, start_pos, state.get_position());
                        }
                        else {
                            state.advance(2);
                            state.add_token(RustSyntaxKind::DotDot, start_pos, state.get_position());
                        }
                    }
                    else {
                        state.advance(1);
                        state.add_token(RustSyntaxKind::Dot, start_pos, state.get_position());
                    }
                    true
                }
                ':' => {
                    if let Some(':') = state.peek_next_n(1) {
                        state.advance(2);
                        state.add_token(RustSyntaxKind::PathSep, start_pos, state.get_position());
                    }
                    else {
                        state.advance(1);
                        state.add_token(RustSyntaxKind::Colon, start_pos, state.get_position());
                    }
                    true
                }
                _ => false,
            }
        }
        else {
            false
        }
    }

    /// 处理单字符标

    fn lex_single_char_tokens(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => RustSyntaxKind::LeftParen,
                ')' => RustSyntaxKind::RightParen,
                '{' => RustSyntaxKind::LeftBrace,
                '}' => RustSyntaxKind::RightBrace,
                '[' => RustSyntaxKind::LeftBracket,
                ']' => RustSyntaxKind::RightBracket,
                ',' => RustSyntaxKind::Comma,
                ';' => RustSyntaxKind::Semicolon,
                '@' => RustSyntaxKind::At,
                '_' => RustSyntaxKind::Underscore,
                '#' => RustSyntaxKind::Pound,
                '$' => RustSyntaxKind::Dollar,
                '?' => RustSyntaxKind::Question,
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

impl<'config> Lexer<RustLanguage> for RustLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<RustSyntaxKind> {
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

            if self.lex_string_literal(&mut state) {
                continue;
            }

            if self.lex_char_literal(&mut state) {
                continue;
            }

            if self.lex_number_literal(&mut state) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_lifetime(&mut state) {
                continue;
            }

            if self.lex_operators(&mut state, source) {
                continue;
            }

            if self.lex_single_char_tokens(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(RustSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(RustSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
