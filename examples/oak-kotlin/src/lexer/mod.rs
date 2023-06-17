use crate::{kind::KotlinSyntaxKind, language::KotlinLanguage};
use oak_core::{
    Lexer, SourceText,
    lexer::{LexOutput, LexerState},
};

type State<'input> = LexerState<'input, KotlinLanguage>;

pub struct KotlinLexer<'config> {
    config: &'config KotlinLanguage,
}

impl<'config> KotlinLexer<'config> {
    pub fn new(config: &'config KotlinLanguage) -> Self {
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
            state.add_token(KotlinSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(KotlinSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(KotlinSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            state.advance(1);
            if let Some('/') = state.peek() {
                // 单行注释
                state.advance(1);
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                state.add_token(KotlinSyntaxKind::Comment, start_pos, state.get_position());
                true
            }
            else if let Some('*') = state.peek() {
                // 多行注释
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
                    else {
                        let ch = state.peek().unwrap();
                        state.advance(ch.len_utf8());
                    }
                }
                state.add_token(KotlinSyntaxKind::Comment, start_pos, state.get_position());
                true
            }
            else {
                // 回退，这是除法操作符
                state.set_position(start_pos);
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

            // 检查是否是三引号字符串
            if let Some('"') = state.peek() {
                state.advance(1);
                if let Some('"') = state.peek() {
                    // 三引号字符串
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
                        else {
                            let ch = state.peek().unwrap();
                            state.advance(ch.len_utf8());
                        }
                    }
                    state.add_token(KotlinSyntaxKind::StringLiteral, start_pos, state.get_position());
                    return true;
                }
                else {
                    // 空字符串
                    state.add_token(KotlinSyntaxKind::StringLiteral, start_pos, state.get_position());
                    return true;
                }
            }

            // 普通字符串
            while let Some(ch) = state.peek() {
                if ch == '"' {
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
                    break; // 字符串不能跨
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }
            state.add_token(KotlinSyntaxKind::StringLiteral, start_pos, state.get_position());
            true
        }
        else if let Some('\'') = state.peek() {
            // 字符字面
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
            state.add_token(KotlinSyntaxKind::CharLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理数字字面量
    fn lex_number(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                // 扫描数字
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 检查小数点
                if let Some('.') = state.peek() {
                    let current_pos = state.get_position();
                    state.advance(1); // 临时前进查看下一个字符
                    if let Some(next_ch) = state.peek() {
                        if next_ch.is_ascii_digit() {
                            // 这是小数点，继续处理
                        }
                        else {
                            // 不是小数点，回退
                            state.set_position(current_pos);
                        }
                    }
                    else {
                        // 没有下一个字符，回退
                        state.set_position(current_pos);
                    }

                    if let Some(next_ch) = state.peek() {
                        if next_ch.is_ascii_digit() {
                            // 小数点已经被处理了
                            while let Some(ch) = state.peek() {
                                if ch.is_ascii_digit() || ch == '_' {
                                    state.advance(ch.len_utf8());
                                }
                                else {
                                    break;
                                }
                            }
                        }
                    }
                }

                // 检查指
                if let Some(ch) = state.peek() {
                    if ch == 'e' || ch == 'E' {
                        state.advance(1);
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
                                state.advance(1);
                            }
                        }
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() || ch == '_' {
                                state.advance(ch.len_utf8());
                            }
                            else {
                                break;
                            }
                        }
                    }
                }

                // 检查后缀
                if let Some(ch) = state.peek() {
                    if ch == 'L' || ch == 'l' || ch == 'F' || ch == 'f' {
                        state.advance(1);
                    }
                }

                state.add_token(KotlinSyntaxKind::NumberLiteral, start_pos, state.get_position());
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

    /// 处理标识符和关键字
    fn lex_identifier_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
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

                let text = source.get_text_in(core::range::Range { start: start_pos, end: state.get_position() }).unwrap_or("");
                let kind = match text {
                    "abstract" | "actual" | "annotation" | "as" | "break" | "by" | "catch" | "class" | "companion"
                    | "const" | "constructor" | "continue" | "crossinline" | "data" | "do" | "dynamic" | "else" | "enum"
                    | "expect" | "external" | "false" | "final" | "finally" | "for" | "fun" | "get" | "if" | "import"
                    | "in" | "infix" | "init" | "inline" | "inner" | "interface" | "internal" | "is" | "lateinit"
                    | "noinline" | "null" | "object" | "open" | "operator" | "out" | "override" | "package" | "private"
                    | "protected" | "public" | "reified" | "return" | "sealed" | "set" | "super" | "suspend" | "tailrec"
                    | "this" | "throw" | "true" | "try" | "typealias" | "typeof" | "val" | "var" | "vararg" | "when"
                    | "where" | "while" => KotlinSyntaxKind::Keyword,
                    _ => KotlinSyntaxKind::Identifier,
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

    /// 处理特殊字符和操作符
    fn lex_special_char(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '(' => KotlinSyntaxKind::LeftParen,
                ')' => KotlinSyntaxKind::RightParen,
                '{' => KotlinSyntaxKind::LeftBrace,
                '}' => KotlinSyntaxKind::RightBrace,
                '[' => KotlinSyntaxKind::LeftBracket,
                ']' => KotlinSyntaxKind::RightBracket,
                ',' => KotlinSyntaxKind::Comma,
                ';' => KotlinSyntaxKind::Semicolon,
                ':' => KotlinSyntaxKind::Colon,
                '.' => KotlinSyntaxKind::Dot,
                '?' => KotlinSyntaxKind::Question,
                '!' => KotlinSyntaxKind::Exclamation,
                '+' => KotlinSyntaxKind::Plus,
                '-' => KotlinSyntaxKind::Minus,
                '*' => KotlinSyntaxKind::Star,
                '/' => KotlinSyntaxKind::Slash,
                '%' => KotlinSyntaxKind::Percent,
                '=' => KotlinSyntaxKind::Equals,
                '<' => KotlinSyntaxKind::Less,
                '>' => KotlinSyntaxKind::Greater,
                '&' => KotlinSyntaxKind::Ampersand,
                '|' => KotlinSyntaxKind::Pipe,
                '^' => KotlinSyntaxKind::Caret,
                '~' => KotlinSyntaxKind::Tilde,
                '@' => KotlinSyntaxKind::At,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<KotlinLanguage> for KotlinLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<KotlinSyntaxKind> {
        let mut state = State::new(source);

        while state.not_at_end() {
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_comment(&mut state) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_special_char(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(KotlinSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(KotlinSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
