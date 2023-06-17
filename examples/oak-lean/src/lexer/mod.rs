//! Lean 词法分析
//!
//! 实现Lean 语言的词法分析，将源代码转换token 序列

use crate::{language::LeanLanguage, syntax::LeanSyntaxKind};
use oak_core::{
    Lexer, SourceText,
    lexer::{LexOutput, LexerState},
};

type State<'input> = LexerState<'input, LeanLanguage>;

/// Lean 词法分析器
pub struct LeanLexer<'config> {
    config: &'config LeanLanguage,
}

impl<'config> LeanLexer<'config> {
    /// 创建新的 Lean 词法分析器
    pub fn new(config: &'config LeanLanguage) -> Self {
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
            state.add_token(LeanSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(LeanSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(LeanSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State, _source: &SourceText) -> bool {
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
                    state.advance(ch.len_utf8());
                }
                state.add_token(LeanSyntaxKind::Comment, start_pos, state.get_position());
                true
            }
            else {
                // 回退，这是减号操作符
                state.set_position(start_pos);
                false
            }
        }
        else if let Some('/') = state.peek() {
            state.advance(1);
            if let Some('-') = state.peek() {
                // 块注释开                state.advance(1);
                let mut depth = 1;
                while depth > 0 && state.not_at_end() {
                    if let Some('/') = state.peek() {
                        state.advance(1);
                        if let Some('-') = state.peek() {
                            state.advance(1);
                            depth += 1;
                        }
                    }
                    else if let Some('-') = state.peek() {
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
                state.add_token(LeanSyntaxKind::Comment, start_pos, state.get_position());
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
    fn lex_string(&self, state: &mut State, _source: &SourceText) -> bool {
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
            state.add_token(LeanSyntaxKind::String, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符字面
    fn lex_char(&self, state: &mut State, _source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
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
            state.add_token(LeanSyntaxKind::Char, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理数字字面
    fn lex_number(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                // 扫描数字
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 检查小数点
                if let Some('.') = state.peek() {
                    let next_pos = state.get_position() + 1;
                    if next_pos < source.len() {
                        let next_char = source.get_char_at(next_pos);
                        if next_char.map_or(false, |c| c.is_ascii_digit()) {
                            state.advance(1); // 跳过小数点
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

                // 检查指数部分
                if let Some(ch) = state.peek() {
                    if ch == 'e' || ch == 'E' {
                        state.advance(1);
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
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

                state.add_token(LeanSyntaxKind::Number, start_pos, state.get_position());
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
                    if ch.is_alphanumeric() || ch == '_' || ch == '\'' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = source.get_text_in((start_pos..state.get_position()).into()).unwrap_or("");
                let kind = match text {
                    "axiom" => LeanSyntaxKind::Axiom,
                    "constant" => LeanSyntaxKind::Constant,
                    "def" => LeanSyntaxKind::Def,
                    "example" => LeanSyntaxKind::Example,
                    "inductive" => LeanSyntaxKind::Inductive,
                    "lemma" => LeanSyntaxKind::Lemma,
                    "namespace" => LeanSyntaxKind::Namespace,
                    "open" => LeanSyntaxKind::Open,
                    "private" => LeanSyntaxKind::Private,
                    "protected" => LeanSyntaxKind::Protected,
                    "section" => LeanSyntaxKind::Section,
                    "structure" => LeanSyntaxKind::Structure,
                    "theorem" => LeanSyntaxKind::Theorem,
                    "universe" => LeanSyntaxKind::Universe,
                    "variable" => LeanSyntaxKind::Variable,
                    "variables" => LeanSyntaxKind::Variables,
                    "end" => LeanSyntaxKind::End,
                    "import" => LeanSyntaxKind::Import,
                    "export" => LeanSyntaxKind::Export,
                    "prelude" => LeanSyntaxKind::Prelude,
                    "noncomputable" => LeanSyntaxKind::Noncomputable,
                    "partial" => LeanSyntaxKind::Partial,
                    "unsafe" => LeanSyntaxKind::Unsafe,
                    "mutual" => LeanSyntaxKind::Mutual,
                    "where" => LeanSyntaxKind::Where,
                    "have" => LeanSyntaxKind::Have,
                    "show" => LeanSyntaxKind::Show,
                    "suffices" => LeanSyntaxKind::Suffices,
                    "let" => LeanSyntaxKind::Let,
                    "in" => LeanSyntaxKind::In,
                    "if" => LeanSyntaxKind::If,
                    "then" => LeanSyntaxKind::Then,
                    "else" => LeanSyntaxKind::Else,
                    "match" => LeanSyntaxKind::Match,
                    "with" => LeanSyntaxKind::With,
                    "fun" => LeanSyntaxKind::Fun,
                    "do" => LeanSyntaxKind::Do,
                    "for" => LeanSyntaxKind::For,
                    "while" => LeanSyntaxKind::While,
                    "break" => LeanSyntaxKind::Break,
                    "continue" => LeanSyntaxKind::Continue,
                    "return" => LeanSyntaxKind::Return,
                    "try" => LeanSyntaxKind::Try,
                    "catch" => LeanSyntaxKind::Catch,
                    "finally" => LeanSyntaxKind::Finally,
                    "throw" => LeanSyntaxKind::Throw,
                    _ => LeanSyntaxKind::Identifier,
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

    /// 处理操作符和分隔
    fn lex_operator_or_delimiter(&self, state: &mut State, _source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '(' => LeanSyntaxKind::LeftParen,
                ')' => LeanSyntaxKind::RightParen,
                '{' => LeanSyntaxKind::LeftBrace,
                '}' => LeanSyntaxKind::RightBrace,
                '[' => LeanSyntaxKind::LeftBracket,
                ']' => LeanSyntaxKind::RightBracket,
                ',' => LeanSyntaxKind::Comma,
                ';' => LeanSyntaxKind::Semicolon,
                '+' => LeanSyntaxKind::Plus,
                '*' => LeanSyntaxKind::Star,
                '/' => LeanSyntaxKind::Slash,
                '%' => LeanSyntaxKind::Percent,
                '^' => LeanSyntaxKind::Caret,
                '#' => LeanSyntaxKind::Hash,
                '&' => LeanSyntaxKind::Ampersand,
                '|' => LeanSyntaxKind::Pipe,
                '~' => LeanSyntaxKind::Tilde,
                '!' => LeanSyntaxKind::Bang,
                '?' => LeanSyntaxKind::Question,
                '@' => LeanSyntaxKind::At,
                '$' => LeanSyntaxKind::Dollar,
                '<' => LeanSyntaxKind::Lt,
                '>' => LeanSyntaxKind::Gt,
                '=' => LeanSyntaxKind::Eq,
                _ => {
                    // 检查多字符操作符
                    match ch {
                        ':' => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                state.add_token(LeanSyntaxKind::ColonEq, start_pos, state.get_position());
                                return true;
                            }
                            else if let Some(':') = state.peek() {
                                state.advance(1);
                                state.add_token(LeanSyntaxKind::ColonColon, start_pos, state.get_position());
                                return true;
                            }
                            else {
                                state.add_token(LeanSyntaxKind::Colon, start_pos, state.get_position());
                                return true;
                            }
                        }
                        '.' => {
                            state.advance(1);
                            if let Some('.') = state.peek() {
                                state.advance(1);
                                state.add_token(LeanSyntaxKind::DotDot, start_pos, state.get_position());
                                return true;
                            }
                            else {
                                state.add_token(LeanSyntaxKind::Dot, start_pos, state.get_position());
                                return true;
                            }
                        }
                        '-' => {
                            state.advance(1);
                            if let Some('>') = state.peek() {
                                state.advance(1);
                                state.add_token(LeanSyntaxKind::Arrow, start_pos, state.get_position());
                                return true;
                            }
                            else {
                                state.add_token(LeanSyntaxKind::Minus, start_pos, state.get_position());
                                return true;
                            }
                        }
                        _ => return false,
                    }
                }
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

impl<'config> Lexer<LeanLanguage> for LeanLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<LeanSyntaxKind> {
        let mut state = State::new(source);

        while state.not_at_end() {
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_comment(&mut state, source) {
                continue;
            }

            if self.lex_string(&mut state, source) {
                continue;
            }

            if self.lex_char(&mut state, source) {
                continue;
            }

            if self.lex_number(&mut state, source) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_operator_or_delimiter(&mut state, source) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(LeanSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(LeanSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
