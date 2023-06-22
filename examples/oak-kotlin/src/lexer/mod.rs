use crate::{kind::KotlinSyntaxKind, language::KotlinLanguage};
use oak_core::{
    Lexer, LexerState, OakError, TextEdit,
    lexer::{LexOutput, LexerCache},
    source::Source,
};

type State<'a, S> = LexerState<'a, S, KotlinLanguage>;

pub struct KotlinLexer<'config> {
    _config: &'config KotlinLanguage,
}

impl<'config> KotlinLexer<'config> {
    pub fn new(config: &'config KotlinLanguage) -> Self {
        Self { _config: config }
    }

    /// 跳过空白字符
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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
    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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
                    else if let Some(ch) = state.peek() {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
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
    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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
                        else if let Some(ch) = state.peek() {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
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
                    break; // 字符串不能跨行
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }
            state.add_token(KotlinSyntaxKind::StringLiteral, start_pos, state.get_position());
            true
        }
        else if let Some('\'') = state.peek() {
            // 字符字面量
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

                // 处理小数点
                if let Some('.') = state.peek() {
                    // 预判下一个字符，如果是数字则是浮点数，如果是其他（如调用方法）则不是
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

                // 处理指数部分
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

                state.add_token(KotlinSyntaxKind::NumberLiteral, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理标识符和关键字
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
                    "abstract" | "actual" | "annotation" | "as" | "break" | "by" | "catch" | "class" | "companion" | "const" | "constructor" | "continue" | "crossinline" | "data" | "do" | "dynamic" | "else" | "enum" | "expect" | "external" | "false"
                    | "final" | "finally" | "for" | "fun" | "get" | "if" | "import" | "in" | "infix" | "init" | "inline" | "inner" | "interface" | "internal" | "is" | "lateinit" | "noinline" | "null" | "object" | "open" | "operator" | "out"
                    | "override" | "package" | "private" | "protected" | "public" | "reified" | "return" | "sealed" | "set" | "super" | "suspend" | "tailrec" | "this" | "throw" | "true" | "try" | "typealias" | "typeof" | "val" | "var" | "vararg"
                    | "when" | "where" | "while" => KotlinSyntaxKind::Keyword,
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
    fn lex_special_char<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '(' => KotlinSyntaxKind::LParen,
                ')' => KotlinSyntaxKind::RParen,
                '{' => KotlinSyntaxKind::LBrace,
                '}' => KotlinSyntaxKind::RBrace,
                '[' => KotlinSyntaxKind::LBracket,
                ']' => KotlinSyntaxKind::RBracket,
                ',' => KotlinSyntaxKind::Comma,
                ';' => KotlinSyntaxKind::Semi,
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
    fn lex<'a, S: Source + ?Sized>(&self, text: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<KotlinLanguage>) -> LexOutput<KotlinLanguage> {
        let mut state = State::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
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

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(KotlinSyntaxKind::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }
}
