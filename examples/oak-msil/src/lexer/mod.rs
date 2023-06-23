#![doc = include_str!("readme.md")]
pub mod token_type;
pub use token_type::MsilTokenType;

use crate::language::MsilLanguage;
use oak_core::{Lexer, LexerCache, LexerState, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, MsilLanguage>;

#[derive(Clone, Debug)]
pub struct MsilLexer<'config> {
    _config: &'config MsilLanguage,
}

impl<'config> MsilLexer<'config> {
    pub fn new(config: &'config MsilLanguage) -> Self {
        Self { _config: config }
    }
}

impl MsilLexer<'_> {
    pub fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        let safe_point = state.get_position();
        while state.not_at_end() {
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

            if self.lex_identifier(state) {
                continue;
            }

            if self.lex_delimiter(state) {
                continue;
            }

            // 如果没有匹配任何规则，跳过当前字符
            if let Some(ch) = state.peek() {
                let start_pos = state.get_position();
                state.advance(ch.len_utf8());
                state.add_token(MsilTokenType::Error, start_pos, state.get_position())
            }

            state.advance_if_dead_lock(safe_point)
        }

        state.add_eof();
        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' { state.advance(ch.len_utf8()) } else { break }
        }

        if state.get_position() > start_pos {
            state.add_token(MsilTokenType::Whitespace, start_pos, state.get_position());
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
            state.add_token(MsilTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1)
            }
            state.add_token(MsilTokenType::Whitespace, start_pos, state.get_position());
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
            if let Some('/') = state.peek_next_n(1) {
                // 行注释
                state.advance(2);
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8())
                }
                state.add_token(MsilTokenType::CommentToken, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理标识符和关键字
    fn lex_identifier<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if !ch.is_ascii_alphabetic() && ch != '_' && ch != '.' {
                return false;
            }

            // 收集标识符字符
            while let Some(ch) = state.peek() {
                if ch.is_ascii_alphanumeric() || ch == '_' || ch == '.' { state.advance(ch.len_utf8()) } else { break }
            }

            // 检查是否是关键字
            let text = state.get_text_in((start_pos..state.get_position()).into());
            let token_kind = match text {
                std::borrow::Cow::Borrowed(".assembly") => MsilTokenType::AssemblyKeyword,
                std::borrow::Cow::Borrowed("extern") => MsilTokenType::ExternKeyword,
                std::borrow::Cow::Borrowed(".module") => MsilTokenType::ModuleKeyword,
                std::borrow::Cow::Borrowed(".class") => MsilTokenType::ClassKeyword,
                std::borrow::Cow::Borrowed(".method") => MsilTokenType::MethodKeyword,
                std::borrow::Cow::Borrowed("public") => MsilTokenType::PublicKeyword,
                std::borrow::Cow::Borrowed("private") => MsilTokenType::PrivateKeyword,
                std::borrow::Cow::Borrowed("static") => MsilTokenType::StaticKeyword,
                std::borrow::Cow::Borrowed("void") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("bool") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("int8") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("int16") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("int32") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("int64") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("float32") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("float64") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("string") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("object") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("char") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("unsigned") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("extends") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("implements") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("auto") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("ansi") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("beforefieldinit") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("sealed") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("abstract") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("serializable") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("sequential") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("explicit") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("unicode") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("autochar") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("family") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("assembly") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("famandassem") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("famorassem") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("privatescope") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("hidebysig") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("specialname") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("rtspecialname") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("cil") => MsilTokenType::Keyword,
                std::borrow::Cow::Borrowed("managed") => MsilTokenType::Keyword,
                _ => MsilTokenType::IdentifierToken,
            };

            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理数字
    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if !ch.is_ascii_digit() {
                return false;
            }

            // 处理整数部分
            while let Some(ch) = state.peek() {
                if ch.is_ascii_digit() { state.advance(ch.len_utf8()) } else { break }
            }

            // 处理小数点
            if let Some('.') = state.peek() {
                if let Some(next_ch) = state.peek_next_n(1) {
                    if next_ch.is_ascii_digit() {
                        state.advance(1); // 跳过小数点
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() { state.advance(ch.len_utf8()) } else { break }
                        }
                    }
                }
            }

            state.add_token(MsilTokenType::NumberToken, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符串
    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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
                else {
                    state.advance(ch.len_utf8())
                }
            }

            state.add_token(MsilTokenType::StringToken, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理分隔符
    fn lex_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '{' => MsilTokenType::LeftBrace,
                '}' => MsilTokenType::RightBrace,
                '(' => MsilTokenType::LeftParen,
                ')' => MsilTokenType::RightParen,
                '[' => MsilTokenType::LeftBracket,
                ']' => MsilTokenType::RightBracket,
                '.' => MsilTokenType::Dot,
                ':' => MsilTokenType::Colon,
                ';' => MsilTokenType::Semicolon,
                ',' => MsilTokenType::Comma,
                '=' => MsilTokenType::Equal,
                '/' => MsilTokenType::Slash,
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

impl Lexer<MsilLanguage> for MsilLexer<'_> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<MsilLanguage>) -> LexOutput<MsilLanguage> {
        let mut state = State::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        state.finish_with_cache(result, cache)
    }
}

impl MsilLexer<'_> {
    pub fn tokenize<'a>(&self, text: &'a str) -> Vec<oak_core::Token<<MsilLanguage as oak_core::Language>::TokenType>> {
        let source = oak_core::SourceText::new(text);
        let mut cache = oak_core::parser::session::ParseSession::<MsilLanguage>::default();
        let mut state = State::new_with_cache(&source, 0, &mut cache);
        let result = self.run(&mut state);
        state.finish_with_cache(result, &mut cache).result.unwrap().to_vec()
    }
}
