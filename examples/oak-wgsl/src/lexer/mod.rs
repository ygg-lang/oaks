#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::WgslLanguage, lexer::token_type::WgslTokenType};
use oak_core::{Lexer, LexerCache, LexerState, OakError, TextEdit, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, WgslLanguage>;

#[derive(Clone)]
pub struct WgslLexer<'config> {
    config: &'config WgslLanguage,
}

impl<'config> WgslLexer<'config> {
    pub fn new(config: &'config WgslLanguage) -> Self {
        Self { config }
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
            state.add_token(WgslTokenType::Whitespace, start_pos, state.get_position());
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
            state.add_token(WgslTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(WgslTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        // 单行注释 //
        if let Some('/') = state.peek() {
            if state.peek_next_n(1) == Some('/') {
                state.advance(2);

                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(WgslTokenType::Comment, start_pos, state.get_position());
                return true;
            }
        }

        // 多行注释 /* */
        if let Some('/') = state.peek() {
            if state.peek_next_n(1) == Some('*') {
                state.advance(2);
                let mut depth = 1;

                while depth > 0 && state.not_at_end() {
                    if let Some('/') = state.peek() {
                        if state.peek_next_n(1) == Some('*') {
                            state.advance(2);
                            depth += 1;
                            continue;
                        }
                    }

                    if let Some('*') = state.peek() {
                        state.advance(1);
                        if state.peek_next_n(1) == Some('/') {
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

                state.add_token(WgslTokenType::Comment, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理字符串字面量
    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1); // 跳过开始的引号

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1); // 跳过结束的引号
                    state.add_token(WgslTokenType::StringLiteral, start_pos, state.get_position());
                    return true;
                }
                else if ch == '\\' {
                    state.advance(1); // 跳过转义字符
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

            state.add_token(WgslTokenType::Error, start_pos, state.get_position());
            return true;
        }

        false
    }

    /// 处理数字字面量
    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                // 十六进制
                if ch == '0' && state.peek_next_n(1) == Some('x') {
                    state.advance(2); // 跳过 "0x"

                    let mut has_digits = false;
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_hexdigit() {
                            state.advance(1);
                            has_digits = true;
                        }
                        else {
                            break;
                        }
                    }

                    if has_digits {
                        // 检查后缀
                        if let Some('u') = state.peek() {
                            state.advance(1);
                        }
                        else if let Some('i') = state.peek() {
                            state.advance(1);
                        }

                        state.add_token(WgslTokenType::IntegerLiteral, start_pos, state.get_position());
                        return true;
                    }
                }

                // 十进制数
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                let mut is_float = false;

                // 小数
                if let Some('.') = state.peek() {
                    if let Some(next_ch) = state.peek_next_n(1) {
                        if next_ch.is_ascii_digit() {
                            state.advance(1);
                            is_float = true;
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

                // 科学计数
                if let Some('e') | Some('E') = state.peek() {
                    let exp_start = state.get_position();
                    state.advance(1);
                    is_float = true;

                    if let Some('+') | Some('-') = state.peek() {
                        state.advance(1);
                    }

                    let mut has_exp_digits = false;
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                            has_exp_digits = true;
                        }
                        else {
                            break;
                        }
                    }

                    if !has_exp_digits {
                        state.set_position(exp_start);
                        is_float = false;
                    }
                }

                // 后缀
                if is_float {
                    if let Some('f') = state.peek() {
                        state.advance(1);
                    }
                    state.add_token(WgslTokenType::FloatLiteral, start_pos, state.get_position());
                }
                else {
                    if let Some('u') = state.peek() {
                        state.advance(1);
                    }
                    else if let Some('i') = state.peek() {
                        state.advance(1);
                    }
                    state.add_token(WgslTokenType::IntegerLiteral, start_pos, state.get_position());
                }

                return true;
            }
        }

        false
    }

    /// 处理标识符或关键字
    fn lex_ident_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let end_pos = state.get_position();
                let text = state.get_text_in(oak_core::Range { start: start_pos, end: end_pos });
                let kind = match text.as_ref() {
                    "i32" => WgslTokenType::I32Kw,
                    "u32" => WgslTokenType::U32Kw,
                    "f32" => WgslTokenType::F32Kw,
                    "f16" => WgslTokenType::F16Kw,
                    "bool" => WgslTokenType::BoolKw,
                    "vec2" => WgslTokenType::Vec2Kw,
                    "vec3" => WgslTokenType::Vec3Kw,
                    "vec4" => WgslTokenType::Vec4Kw,
                    "mat2x2" => WgslTokenType::Mat2x2Kw,
                    "mat3x3" => WgslTokenType::Mat3x3Kw,
                    "mat4x4" => WgslTokenType::Mat4x4Kw,
                    "array" => WgslTokenType::ArrayKw,
                    "ptr" => WgslTokenType::PtrKw,
                    "atomic" => WgslTokenType::AtomicKw,
                    "texture_1d" => WgslTokenType::Texture1dKw,
                    "texture_2d" => WgslTokenType::Texture2dKw,
                    "texture_3d" => WgslTokenType::Texture3dKw,
                    "texture_cube" => WgslTokenType::TextureCubeKw,
                    "sampler" => WgslTokenType::SamplerKw,
                    "if" => WgslTokenType::IfKw,
                    "else" => WgslTokenType::ElseKw,
                    "switch" => WgslTokenType::SwitchKw,
                    "case" => WgslTokenType::CaseKw,
                    "default" => WgslTokenType::DefaultKw,
                    "loop" => WgslTokenType::LoopKw,
                    "for" => WgslTokenType::ForKw,
                    "while" => WgslTokenType::WhileKw,
                    "break" => WgslTokenType::BreakKw,
                    "continue" => WgslTokenType::ContinueKw,
                    "return" => WgslTokenType::ReturnKw,
                    "discard" => WgslTokenType::DiscardKw,
                    "fn" => WgslTokenType::FnKw,
                    "var" => WgslTokenType::VarKw,
                    "let" => WgslTokenType::LetKw,
                    "const" => WgslTokenType::ConstKw,
                    "override" => WgslTokenType::OverrideKw,
                    "struct" => WgslTokenType::StructKw,
                    "alias" => WgslTokenType::AliasKw,
                    "uniform" => WgslTokenType::UniformKw,
                    "storage" => WgslTokenType::StorageKw,
                    "workgroup" => WgslTokenType::WorkgroupKw,
                    "private" => WgslTokenType::PrivateKw,
                    "function" => WgslTokenType::FunctionKw,
                    "read" => WgslTokenType::ReadKw,
                    "write" => WgslTokenType::WriteKw,
                    "read_write" => WgslTokenType::ReadWriteKw,
                    "vertex" => WgslTokenType::VertexKw,
                    "fragment" => WgslTokenType::FragmentKw,
                    "compute" => WgslTokenType::ComputeKw,
                    "true" => WgslTokenType::BoolLiteral,
                    "false" => WgslTokenType::BoolLiteral,
                    _ => WgslTokenType::Identifier,
                };

                state.add_token(kind, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理标点符号和操作符
    fn lex_punctuation<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '<' if state.peek_next_n(1) == Some('<') && state.peek_next_n(2) == Some('=') => {
                    state.advance(3);
                    WgslTokenType::LeftShiftAssign
                }
                '>' if state.peek_next_n(1) == Some('>') && state.peek_next_n(2) == Some('=') => {
                    state.advance(3);
                    WgslTokenType::RightShiftAssign
                }
                '+' if state.peek_next_n(1) == Some('=') => {
                    state.advance(2);
                    WgslTokenType::PlusAssign
                }
                '-' if state.peek_next_n(1) == Some('=') => {
                    state.advance(2);
                    WgslTokenType::MinusAssign
                }
                '*' if state.peek_next_n(1) == Some('=') => {
                    state.advance(2);
                    WgslTokenType::StarAssign
                }
                '/' if state.peek_next_n(1) == Some('=') => {
                    state.advance(2);
                    WgslTokenType::SlashAssign
                }
                '%' if state.peek_next_n(1) == Some('=') => {
                    state.advance(2);
                    WgslTokenType::PercentAssign
                }
                '&' if state.peek_next_n(1) == Some('=') => {
                    state.advance(2);
                    WgslTokenType::AmpersandAssign
                }
                '|' if state.peek_next_n(1) == Some('=') => {
                    state.advance(2);
                    WgslTokenType::PipeAssign
                }
                '^' if state.peek_next_n(1) == Some('=') => {
                    state.advance(2);
                    WgslTokenType::CaretAssign
                }
                '=' if state.peek_next_n(1) == Some('=') => {
                    state.advance(2);
                    WgslTokenType::EqEq
                }
                '!' if state.peek_next_n(1) == Some('=') => {
                    state.advance(2);
                    WgslTokenType::BangEq
                }
                '<' if state.peek_next_n(1) == Some('=') => {
                    state.advance(2);
                    WgslTokenType::Le
                }
                '>' if state.peek_next_n(1) == Some('=') => {
                    state.advance(2);
                    WgslTokenType::Ge
                }
                '<' if state.peek_next_n(1) == Some('<') => {
                    state.advance(2);
                    WgslTokenType::LeftShift
                }
                '>' if state.peek_next_n(1) == Some('>') => {
                    state.advance(2);
                    WgslTokenType::RightShift
                }
                '&' if state.peek_next_n(1) == Some('&') => {
                    state.advance(2);
                    WgslTokenType::AmpersandAmpersand
                }
                '|' if state.peek_next_n(1) == Some('|') => {
                    state.advance(2);
                    WgslTokenType::PipePipe
                }
                '-' if state.peek_next_n(1) == Some('>') => {
                    state.advance(2);
                    WgslTokenType::Arrow
                }
                '+' => {
                    state.advance(1);
                    WgslTokenType::Plus
                }
                '-' => {
                    state.advance(1);
                    WgslTokenType::Minus
                }
                '*' => {
                    state.advance(1);
                    WgslTokenType::Star
                }
                '/' => {
                    state.advance(1);
                    WgslTokenType::Slash
                }
                '%' => {
                    state.advance(1);
                    WgslTokenType::Percent
                }
                '=' => {
                    state.advance(1);
                    WgslTokenType::Assign
                }
                '<' => {
                    state.advance(1);
                    WgslTokenType::Lt
                }
                '>' => {
                    state.advance(1);
                    WgslTokenType::Gt
                }
                '!' => {
                    state.advance(1);
                    WgslTokenType::Bang
                }
                '&' => {
                    state.advance(1);
                    WgslTokenType::Ampersand
                }
                '|' => {
                    state.advance(1);
                    WgslTokenType::Pipe
                }
                '^' => {
                    state.advance(1);
                    WgslTokenType::Caret
                }
                '~' => {
                    state.advance(1);
                    WgslTokenType::Tilde
                }
                '(' => {
                    state.advance(1);
                    WgslTokenType::LeftParen
                }
                ')' => {
                    state.advance(1);
                    WgslTokenType::RightParen
                }
                '{' => {
                    state.advance(1);
                    WgslTokenType::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    WgslTokenType::RightBrace
                }
                '[' => {
                    state.advance(1);
                    WgslTokenType::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    WgslTokenType::RightBracket
                }
                ',' => {
                    state.advance(1);
                    WgslTokenType::Comma
                }
                ';' => {
                    state.advance(1);
                    WgslTokenType::Semicolon
                }
                ':' => {
                    state.advance(1);
                    WgslTokenType::Colon
                }
                '.' => {
                    state.advance(1);
                    WgslTokenType::Dot
                }
                '@' => {
                    state.advance(1);
                    WgslTokenType::At
                }
                _ => return false,
            };

            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理普通文本
    fn lex_text<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            state.advance(ch.len_utf8());
            state.add_token(WgslTokenType::Text, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
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

            if self.lex_ident_or_keyword(state) {
                continue;
            }

            if self.lex_punctuation(state) {
                continue;
            }

            if self.lex_text(state) {
                continue;
            }

            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(WgslTokenType::Error, start_pos, state.get_position());
            }
        }

        Ok(())
    }
}

impl<'config> Lexer<WgslLanguage> for WgslLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<WgslLanguage>) -> LexOutput<WgslLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}
