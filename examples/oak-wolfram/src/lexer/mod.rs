use crate::{kind::WolframSyntaxKind, language::WolframLanguage};
use core::range::Range;
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, WolframLanguage>;

pub struct WolframLexer<'config> {
    config: &'config WolframLanguage,
}

impl<'config> WolframLexer<'config> {
    pub fn new(config: &'config WolframLanguage) -> Self {
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
            state.add_token(WolframSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(WolframSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(WolframSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        // Wolfram 注释 (* ... *)
        if let Some('(') = state.peek() {
            let next_pos = state.get_position() + 1;
            if let Some('*') = source.get_char_at(next_pos) {
                state.advance(2);
                let mut depth = 1;

                while depth > 0 && state.not_at_end() {
                    if let Some('(') = state.peek() {
                        let next_pos = state.get_position() + 1;
                        if let Some('*') = source.get_char_at(next_pos) {
                            state.advance(2);
                            depth += 1;
                        }
                        else {
                            state.advance(1);
                        }
                    }
                    else if let Some('*') = state.peek() {
                        let next_pos = state.get_position() + 1;
                        if let Some(')') = source.get_char_at(next_pos) {
                            state.advance(2);
                            depth -= 1;
                        }
                        else {
                            state.advance(1);
                        }
                    }
                    else if let Some(ch) = state.peek() {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                state.add_token(WolframSyntaxKind::Comment, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理字符串字面量
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        // Wolfram 字符串字面量 "..."
        if let Some('"') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    state.add_token(WolframSyntaxKind::String, start_pos, state.get_position());
                    return true;
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

            // 未闭合的字符

            state.add_token(WolframSyntaxKind::Error, start_pos, state.get_position());
            return true;
        }

        false
    }

    /// 处理数字字面量
    fn lex_number(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let next_pos = state.get_position() + 1;
            if ch.is_ascii_digit() || (ch == '.' && source.get_char_at(next_pos).map_or(false, |c| c.is_ascii_digit())) {
                let mut is_real = false;

                // 处理整数部分
                while let Some(digit) = state.peek() {
                    if digit.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // 处理小数部分
                if let Some('.') = state.peek() {
                    let next_pos = state.get_position() + 1;
                    if let Some(next_ch) = source.get_char_at(next_pos) {
                        if next_ch.is_ascii_digit() {
                            state.advance(1); // 跳过 '.'
                            is_real = true;
                            while let Some(digit) = state.peek() {
                                if digit.is_ascii_digit() {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }
                        }
                    }
                }

                // 处理科学计数

                if let Some(e) = state.peek() {
                    if e == 'e' || e == 'E' {
                        let saved_pos = state.get_position();
                        state.advance(1);
                        is_real = true;

                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
                                state.advance(1);
                            }
                        }

                        let mut has_digits = false;
                        while let Some(digit) = state.peek() {
                            if digit.is_ascii_digit() {
                                state.advance(1);
                                has_digits = true;
                            }
                            else {
                                break;
                            }
                        }

                        // 如果没有数字，回退
                        if !has_digits {
                            state.set_position(saved_pos);
                            is_real = false;
                        }
                    }
                }

                // 处理 Wolfram 特殊数字后缀（如 `10 等）
                if let Some('`') = state.peek() {
                    state.advance(1);
                    is_real = true;
                    while let Some(digit) = state.peek() {
                        if digit.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                if is_real {
                    state.add_token(WolframSyntaxKind::Real, start_pos, state.get_position());
                }
                else {
                    state.add_token(WolframSyntaxKind::Integer, start_pos, state.get_position());
                }
                return true;
            }
        }

        false
    }

    /// 处理标识符和关键字
    fn lex_ident_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '$' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '$' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 检查是否是关键字
                let text = source.get_text_in(Range { start: start_pos, end: state.get_position() }).unwrap_or("");
                let token_kind = match text {
                    "If" => WolframSyntaxKind::If,
                    "Then" => WolframSyntaxKind::Then,
                    "Else" => WolframSyntaxKind::Else,
                    "While" => WolframSyntaxKind::While,
                    "For" => WolframSyntaxKind::For,
                    "Do" => WolframSyntaxKind::Do,
                    "Function" => WolframSyntaxKind::Function,
                    "Module" => WolframSyntaxKind::Module,
                    "Block" => WolframSyntaxKind::Block,
                    "With" => WolframSyntaxKind::With,
                    "Table" => WolframSyntaxKind::Table,
                    "Map" => WolframSyntaxKind::Map,
                    "Apply" => WolframSyntaxKind::Apply,
                    "Select" => WolframSyntaxKind::Select,
                    "Cases" => WolframSyntaxKind::Cases,
                    "Rule" => WolframSyntaxKind::Rule,
                    "RuleDelayed" => WolframSyntaxKind::RuleDelayed,
                    "Set" => WolframSyntaxKind::Set,
                    "SetDelayed" => WolframSyntaxKind::SetDelayed,
                    "Unset" => WolframSyntaxKind::Unset,
                    "Clear" => WolframSyntaxKind::Clear,
                    "ClearAll" => WolframSyntaxKind::ClearAll,
                    "Return" => WolframSyntaxKind::Return,
                    "Break" => WolframSyntaxKind::Break,
                    "Continue" => WolframSyntaxKind::Continue,
                    "True" => WolframSyntaxKind::True,
                    "False" => WolframSyntaxKind::False,
                    "Null" => WolframSyntaxKind::Null,
                    "Export" => WolframSyntaxKind::Export,
                    "Import" => WolframSyntaxKind::Import,
                    _ => WolframSyntaxKind::Identifier,
                };

                state.add_token(token_kind, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理操作符和标点符号
    fn lex_operator_or_punctuation(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        WolframSyntaxKind::AddTo
                    }
                    else {
                        WolframSyntaxKind::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        WolframSyntaxKind::SubtractFrom
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        WolframSyntaxKind::Arrow
                    }
                    else {
                        WolframSyntaxKind::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        WolframSyntaxKind::TimesBy
                    }
                    else {
                        WolframSyntaxKind::Times
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        WolframSyntaxKind::DivideBy
                    }
                    else {
                        WolframSyntaxKind::Divide
                    }
                }
                '^' => {
                    state.advance(1);
                    WolframSyntaxKind::Power
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        WolframSyntaxKind::Equal
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        WolframSyntaxKind::DoubleArrow
                    }
                    else {
                        WolframSyntaxKind::Assign
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        WolframSyntaxKind::NotEqual
                    }
                    else {
                        WolframSyntaxKind::Not
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        WolframSyntaxKind::LessEqual
                    }
                    else {
                        WolframSyntaxKind::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        WolframSyntaxKind::GreaterEqual
                    }
                    else {
                        WolframSyntaxKind::Greater
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        WolframSyntaxKind::And
                    }
                    else {
                        return false; // 单个 & 不是有效token
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        WolframSyntaxKind::Or
                    }
                    else {
                        return false; // 单个 | 不是有效token
                    }
                }
                '_' => {
                    state.advance(1);
                    if let Some('_') = state.peek() {
                        state.advance(1);
                        if let Some('_') = state.peek() {
                            state.advance(1);
                            WolframSyntaxKind::TripleUnderscore
                        }
                        else {
                            WolframSyntaxKind::DoubleUnderscore
                        }
                    }
                    else {
                        WolframSyntaxKind::Underscore
                    }
                }
                '#' => {
                    state.advance(1);
                    if let Some('#') = state.peek() {
                        state.advance(1);
                        WolframSyntaxKind::SlotSequence
                    }
                    else {
                        WolframSyntaxKind::Slot
                    }
                }
                '(' => {
                    state.advance(1);
                    WolframSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    WolframSyntaxKind::RightParen
                }
                '[' => {
                    state.advance(1);
                    WolframSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    WolframSyntaxKind::RightBracket
                }
                '{' => {
                    state.advance(1);
                    WolframSyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    WolframSyntaxKind::RightBrace
                }
                ',' => {
                    state.advance(1);
                    WolframSyntaxKind::Comma
                }
                ';' => {
                    state.advance(1);
                    WolframSyntaxKind::Semicolon
                }
                '.' => {
                    state.advance(1);
                    WolframSyntaxKind::Dot
                }
                ':' => {
                    state.advance(1);
                    WolframSyntaxKind::Colon
                }
                '?' => {
                    state.advance(1);
                    WolframSyntaxKind::Question
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

    /// 处理普通文

    fn lex_text(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            // 遇到特殊字符时停

            match ch {
                ' ' | '\t' | '\n' | '\r' | '(' | ')' | '[' | ']' | '{' | '}' | ',' | ';' | '.' | ':' | '?' | '+' | '-'
                | '*' | '/' | '^' | '=' | '!' | '<' | '>' | '&' | '|' | '_' | '#' | '"' => break,
                _ => {
                    if ch.is_ascii_alphabetic() || ch.is_ascii_digit() || ch == '$' {
                        break; // 这些应该由其他规则处
                    }
                    state.advance(ch.len_utf8());
                }
            }
        }

        if state.get_position() > start_pos {
            state.add_token(WolframSyntaxKind::Text, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<WolframLanguage> for WolframLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<WolframSyntaxKind> {
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

            if self.lex_number(&mut state, source) {
                continue;
            }

            if self.lex_ident_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_operator_or_punctuation(&mut state) {
                continue;
            }

            if self.lex_text(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(WolframSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(WolframSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
